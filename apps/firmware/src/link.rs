use crate::audio::{AudioFrame, BYTES_PER_FRAME, DEFAULT_VAD_THRESHOLD, FRAME_DURATION_MS};
use crate::avatar::{AvatarEmotion, AvatarRenderer, AvatarState};
use crate::ble::{BleProvisioner, TransportKind};
use crate::haptic::HapticPulse;
use crate::power::PowerManager;
use crate::protocol::{BohsFrame, ProtocolDispatcher, FLAG_END_OF_UTTERANCE, FLAG_VOICE_ACTIVITY};
use crate::{Packet, MAX_PACKET_BYTES};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::signal::Signal;

/// The lock every cross-task channel and signal uses.
pub type Mutex = CriticalSectionRawMutex;

/// Downstream frames waiting for the speaker; the I2S playback driver is not in v0.4.0.
pub static AUDIO_RX_CHANNEL: Channel<Mutex, AudioFrame, 4> = Channel::new();
/// Frames waiting for the active transport writer.
pub static OUTBOUND_PKT_CHANNEL: Channel<Mutex, Packet, 8> = Channel::new();
/// Latest avatar face; a newer value replaces one the display has not drawn yet.
pub static AVATAR_SIGNAL: Signal<Mutex, (AvatarState, AvatarEmotion)> = Signal::new();
/// Latest haptic request for the motor driver.
pub static HAPTIC_SIGNAL: Signal<Mutex, HapticPulse> = Signal::new();

/// Protocol state owned by the network task: one Reverse MCP server and one provisioner.
pub struct Node {
    pub dispatcher: ProtocolDispatcher,
    pub avatar: AvatarRenderer,
    pub prov: BleProvisioner,
    pub power: PowerManager,
    seq: u16,
    was_active: bool,
}

impl Node {
    pub fn new(hardware_id: [u8; 4]) -> Self {
        let mut power = PowerManager::new();
        power.on_wake_trigger();
        let prov = BleProvisioner::new(hardware_id);
        power.on_hardware_ready(prov.transport);
        let dispatcher = ProtocolDispatcher::new();
        Self { dispatcher, avatar: AvatarRenderer::new(), prov, power, seq: 0, was_active: false }
    }

    /// Routes one frame received over `link`: audio to playback, `0x02` to the Reverse MCP server,
    /// `0x03` to the provisioner. Responses go to `OUTBOUND_PKT_CHANNEL`. Over WSS a `0x03` frame
    /// is dropped unanswered, because the device never sends `0x03` there.
    pub fn handle_inbound(&mut self, link: TransportKind, packet: &[u8]) {
        let mut out = [0u8; MAX_PACKET_BYTES];
        let written = match self.dispatcher.decode_packet(packet) {
            Ok(BohsFrame::Audio { payload, .. }) => {
                let mut frame = AudioFrame::new();
                if frame.copy_from_bytes(payload).is_ok() {
                    let _ = AUDIO_RX_CHANNEL.try_send(frame);
                }
                None
            }
            Ok(BohsFrame::Control { json }) => self.dispatch_control(json, &mut out),
            Ok(BohsFrame::Provision { .. }) if link == TransportKind::WifiDirect => None,
            Ok(BohsFrame::Provision { json }) => self.prov.dispatch_provision_rpc(json, link, &mut out),
            Err(_) => None,
        };
        if let Some(len) = written {
            let _ = OUTBOUND_PKT_CHANNEL.try_send((out, len));
        }
    }

    fn dispatch_control(&mut self, json: &str, out: &mut [u8]) -> Option<usize> {
        let before = (self.avatar.state, self.avatar.emotion);
        let written = self.dispatcher.dispatch_json_rpc(json, &mut self.avatar, out);
        let after = (self.avatar.state, self.avatar.emotion);
        if after != before {
            AVATAR_SIGNAL.signal(after);
        }
        if let Some(pulse) = self.dispatcher.haptic.take() {
            HAPTIC_SIGNAL.signal(pulse);
        }
        written
    }

    /// Encodes one captured frame as a `0x01` frame with VAD and end-of-utterance flags.
    pub fn encode_audio(&mut self, frame: &AudioFrame, out: &mut [u8]) -> Option<usize> {
        let is_active = frame.is_voice_active(DEFAULT_VAD_THRESHOLD);
        let eou = if self.was_active && !is_active { FLAG_END_OF_UTTERANCE } else { 0 };
        self.was_active = is_active;
        self.power.on_audio_activity(is_active, FRAME_DURATION_MS);
        let flags = if is_active { FLAG_VOICE_ACTIVITY } else { 0 } | eou;
        let mut payload = [0u8; BYTES_PER_FRAME];
        frame.write_le_bytes(&mut payload).ok()?;
        let len = ProtocolDispatcher::encode_audio_frame(flags, self.seq, &payload, out).ok()?;
        self.seq = self.seq.wrapping_add(1);
        Some(len)
    }
}

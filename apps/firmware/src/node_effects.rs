use crate::audio::{AudioFrame, BYTES_PER_FRAME, DEFAULT_VAD_THRESHOLD, FRAME_DURATION_MS};
use crate::ble::{ProvisionConfig, ProvisionState, TransportKind};
use crate::events::encode_prov_state;
use crate::haptic::{HapticPattern, HapticPulse};
use crate::link::{
    device_info, outbound, set_device_info, DeviceInfo, Overlay, StoreRequest, CAPTURE_ENABLED, CONFIG_WATCH,
    HAPTIC_SIGNAL, OVERLAY_SIGNAL, STORE_CHANNEL,
};
use crate::node::Node;
use crate::power::PowerState;
use crate::protocol::{ProtocolDispatcher, ProtocolError, FLAG_END_OF_UTTERANCE, FLAG_VOICE_ACTIVITY};
use crate::MAX_PACKET_BYTES;
use core::sync::atomic::Ordering;

/// The double pulse `prov.identify` plays so the user finds the charm.
const IDENTIFY_PULSE: HapticPulse = HapticPulse { pattern: HapticPattern::DoubleTap, duration_ms: 60, intensity: 80 };

impl Node {
    /// Encodes a notification and queues it on the gateway link; it is dropped when no session is open.
    pub fn send_event(&mut self, encode: impl FnOnce(&mut [u8]) -> Result<usize, ProtocolError>) {
        let Some(link) = self.gateway_link else {
            return;
        };
        let mut out = [0u8; MAX_PACKET_BYTES];
        if let Ok(len) = encode(&mut out) {
            let _ = outbound(link).try_send((out, len));
        }
    }

    pub(crate) fn after_event(&mut self, before: ProvisionState, old_config: Option<ProvisionConfig>) {
        let state = self.prov.state;
        let transport = self.gateway_link.unwrap_or(self.prov.transport);
        set_device_info(DeviceInfo { state, transport, ..device_info() });
        self.publish_capture();
        if state == before {
            return;
        }
        let window_open = matches!(state, ProvisionState::AwaitingTap | ProvisionState::Authorized);
        if state == ProvisionState::AwaitingTap {
            HAPTIC_SIGNAL.signal(IDENTIFY_PULSE);
        }
        OVERLAY_SIGNAL.signal(if window_open { Overlay::PairingRing } else { Overlay::None });
        self.notify_prov_state();
        let configured = before == ProvisionState::Authorized && state == ProvisionState::Provisioned;
        if let Some(cfg) = self.prov.config.clone().filter(|cfg| configured && Some(cfg) != old_config.as_ref()) {
            let _ = STORE_CHANNEL.try_send(StoreRequest::Config(cfg.clone()));
            CONFIG_WATCH.sender().send(cfg);
        }
    }

    fn notify_prov_state(&mut self) {
        if self.prov.conn.is_none() {
            return;
        }
        let mut out = [0u8; MAX_PACKET_BYTES];
        if let Ok(len) = encode_prov_state(self.prov.state, &mut out) {
            let _ = outbound(TransportKind::BleRelay).try_send((out, len));
        }
    }

    /// Encodes a captured microphone frame and queues it on the gateway link while the device
    /// streams; in sleep grace the frame only feeds the VAD, so speech resumes the stream. Frames
    /// that are not sent give their sequence number back, so the upstream sequence has no gaps.
    pub fn send_audio(&mut self, frame: &AudioFrame) {
        let before = self.power.state;
        let mut out = [0u8; MAX_PACKET_BYTES];
        let encoded = self.encode_audio(frame, &mut out);
        let streaming = before == PowerState::StreamFullDuplex || self.power.state == PowerState::StreamFullDuplex;
        match (self.gateway_link, encoded, streaming) {
            (Some(link), Some(len), true) => {
                let _ = outbound(link).try_send((out, len));
            }
            (_, Some(_), _) => self.seq = self.seq.wrapping_sub(1),
            _ => {}
        }
        self.publish_capture();
    }

    /// Tells the microphone task whether to run.
    pub fn publish_capture(&self) {
        let enabled = self.gateway_link.is_some() && self.power.state != PowerState::DeepSleep;
        CAPTURE_ENABLED.store(enabled, Ordering::Relaxed);
    }

    /// Encodes one captured frame as a `0x01` frame with VAD and end-of-utterance flags.
    pub fn encode_audio(&mut self, frame: &AudioFrame, out: &mut [u8]) -> Option<usize> {
        let is_active = self.vad.update(frame, DEFAULT_VAD_THRESHOLD);
        let eou = if self.was_active && !is_active { FLAG_END_OF_UTTERANCE } else { 0 };
        self.was_active = is_active;
        let playing = self.playback_hold > 0;
        self.playback_hold = self.playback_hold.saturating_sub(1);
        self.power.on_audio_activity(is_active || playing, FRAME_DURATION_MS);
        let flags = if is_active { FLAG_VOICE_ACTIVITY } else { 0 } | eou;
        let mut payload = [0u8; BYTES_PER_FRAME];
        frame.write_le_bytes(&mut payload).ok()?;
        let len = ProtocolDispatcher::encode_audio_frame(flags, self.seq, &payload, out).ok()?;
        self.seq = self.seq.wrapping_add(1);
        Some(len)
    }
}

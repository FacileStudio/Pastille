#![cfg_attr(target_arch = "xtensa", no_std)]
#![cfg_attr(target_arch = "xtensa", no_main)]

mod audio;
mod avatar;
mod battery;
mod ble;
mod pins;
mod power;
mod prov;
mod protocol;
/// Length-prefix framing for the BLE and USB byte streams; their drivers are not in this release.
#[cfg_attr(not(test), allow(dead_code))]
mod stream;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_ble;
mod tools;
mod trig;

use audio::{AudioFrame, I2sPipeline, DEFAULT_VAD_THRESHOLD, FRAME_DURATION_MS};
use avatar::{AvatarEmotion, AvatarRenderer, AvatarState};
use ble::{BleProvisioner, TransportKind};
use pins::BOARD_PINS;
use power::PowerManager;
use protocol::{BohsFrame, ProtocolDispatcher, ProtocolError, FLAG_END_OF_UTTERANCE, FLAG_VOICE_ACTIVITY};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};

pub static AUDIO_TX_CHANNEL: Channel<CriticalSectionRawMutex, AudioFrame, 4> = Channel::new();
pub static AUDIO_RX_CHANNEL: Channel<CriticalSectionRawMutex, AudioFrame, 4> = Channel::new();
/// Largest bohs.v1 frame on any transport: a 644-byte audio frame or a JSON-RPC message.
pub const MAX_PACKET_BYTES: usize = 1024;
/// A packet buffer paired with the number of bytes it actually holds.
pub type Packet = ([u8; MAX_PACKET_BYTES], usize);

pub static INBOUND_WS_CHANNEL: Channel<CriticalSectionRawMutex, Packet, 2> = Channel::new();
pub static OUTBOUND_PKT_CHANNEL: Channel<CriticalSectionRawMutex, Packet, 4> = Channel::new();
pub static AVATAR_STATE_CHANNEL: Channel<CriticalSectionRawMutex, (AvatarState, AvatarEmotion), 2> =
    Channel::new();

#[embassy_executor::task]
pub async fn display_task() {
    let mut renderer = AvatarRenderer::new();
    loop {
        if let Ok((state, emotion)) = AVATAR_STATE_CHANNEL.try_receive() {
            renderer.state = state;
            renderer.emotion = emotion;
        }
        renderer.step();
        Timer::after(Duration::from_millis(33)).await;
    }
}

#[embassy_executor::task]
pub async fn audio_task() {
    let mut pipeline = I2sPipeline::new();
    pipeline.amp_enabled = true;
    let mut frame = AudioFrame::new();

    loop {
        if pipeline.capture_frame(&mut frame).is_ok() {
            let _ = AUDIO_TX_CHANNEL.try_send(frame);
        }
        if let Ok(playback_frame) = AUDIO_RX_CHANNEL.try_receive() {
            let _ = pipeline.play_frame(&playback_frame);
        }
        Timer::after(Duration::from_millis(FRAME_DURATION_MS as u64)).await;
    }
}

pub fn encode_next_audio_packet(
    frame: &AudioFrame,
    was_active: &mut bool,
    seq: &mut u16,
    power: &mut PowerManager,
    out: &mut [u8],
) -> Option<usize> {
    let is_active = frame.is_voice_active(DEFAULT_VAD_THRESHOLD);
    let eou = if *was_active && !is_active {
        FLAG_END_OF_UTTERANCE
    } else {
        0
    };
    *was_active = is_active;
    power.on_audio_activity(is_active, FRAME_DURATION_MS);

    let flags = (if is_active { FLAG_VOICE_ACTIVITY } else { 0 }) | eou;
    let len = ProtocolDispatcher::encode_audio_frame(flags, *seq, frame.as_bytes(), out).ok()?;
    *seq = seq.wrapping_add(1);
    Some(len)
}

/// Routes one frame received over `link`: audio to playback, `0x02` and `0x03` requests to
/// their handlers, with every response queued on `OUTBOUND_PKT_CHANNEL`.
pub fn handle_inbound_packet(
    dispatcher: &mut ProtocolDispatcher,
    avatar: &mut AvatarRenderer,
    prov: &mut BleProvisioner,
    link: TransportKind,
    packet: &[u8],
) {
    let mut out = [0u8; MAX_PACKET_BYTES];
    match dispatcher.decode_packet(packet) {
        Ok(BohsFrame::Audio { payload, .. }) => {
            let mut rx_frame = AudioFrame::new();
            if rx_frame.copy_from_bytes(payload).is_ok() {
                let _ = AUDIO_RX_CHANNEL.try_send(rx_frame);
            }
        }
        Ok(BohsFrame::Control { json }) => {
            let result = dispatcher.dispatch_json_rpc(json, avatar, &mut out);
            if queue_outbound(result, out) {
                let _ = AVATAR_STATE_CHANNEL.try_send((avatar.state, avatar.emotion));
            }
        }
        Ok(BohsFrame::Provision { json }) => {
            queue_outbound(prov.dispatch_provision_rpc(json, link, &mut out), out);
        }
        Err(_) => {}
    }
}

fn queue_outbound(result: Result<usize, ProtocolError>, pkt: [u8; MAX_PACKET_BYTES]) -> bool {
    let Ok(len) = result else {
        return false;
    };
    let _ = OUTBOUND_PKT_CHANNEL.try_send((pkt, len));
    true
}

#[embassy_executor::task]
pub async fn network_task() {
    let mut dispatcher = ProtocolDispatcher::new();
    let mut avatar = AvatarRenderer::new();
    let mut prov = BleProvisioner::new();
    let mut power = PowerManager::new();
    power.on_wake_trigger();
    power.on_hardware_ready(prov.transport);
    power.on_ble_connected();

    let mut seq: u16 = 0;
    let mut was_active = false;

    loop {
        if let Ok(captured) = AUDIO_TX_CHANNEL.try_receive() {
            let mut pkt = [0u8; MAX_PACKET_BYTES];
            let len = encode_next_audio_packet(&captured, &mut was_active, &mut seq, &mut power, &mut pkt);
            queue_outbound(len.ok_or(ProtocolError::BufferTooSmall), pkt);
        }
        if let Ok((inbound, len)) = INBOUND_WS_CHANNEL.try_receive() {
            let packet = &inbound[..len.min(MAX_PACKET_BYTES)];
            handle_inbound_packet(&mut dispatcher, &mut avatar, &mut prov, TransportKind::WifiDirect, packet);
        }
        prov.tick(20);
        Timer::after(Duration::from_millis(20)).await;
    }
}

#[cfg(target_arch = "xtensa")]
#[esp_hal::entry]
fn main() -> ! {
    use esp_hal::clock::ClockControl;
    use esp_hal::peripherals::Peripherals;

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let _clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let _wake_gpio = BOARD_PINS.general.touch_wake;

    loop {
        core::hint::spin_loop();
    }
}

#[cfg(not(target_arch = "xtensa"))]
fn main() {
    println!("Pastille BOHS-01 firmware initialized: {:?}", BOARD_PINS);
}

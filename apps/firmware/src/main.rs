#![cfg_attr(target_arch = "xtensa", no_std)]
#![cfg_attr(target_arch = "xtensa", no_main)]

mod audio;
mod avatar;
mod pins;
mod power;
mod protocol;
#[cfg(test)]
mod tests;
mod tools;
mod trig;

use audio::{AudioFrame, I2sPipeline, DEFAULT_VAD_THRESHOLD, FRAME_DURATION_MS};
use avatar::{AvatarEmotion, AvatarRenderer, AvatarState};
use pins::BOARD_PINS;
use power::PowerManager;
use protocol::{
    BohsFrame, ProtocolDispatcher, FLAG_END_OF_UTTERANCE, FLAG_VOICE_ACTIVITY,
};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};

pub static AUDIO_TX_CHANNEL: Channel<CriticalSectionRawMutex, AudioFrame, 4> = Channel::new();
pub static AUDIO_RX_CHANNEL: Channel<CriticalSectionRawMutex, AudioFrame, 4> = Channel::new();
pub static INBOUND_WS_CHANNEL: Channel<CriticalSectionRawMutex, [u8; 512], 2> = Channel::new();
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

pub fn handle_inbound_packet(
    dispatcher: &mut ProtocolDispatcher,
    avatar: &mut AvatarRenderer,
    packet: &[u8],
    response_buf: &mut [u8],
) {
    match dispatcher.decode_packet(packet) {
        Ok(BohsFrame::Audio { payload, .. }) => {
            let mut rx_frame = AudioFrame::new();
            if rx_frame.copy_from_bytes(payload).is_ok() {
                let _ = AUDIO_RX_CHANNEL.try_send(rx_frame);
            }
        }
        Ok(BohsFrame::Control { json }) => {
            if dispatcher.dispatch_json_rpc(json, avatar, response_buf).is_ok() {
                let _ = AVATAR_STATE_CHANNEL.try_send((avatar.state, avatar.emotion));
            }
        }
        Err(_) => {}
    }
}

#[embassy_executor::task]
pub async fn network_task() {
    let mut dispatcher = ProtocolDispatcher::new();
    let mut avatar = AvatarRenderer::new();
    let mut power = PowerManager::new();
    power.on_wake_trigger();
    power.on_hardware_ready();
    power.on_ws_connected();

    let mut packet_buf = [0u8; 1024];
    let mut seq: u16 = 0;
    let mut was_active = false;

    loop {
        if let Ok(captured) = AUDIO_TX_CHANNEL.try_receive() {
            let _ = encode_next_audio_packet(
                &captured,
                &mut was_active,
                &mut seq,
                &mut power,
                &mut packet_buf,
            );
        }
        if let Ok(inbound) = INBOUND_WS_CHANNEL.try_receive() {
            handle_inbound_packet(&mut dispatcher, &mut avatar, &inbound, &mut packet_buf);
        }
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

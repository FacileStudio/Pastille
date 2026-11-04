#![cfg_attr(target_arch = "xtensa", no_std)]
#![cfg_attr(target_arch = "xtensa", no_main)]

mod audio;
mod avatar;
mod pins;
mod protocol;

use audio::{AudioFrame, I2sPipeline};
use avatar::{AvatarEmotion, AvatarRenderer, AvatarState};
use pins::BOARD_PINS;
use protocol::{BohsFrame, ProtocolDispatcher, FLAG_VOICE_ACTIVITY};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};

pub static AUDIO_TX_CHANNEL: Channel<CriticalSectionRawMutex, AudioFrame, 4> = Channel::new();
pub static AUDIO_RX_CHANNEL: Channel<CriticalSectionRawMutex, AudioFrame, 4> = Channel::new();
pub static AVATAR_STATE_CHANNEL: Channel<CriticalSectionRawMutex, (AvatarState, AvatarEmotion), 2> =
    Channel::new();

#[embassy_executor::task]
pub async fn display_task() {
    let mut renderer = AvatarRenderer::new();
    loop {
        if let Ok((state, emotion)) = AVATAR_STATE_CHANNEL.try_receive() {
            renderer.set_state(state);
            renderer.set_emotion(emotion);
        }
        renderer.step();
        Timer::after(Duration::from_millis(33)).await;
    }
}

#[embassy_executor::task]
pub async fn audio_task() {
    let mut pipeline = I2sPipeline::new();
    pipeline.set_amp_enabled(true);
    let mut frame = AudioFrame::new();

    loop {
        if pipeline.capture_frame(&mut frame).is_ok() {
            let _ = AUDIO_TX_CHANNEL.try_send(frame);
        }

        if let Ok(playback_frame) = AUDIO_RX_CHANNEL.try_receive() {
            let _ = pipeline.play_frame(&playback_frame);
        }

        Timer::after(Duration::from_millis(20)).await;
    }
}

#[embassy_executor::task]
pub async fn network_task() {
    let mut dispatcher = ProtocolDispatcher::new();
    let mut avatar_mock = AvatarRenderer::new();
    let mut packet_buffer = [0u8; 1024];
    let mut seq: u16 = 0;

    loop {
        if let Ok(captured_audio) = AUDIO_TX_CHANNEL.try_receive() {
            let is_active = captured_audio.is_voice_active(audio::DEFAULT_VAD_THRESHOLD);
            let flags = if is_active { FLAG_VOICE_ACTIVITY } else { 0 };
            let _ = ProtocolDispatcher::encode_audio_frame(
                flags,
                seq,
                captured_audio.as_bytes(),
                &mut packet_buffer,
            );
            seq = seq.wrapping_add(1);
        }

        let demo_rpc = r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"charm.display_avatar","arguments":{"state":"listening","emotion":"focused"}}}"#;
        if let Ok(BohsFrame::Control { json }) = dispatcher.decode_packet(demo_rpc.as_bytes()) {
            let mut response_buf = [0u8; 1024];
            if dispatcher
                .dispatch_json_rpc(json, &mut avatar_mock, &mut response_buf)
                .is_ok()
            {
                let _ = AVATAR_STATE_CHANNEL.try_send((
                    avatar_mock.state(),
                    avatar_mock.emotion(),
                ));
            }
        }

        Timer::after(Duration::from_millis(100)).await;
    }
}

#[cfg(target_arch = "xtensa")]
#[esp_hal::entry]
fn main() -> ! {
    use esp_hal::clock::ClockControl;
    use esp_hal::gpio::Io;
    use esp_hal::peripherals::Peripherals;
    use esp_hal::rtc_cntl::Rtc;

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut rtc = Rtc::new(peripherals.LPWR);
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let wake_pin = io.pins.gpio3;
    rtc.sleep_deep(&[&wake_pin]);

    let executor = embassy_executor::Executor::new();
    let executor = make_static!(executor);
    executor.run(|spawner| {
        spawner.spawn(display_task()).ok();
        spawner.spawn(audio_task()).ok();
        spawner.spawn(network_task()).ok();
    });
}

#[cfg(not(target_arch = "xtensa"))]
fn main() {
    println!("Pastille Embassy firmware initialized.");
    println!("PINS: Board={:?}", BOARD_PINS);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_wiring() {
        assert_eq!(BOARD_PINS.general.r_boot_pin1, 0);
        assert_eq!(BOARD_PINS.general.touch_wake, 7);
        assert_eq!(BOARD_PINS.general.mic_sck, 4);
        assert_eq!(BOARD_PINS.general.spk_din, 17);
        assert_eq!(BOARD_PINS.general.lcd_sclk, 12);
        assert_eq!(BOARD_PINS.general.lcd_mosi, 13);
    }

    #[test]
    fn test_channel_interaction() {
        let frame = AudioFrame::new();
        assert!(AUDIO_TX_CHANNEL.try_send(frame).is_ok());
        let received = AUDIO_TX_CHANNEL.try_receive().unwrap();
        assert_eq!(received.samples[0], 0);

        assert!(AVATAR_STATE_CHANNEL
            .try_send((AvatarState::Speaking, AvatarEmotion::Happy))
            .is_ok());
        let (st, em) = AVATAR_STATE_CHANNEL.try_receive().unwrap();
        assert_eq!(st, AvatarState::Speaking);
        assert_eq!(em, AvatarEmotion::Happy);
    }
}

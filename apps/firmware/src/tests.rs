use crate::audio::{AudioFrame, InvalidFrameLength, BYTES_PER_FRAME, DEFAULT_VAD_THRESHOLD};
use crate::avatar::{AvatarRenderer, AvatarState};
use crate::battery::BatteryMonitor;
use crate::ble::TransportKind;
use crate::pins::BOARD_PINS;
use crate::power::{PowerManager, PowerState, SLEEP_GRACE_TIMEOUT_MS};
use crate::protocol::{
    BohsFrame, ProtocolDispatcher, ProtocolError, FLAG_END_OF_UTTERANCE, FLAG_VOICE_ACTIVITY, OPCODE_AUDIO,
};
use crate::trig::{cos_deg, sin_deg};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::Rgb565,
    primitives::Rectangle,
    Pixel,
};

struct TestBuffer {
    drawn_pixels: usize,
}

impl DrawTarget for TestBuffer {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.drawn_pixels += pixels.into_iter().count();
        Ok(())
    }
}

impl Dimensions for TestBuffer {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), Size::new(240, 240))
    }
}

#[test]
fn test_system_wiring() {
    assert_eq!(BOARD_PINS.general.boot_btn, 0);
    assert_eq!(BOARD_PINS.power.batt_sense, 1);
    assert_eq!(BOARD_PINS.power.chg_stat, 2);
    assert_eq!(BOARD_PINS.general.mic_sck, 4);
    assert_eq!(BOARD_PINS.general.touch_wake, 7);
    assert_eq!(BOARD_PINS.general.i2c_sda, 8);
    assert_eq!(BOARD_PINS.general.i2c_scl, 9);
    assert_eq!(BOARD_PINS.general.lcd_sclk, 12);
    assert_eq!(BOARD_PINS.general.lcd_mosi, 13);
    assert_eq!(BOARD_PINS.general.spk_din, 17);
    assert_eq!(BOARD_PINS.general.usb_dm, 19);
    assert_eq!(BOARD_PINS.general.usb_dp, 20);
}

#[test]
fn test_power_state_machine() {
    let mut pm = PowerManager::new();
    assert_eq!(pm.state, PowerState::DeepSleep);
    pm.on_wake_trigger();
    assert_eq!(pm.state, PowerState::BootWake);
    pm.on_hardware_ready(TransportKind::WifiDirect);
    assert_eq!(pm.state, PowerState::WifiFastAssociate);
    pm.on_ws_connected();
    assert_eq!(pm.state, PowerState::StreamFullDuplex);

    pm.on_audio_activity(false, 1000);
    assert_eq!(pm.state, PowerState::SleepGrace);
    pm.on_audio_activity(true, 20);
    assert_eq!(pm.state, PowerState::StreamFullDuplex);
    pm.on_audio_activity(false, SLEEP_GRACE_TIMEOUT_MS);
    pm.on_audio_activity(false, SLEEP_GRACE_TIMEOUT_MS);
    assert_eq!(pm.state, PowerState::DeepSleep);
    assert_eq!(PowerManager::battery_pct_from_mv(3750), 50);
    assert_eq!(PowerManager::battery_pct_from_mv(3434), 14);
}

#[test]
fn test_low_battery_alerts_once_per_threshold() {
    let mut monitor = BatteryMonitor::new();
    assert_eq!(monitor.update(15), None);
    assert_eq!(monitor.update(14), Some(15));
    assert_eq!(monitor.update(15), None);
    assert_eq!(monitor.update(14), None);
    assert_eq!(monitor.update(4), Some(5));
    assert_eq!(monitor.update(6), None);
    assert_eq!(monitor.update(4), None);
    assert_eq!(monitor.update(8), None);
    assert_eq!(monitor.update(4), Some(5));
    assert_eq!(monitor.update(18), None);
    assert_eq!(monitor.update(14), Some(15));

    let mut boot_low = BatteryMonitor::new();
    assert_eq!(boot_low.update(3), Some(5));
    assert_eq!(boot_low.update(3), None);
}

#[test]
fn test_audio_frame_le_codec_and_vad() {
    let mut frame = AudioFrame::new();
    frame.samples[0] = 0x1234;
    frame.samples[1] = -500;
    let mut bytes = [0u8; BYTES_PER_FRAME];
    frame.write_le_bytes(&mut bytes).unwrap();
    assert_eq!(&bytes[..4], &[0x34, 0x12, 0x0C, 0xFE]);
    assert_eq!(frame.write_le_bytes(&mut [0u8; BYTES_PER_FRAME - 1]), Err(InvalidFrameLength));

    let mut roundtrip = AudioFrame::new();
    roundtrip.copy_from_bytes(&bytes).unwrap();
    assert_eq!(roundtrip.samples[..2], [0x1234, -500]);
    assert_eq!(roundtrip.copy_from_bytes(&bytes[..BYTES_PER_FRAME - 2]), Err(InvalidFrameLength));
    assert_eq!(roundtrip.copy_from_bytes(&[0u8; BYTES_PER_FRAME + 2]), Err(InvalidFrameLength));
    assert!(!roundtrip.is_voice_active(DEFAULT_VAD_THRESHOLD));
    roundtrip.samples.fill(2000);
    assert!(roundtrip.is_voice_active(DEFAULT_VAD_THRESHOLD));
}

#[test]
fn test_avatar_and_trig() {
    assert_eq!(sin_deg(0), 0);
    assert_eq!(sin_deg(90), 1000);
    assert_eq!(cos_deg(0), 1000);

    let mut renderer = AvatarRenderer::new();
    for state in [
        AvatarState::Idle,
        AvatarState::Listening,
        AvatarState::Thinking,
        AvatarState::Speaking,
        AvatarState::Error,
    ] {
        renderer.state = state;
        renderer.step();
        let mut buf = TestBuffer { drawn_pixels: 0 };
        renderer.render(&mut buf).unwrap();
        assert!(buf.drawn_pixels > 0);
    }
}

#[test]
fn test_bohs_wire_framing() {
    let mut buf = [0u8; 2048];
    let flags = FLAG_VOICE_ACTIVITY | FLAG_END_OF_UTTERANCE;
    let payload = [7u8; BYTES_PER_FRAME];
    let len = ProtocolDispatcher::encode_audio_frame(flags, 0x1234, &payload, &mut buf).unwrap();
    assert_eq!((len, buf[0]), (644, OPCODE_AUDIO));

    let dispatcher = ProtocolDispatcher::new();
    let decoded = dispatcher.decode_packet(&buf[..len]).unwrap();
    assert_eq!(decoded, BohsFrame::Audio { flags, seq: 0x1234, payload: &payload });

    let short = ProtocolDispatcher::encode_audio_frame(0, 1, &[1, 2, 3, 4], &mut buf).unwrap();
    assert_eq!(dispatcher.decode_packet(&buf[..short]), Err(ProtocolError::InvalidFrameLength));
    assert_eq!(dispatcher.decode_packet(&buf[..3]), Err(ProtocolError::TruncatedHeader));
    ProtocolDispatcher::encode_audio_frame(0x04, 1, &payload, &mut buf).unwrap();
    assert_eq!(dispatcher.decode_packet(&buf[..644]), Err(ProtocolError::ReservedFlags));
    ProtocolDispatcher::encode_audio_frame(0, 1, &payload, &mut buf).unwrap();
    assert_eq!(dispatcher.decode_packet(&buf[..645]), Err(ProtocolError::InvalidFrameLength));

    buf[..1025].fill(b' ');
    buf[0] = 0x02;
    assert_eq!(dispatcher.decode_packet(&buf[..1025]), Err(ProtocolError::InvalidFrameLength));
    assert_eq!(dispatcher.decode_packet(&[0x04]), Err(ProtocolError::InvalidOpcode(0x04)));
    assert_eq!(dispatcher.decode_packet(&[]), Err(ProtocolError::EmptyPacket));

    let batt_len = ProtocolDispatcher::encode_low_battery_alert(12, 3480, &mut buf).unwrap();
    let alert = core::str::from_utf8(&buf[1..batt_len]).unwrap();
    let expected = r#"{"jsonrpc":"2.0","method":"charm.low_battery","params":{"battery_pct":12,"voltage_mv":3480}}"#;
    assert_eq!(alert, expected);
}

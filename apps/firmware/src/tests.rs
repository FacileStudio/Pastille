use crate::audio::{AudioError, AudioFrame, I2sPipeline, BYTES_PER_FRAME, DEFAULT_VAD_THRESHOLD};
use crate::avatar::{AvatarEmotion, AvatarRenderer, AvatarState};
use crate::ble::TransportKind;
use crate::pins::BOARD_PINS;
use crate::power::{PowerManager, PowerState, SLEEP_GRACE_TIMEOUT_MS};
use crate::protocol::{
    BohsFrame, ProtocolDispatcher, ProtocolError, TapType, FLAG_END_OF_UTTERANCE,
    FLAG_VOICE_ACTIVITY, OPCODE_AUDIO, OPCODE_CONTROL,
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
    assert!(pm.is_amp_active());

    pm.on_audio_activity(false, 1000);
    assert_eq!(pm.state, PowerState::SleepGrace);
    pm.on_audio_activity(true, 20);
    assert_eq!(pm.state, PowerState::StreamFullDuplex);
    pm.on_audio_activity(false, SLEEP_GRACE_TIMEOUT_MS);
    pm.on_audio_activity(false, SLEEP_GRACE_TIMEOUT_MS);
    assert_eq!(pm.state, PowerState::DeepSleep);
    assert_eq!(PowerManager::battery_pct_from_mv(3750), 50);
    assert_eq!(PowerManager::low_battery_threshold(3400), Some(11));
}

#[test]
fn test_audio_pipeline_and_vad() {
    let mut frame = AudioFrame::new();
    frame.samples[0] = 0x1234;
    frame.samples[1] = -500;
    assert_eq!(frame.as_bytes().len(), BYTES_PER_FRAME);

    let mut roundtrip = AudioFrame::new();
    roundtrip.copy_from_bytes(frame.as_bytes()).unwrap();
    assert_eq!(roundtrip.samples[0], 0x1234);
    assert_eq!(roundtrip.samples[1], -500);

    roundtrip.samples.fill(2000);
    assert!(roundtrip.is_voice_active(DEFAULT_VAD_THRESHOLD));

    let mut pipe = I2sPipeline::new();
    assert_eq!(pipe.play_frame(&frame), Err(AudioError::HardwareFault));
    pipe.amp_enabled = true;
    pipe.mic_gain = 50;
    assert_eq!(pipe.play_frame(&frame), Ok(()));
    pipe.capture_frame(&mut roundtrip).unwrap();
    assert_eq!(roundtrip.samples[0], -160);
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
    let len = ProtocolDispatcher::encode_audio_frame(flags, 0x1234, &[1, 2, 3, 4], &mut buf).unwrap();
    assert_eq!(buf[0], OPCODE_AUDIO);

    let mut dispatcher = ProtocolDispatcher::new();
    let decoded = dispatcher.decode_packet(&buf[..len]).unwrap();
    assert_eq!(
        decoded,
        BohsFrame::Audio {
            flags,
            seq: 0x1234,
            payload: &[1, 2, 3, 4]
        }
    );

    let mut avatar = AvatarRenderer::new();
    let list_req = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}"#;
    let list_len = dispatcher.dispatch_json_rpc(list_req, &mut avatar, &mut buf).unwrap();
    assert_eq!(buf[0], OPCODE_CONTROL);
    let list_str = core::str::from_utf8(&buf[1..list_len]).unwrap();
    assert!(list_str.contains("charm.get_telemetry") && !list_str.contains("charm.snapshot"));
}

#[test]
fn test_reverse_mcp_tools_and_events() {
    let mut buf = [0u8; 2048];
    let mut dispatcher = ProtocolDispatcher::new();
    let mut avatar = AvatarRenderer::new();

    let call_req = concat!(
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","#,
        r#""params":{"name":"charm.display_avatar","arguments":{"state":"speaking","emotion":"happy"}}}"#
    );
    dispatcher.dispatch_json_rpc(call_req, &mut avatar, &mut buf).unwrap();
    assert_eq!(avatar.state, AvatarState::Speaking);
    assert_eq!(avatar.emotion, AvatarEmotion::Happy);

    let bad_req = concat!(
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","#,
        r#""params":{"name":"charm.display_avatar","arguments":{"state":"invalid"}}}"#
    );
    assert_eq!(
        dispatcher.dispatch_json_rpc(bad_req, &mut avatar, &mut buf),
        Err(ProtocolError::InvalidArguments)
    );

    let snap_req = concat!(
        r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","#,
        r#""params":{"name":"charm.snapshot","arguments":{"resolution":"320x240"}}}"#
    );
    let snap_len = dispatcher.dispatch_json_rpc(snap_req, &mut avatar, &mut buf).unwrap();
    assert!(core::str::from_utf8(&buf[1..snap_len]).unwrap().contains("-32601"));

    let tap_len = ProtocolDispatcher::encode_tap_event(TapType::DoubleTap, 12345, &mut buf).unwrap();
    assert!(core::str::from_utf8(&buf[1..tap_len]).unwrap().contains("double_tap"));
    let batt_len = ProtocolDispatcher::encode_low_battery_alert(12, 3480, &mut buf).unwrap();
    assert!(core::str::from_utf8(&buf[1..batt_len]).unwrap().contains("3480"));
}

use crate::audio::AudioFrame;
use crate::ble::{ProvisionState, TransportKind};
use crate::events::TapType;
use crate::haptic::HapticPattern;
use crate::link::CONFIG_WATCH;
use crate::link::{device_info, Inbound, Overlay, StoreRequest, HAPTIC_SIGNAL, OVERLAY_SIGNAL, STORE_CHANNEL};
use crate::link::{outbound, CAPTURE_ENABLED};
use crate::mic::VAD_HANGOVER_FRAMES;
use crate::node::Node;
use crate::power::PowerState;
use crate::protocol::{OPCODE_CONTROL, OPCODE_PROVISION};
use crate::tests_link::{frame, next_reply};
use crate::{Packet, MAX_PACKET_BYTES};
use core::sync::atomic::Ordering;

const TOKEN: &str = "bohs_tok_0123456789abcdef0123456789abcdef";

fn packet(opcode: u8, json: &str) -> Packet {
    let bytes = frame(opcode, json);
    let mut buf = [0u8; MAX_PACKET_BYTES];
    buf[..bytes.len()].copy_from_slice(&bytes);
    (buf, bytes.len())
}

fn ble_request(node: &mut Node, json: &str) -> String {
    node.handle(Inbound::Frame(TransportKind::BleRelay, packet(OPCODE_PROVISION, json)));
    next_reply(TransportKind::BleRelay).unwrap()
}

fn expect_state_notification(state: &str) {
    let expected =
        format!("\u{3}{{\"jsonrpc\":\"2.0\",\"method\":\"prov.state\",\"params\":{{\"state\":\"{state}\"}}}}");
    assert_eq!(next_reply(TransportKind::BleRelay).unwrap(), expected);
}

/// Taps to authorize, then configures Wi-Fi direct, and returns the config handed to storage.
fn tap_and_configure(node: &mut Node) -> crate::ble::ProvisionConfig {
    node.handle(Inbound::Tap(TapType::SingleTap, 1000));
    expect_state_notification("authorized");
    let configure = format!(
        concat!(
            r#"{{"jsonrpc":"2.0","id":2,"method":"prov.configure","params":{{"gateway_url":"wss://g.example/ws","#,
            r#""device_token":"{}","wifi_ssid":"Atelier","prefer_ble":false}}}}"#
        ),
        TOKEN
    );
    assert!(ble_request(node, &configure).contains(r#""status":"provisioned""#));
    expect_state_notification("provisioned");
    assert_eq!(OVERLAY_SIGNAL.try_take(), Some(Overlay::None));
    let Ok(StoreRequest::Config(stored)) = STORE_CHANNEL.try_receive() else { panic!("config not persisted") };
    assert_eq!((stored.wifi_ssid.as_str(), stored.prefer_ble), ("Atelier", false));
    assert_eq!(CONFIG_WATCH.try_get().map(|c| c.device_token), Some(TOKEN.try_into().unwrap()));
    assert_eq!(device_info().transport, TransportKind::WifiDirect);

    stored
}

/// Pairs over BLE through the node: identify, tap, configure, and checks every side effect.
pub(crate) fn check_ble_pairing_flow() {
    let mut node = Node::new([1, 2, 3, 4], None);
    node.handle(Inbound::BleConnected(7));
    node.handle(Inbound::BleSecured(true));
    let identify = ble_request(&mut node, r#"{"jsonrpc":"2.0","id":1,"method":"prov.identify"}"#);
    assert!(identify.contains(r#""state":"awaiting_tap""#));
    expect_state_notification("awaiting_tap");
    assert_eq!(HAPTIC_SIGNAL.try_take().map(|p| p.pattern), Some(HapticPattern::DoubleTap));
    assert_eq!(OVERLAY_SIGNAL.try_take(), Some(Overlay::PairingRing));
    assert_eq!(device_info().state, ProvisionState::AwaitingTap);

    let stored = tap_and_configure(&mut node);
    node.handle(Inbound::BleDisconnected);
    assert_eq!((node.prov.conn, node.prov.state), (None, ProvisionState::Provisioned));
    let restored = Node::new([1, 2, 3, 4], Some(stored));
    assert_eq!(
        (restored.prov.state, restored.prov.transport),
        (ProvisionState::Provisioned, TransportKind::WifiDirect)
    );
}

/// Sends taps and battery alerts to the link the gateway last used, and drops them without one.
pub(crate) fn check_gateway_events() {
    let mut node = Node::new([0; 4], None);
    node.handle(Inbound::Tap(TapType::DoubleTap, 5));
    node.handle(Inbound::LowBattery(12, 3480));
    assert_eq!(next_reply(TransportKind::UsbSerial), None);

    node.handle(Inbound::Frame(
        TransportKind::UsbSerial,
        packet(OPCODE_CONTROL, r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#),
    ));
    assert!(next_reply(TransportKind::UsbSerial).unwrap().contains(r#""id":1"#));
    node.handle(Inbound::Tap(TapType::LongPress, 42));
    let tap = next_reply(TransportKind::UsbSerial).unwrap();
    assert!(tap.contains(r#""params":{"type":"long_press","timestamp_ms":"#));
    node.handle(Inbound::LowBattery(4, 3330));
    assert!(next_reply(TransportKind::UsbSerial).unwrap().contains(r#""battery_pct":4"#));
    node.handle(Inbound::LinkDown(TransportKind::UsbSerial));
    node.handle(Inbound::LinkUp(TransportKind::WifiDirect));
    assert_eq!(node.gateway_link, Some(TransportKind::WifiDirect));
    node.handle(Inbound::Tap(TapType::SingleTap, 49));
    assert!(next_reply(TransportKind::WifiDirect).unwrap().contains("single_tap"));
    node.handle(Inbound::LinkDown(TransportKind::WifiDirect));
    node.handle(Inbound::Tap(TapType::SingleTap, 50));
    assert_eq!(next_reply(TransportKind::UsbSerial), None);
}

/// Streams microphone frames only during a session, stops after speech ends, and resumes on a tap.
pub(crate) fn check_audio_streaming() {
    let mut node = Node::new([0; 4], None);
    let (mut loud, quiet) = (AudioFrame::new(), AudioFrame::new());
    loud.samples.fill(2000);
    node.send_audio(&loud);
    assert!(!CAPTURE_ENABLED.load(Ordering::Relaxed));
    let list = packet(OPCODE_CONTROL, r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#);
    node.handle(Inbound::Frame(TransportKind::UsbSerial, list));
    assert!(next_reply(TransportKind::UsbSerial).is_some());
    assert!(CAPTURE_ENABLED.load(Ordering::Relaxed));
    node.send_audio(&loud);
    let (first, len) = outbound(TransportKind::UsbSerial).try_receive().unwrap();
    assert_eq!((len, &first[..4]), (644, &[0x01, 0x02, 0, 0][..]));
    for _ in 0..VAD_HANGOVER_FRAMES {
        node.send_audio(&quiet);
        assert!(next_reply(TransportKind::UsbSerial).is_some());
    }
    assert_eq!(node.power.state, PowerState::SleepGrace);
    node.send_audio(&quiet);
    assert_eq!(next_reply(TransportKind::UsbSerial), None);
    for _ in 0..1_500 {
        node.send_audio(&quiet);
    }
    assert_eq!(node.power.state, PowerState::DeepSleep);
    assert!(!CAPTURE_ENABLED.load(Ordering::Relaxed));
    node.handle(Inbound::Tap(TapType::SingleTap, 9));
    assert!(next_reply(TransportKind::UsbSerial).unwrap().contains("single_tap"));
    assert_eq!(node.power.state, PowerState::StreamFullDuplex);
    assert!(CAPTURE_ENABLED.load(Ordering::Relaxed));
}

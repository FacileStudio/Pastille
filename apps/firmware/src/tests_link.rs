use crate::audio::{AudioFrame, BYTES_PER_FRAME};
use crate::avatar::{AvatarEmotion, AvatarState};
use crate::ble::{ProvisionState, TransportKind};
use crate::link::{Node, AUDIO_RX_CHANNEL, AVATAR_SIGNAL, HAPTIC_SIGNAL, OUTBOUND_PKT_CHANNEL};
use crate::protocol::{ProtocolDispatcher, FLAG_END_OF_UTTERANCE, FLAG_VOICE_ACTIVITY, OPCODE_CONTROL, OPCODE_PROVISION};
use crate::MAX_PACKET_BYTES;

fn frame(opcode: u8, json: &str) -> Vec<u8> {
    let mut bytes = vec![opcode];
    bytes.extend_from_slice(json.as_bytes());
    bytes
}

fn next_reply() -> Option<String> {
    let (pkt, len) = OUTBOUND_PKT_CHANNEL.try_receive().ok()?;
    Some(String::from_utf8(pkt[..len].to_vec()).unwrap())
}

fn tool_call(id: u32, name: &str, arguments: &str) -> Vec<u8> {
    let json = format!(
        r#"{{"jsonrpc":"2.0","id":{},"method":"tools/call","params":{{"name":"{}","arguments":{}}}}}"#,
        id, name, arguments
    );
    frame(OPCODE_CONTROL, &json)
}

fn check_audio_routing(node: &mut Node) {
    let mut audio = [0u8; 4 + BYTES_PER_FRAME];
    let len = ProtocolDispatcher::encode_audio_frame(0, 9, &[0; BYTES_PER_FRAME], &mut audio).unwrap();
    node.handle_inbound(TransportKind::WifiDirect, &audio[..len]);
    assert!(AUDIO_RX_CHANNEL.try_receive().is_ok());
    node.handle_inbound(TransportKind::WifiDirect, &audio[..len - 1]);
    assert!(AUDIO_RX_CHANNEL.try_receive().is_err());
}

fn check_control_routing(node: &mut Node) {
    for (id, state) in [(1, "listening"), (2, "speaking"), (3, "idle")] {
        let args = format!(r#"{{"state":"{state}"}}"#);
        node.handle_inbound(TransportKind::UsbSerial, &tool_call(id, "charm.display_avatar", &args));
        assert!(next_reply().unwrap().contains(&format!(r#""id":{id},"result""#)));
    }
    assert_eq!(AVATAR_SIGNAL.try_take(), Some((AvatarState::Idle, AvatarEmotion::Neutral)));
    assert_eq!(AVATAR_SIGNAL.try_take(), None);

    node.handle_inbound(TransportKind::UsbSerial, &tool_call(6, "charm.haptic_pulse", r#"{"pattern":"tap"}"#));
    assert!(next_reply().unwrap().contains(r#""id":6,"result""#));
    assert!(HAPTIC_SIGNAL.try_take().is_some());
    node.handle_inbound(TransportKind::UsbSerial, &tool_call(7, "charm.haptic_pulse", r#"{"intensity":500}"#));
    assert!(next_reply().unwrap().contains(r#""id":7,"error":{"code":-32602"#));
    assert!(HAPTIC_SIGNAL.try_take().is_none());
}

fn check_provision_routing(node: &mut Node) {
    let identify = frame(OPCODE_PROVISION, r#"{"jsonrpc":"2.0","id":8,"method":"prov.identify"}"#);
    node.handle_inbound(TransportKind::WifiDirect, &identify);
    assert_eq!(next_reply(), None);
    node.handle_inbound(TransportKind::UsbSerial, &identify);
    let reply = next_reply().unwrap();
    assert!(reply.starts_with('\u{3}') && reply.contains("-32001"));
    assert_eq!(node.prov.state, ProvisionState::Unclaimed);
}

#[test]
fn test_inbound_routing_and_outbound_queue() {
    let mut node = Node::new([0x4F, 0x2A, 0x91, 0xC0]);
    check_audio_routing(&mut node);
    check_control_routing(&mut node);
    check_provision_routing(&mut node);
}

#[test]
fn test_audio_uplink_flags_and_sequence() {
    let mut node = Node::new([0; 4]);
    let mut loud = AudioFrame::new();
    loud.samples.fill(2000);
    let mut out = [0u8; MAX_PACKET_BYTES];
    assert_eq!(node.encode_audio(&loud, &mut out), Some(644));
    assert_eq!(&out[..4], &[0x01, FLAG_VOICE_ACTIVITY, 0, 0]);
    assert_eq!(&out[4..6], &2000i16.to_le_bytes());
    assert_eq!(node.encode_audio(&AudioFrame::new(), &mut out), Some(644));
    assert_eq!(&out[..4], &[0x01, FLAG_END_OF_UTTERANCE, 0, 1]);
    assert_eq!(node.encode_audio(&loud, &mut [0u8; 100]), None);
}

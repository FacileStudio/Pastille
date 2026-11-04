use crate::avatar::{AvatarEmotion, AvatarRenderer, AvatarState};
use crate::haptic::{HapticPattern, HapticPulse};
use crate::protocol::{DeviceTelemetry, ProtocolDispatcher, OPCODE_CONTROL};
use crate::MAX_PACKET_BYTES;

fn call(dispatcher: &mut ProtocolDispatcher, avatar: &mut AvatarRenderer, req: &str) -> Option<String> {
    let mut out = [0u8; MAX_PACKET_BYTES];
    let len = dispatcher.dispatch_json_rpc(req, avatar, &mut out)?;
    assert_eq!(out[0], OPCODE_CONTROL);
    Some(String::from_utf8(out[1..len].to_vec()).unwrap())
}

fn tool_call(id: u32, name: &str, arguments: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{},"method":"tools/call","params":{{"name":"{}","arguments":{}}}}}"#,
        id, name, arguments
    )
}

fn error(id: &str, code: i32, message: &str) -> String {
    format!(r#"{{"jsonrpc":"2.0","id":{},"error":{{"code":{},"message":"{}"}}}}"#, id, code, message)
}

#[test]
fn test_tools_list_and_avatar_call() {
    let (mut dispatcher, mut avatar) = (ProtocolDispatcher::new(), AvatarRenderer::new());
    let list = call(&mut dispatcher, &mut avatar, r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}"#);
    let list = list.unwrap();
    assert!(list.starts_with(r#"{"jsonrpc":"2.0","id":1,"result":{"tools":[{"name":"charm.display_avatar""#));
    assert!(list.contains("charm.get_telemetry") && !list.contains("charm.snapshot"));
    assert!(list.len() < MAX_PACKET_BYTES - 64);

    let req = tool_call(2, "charm.display_avatar", r#"{"state":"speaking","emotion":"happy"}"#);
    assert!(call(&mut dispatcher, &mut avatar, &req).unwrap().contains("speaking (happy)"));
    assert_eq!((avatar.state, avatar.emotion), (AvatarState::Speaking, AvatarEmotion::Happy));

    let bad = tool_call(3, "charm.display_avatar", r#"{"state":"invalid"}"#);
    assert_eq!(call(&mut dispatcher, &mut avatar, &bad).unwrap(), error("3", -32602, "invalid params"));
    assert_eq!(avatar.state, AvatarState::Speaking);

    let snap = tool_call(4, "charm.snapshot", r#"{"resolution":"320x240"}"#);
    assert!(call(&mut dispatcher, &mut avatar, &snap).unwrap().contains(r#""code":-32601"#));
}

#[test]
fn test_every_request_gets_an_answer() {
    let (mut dispatcher, mut avatar) = (ProtocolDispatcher::new(), AvatarRenderer::new());
    let mut ask = |req: &str| call(&mut dispatcher, &mut avatar, req);
    let unknown = r#"{"jsonrpc":"2.0","id":"a","method":"resources/list"}"#;
    assert_eq!(ask(unknown).unwrap(), error(r#""a""#, -32601, "method not found"));
    assert_eq!(ask(&tool_call(5, "charm.teleport", "{}")).unwrap(), error("5", -32602, "unknown tool"));
    let no_params = r#"{"jsonrpc":"2.0","id":6,"method":"tools/call"}"#;
    assert_eq!(ask(no_params).unwrap(), error("6", -32602, "invalid params"));
    assert_eq!(ask("{not json").unwrap(), error("null", -32700, "parse error"));
    assert_eq!(ask("[1,2]").unwrap(), error("null", -32600, "invalid request"));
    let object_id = r#"{"jsonrpc":"2.0","id":{"x":1},"method":"tools/list"}"#;
    assert_eq!(ask(object_id).unwrap(), error("null", -32600, "invalid request"));
    let long_id = format!(r#"{{"jsonrpc":"2.0","id":"{}","method":"tools/list"}}"#, "x".repeat(65));
    assert_eq!(ask(&long_id).unwrap(), error("null", -32600, "invalid request"));
    let max_id = format!(r#"{{"jsonrpc":"2.0","id":"{}","method":"tools/list"}}"#, "x".repeat(64));
    assert!(ask(&max_id).unwrap().contains(r#""result":{"tools":"#));
    assert_eq!(ask(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#), None);
    let notified_call = r#"{"jsonrpc":"2.0","method":"tools/call","params":{"name":"charm.get_telemetry"}}"#;
    assert_eq!(ask(notified_call), None);
}

#[test]
fn test_haptic_pulse_validation() {
    let (mut dispatcher, mut avatar) = (ProtocolDispatcher::new(), AvatarRenderer::new());
    let req = tool_call(7, "charm.haptic_pulse", r#"{"pattern":"double_tap","duration_ms":60,"intensity":80}"#);
    assert!(call(&mut dispatcher, &mut avatar, &req).unwrap().contains("Haptic pulse dispatched"));
    let pulse = HapticPulse { pattern: HapticPattern::DoubleTap, duration_ms: 60, intensity: 80 };
    assert_eq!(dispatcher.haptic.take(), Some(pulse));
    assert_eq!((pulse.rtp_level(), pulse.pattern.pulse_count()), (204, 2));

    let defaults = tool_call(8, "charm.haptic_pulse", r#"{"pattern":"tap"}"#);
    call(&mut dispatcher, &mut avatar, &defaults).unwrap();
    assert_eq!(dispatcher.haptic.take().map(|p| (p.duration_ms, p.intensity)), Some((50, 80)));

    let invalid = [
        r#"{"pattern":"tap","intensity":500}"#,
        r#"{"pattern":"buzz"}"#,
        r#"{"pattern":"tap","duration_ms":"x"}"#,
    ];
    for args in invalid {
        let reply = call(&mut dispatcher, &mut avatar, &tool_call(9, "charm.haptic_pulse", args)).unwrap();
        assert_eq!(reply, error("9", -32602, "invalid params"));
        assert_eq!(dispatcher.haptic, None);
    }
}

#[test]
fn test_telemetry_reports_missing_sources_as_null() {
    let (mut dispatcher, mut avatar) = (ProtocolDispatcher::new(), AvatarRenderer::new());
    dispatcher.telemetry = DeviceTelemetry { battery_pct: 64, is_charging: true, rssi_dbm: None, temp_c: None };
    let req = tool_call(10, "charm.get_telemetry", "{}");
    let expected = r#"{\"battery_pct\":64,\"is_charging\":true,\"rssi_dbm\":null,\"temp_c\":null}"#;
    assert!(call(&mut dispatcher, &mut avatar, &req).unwrap().contains(expected));

    let full = DeviceTelemetry { battery_pct: 87, is_charging: false, rssi_dbm: Some(-54), temp_c: Some(31.84) };
    dispatcher.telemetry = full;
    let expected = r#"{\"battery_pct\":87,\"is_charging\":false,\"rssi_dbm\":-54,\"temp_c\":31.8}"#;
    assert!(call(&mut dispatcher, &mut avatar, &req).unwrap().contains(expected));
}

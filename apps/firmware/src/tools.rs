use crate::avatar::{AvatarEmotion, AvatarRenderer, AvatarState};
use crate::haptic::{HapticPattern, HapticPulse};
use crate::protocol::{Body, ProtocolDispatcher, ProtocolError};
use core::fmt::{Display, Write};
use heapless::String;

const TOOLS_SCHEMA: &str = concat!(
    r#"[{"name":"charm.display_avatar","#,
    r#""description":"Updates the face expression on the circular LCD display","#,
    r#""inputSchema":{"type":"object","properties":{"#,
    r#""state":{"type":"string","enum":["idle","listening","thinking","speaking","error"]},"#,
    r#""emotion":{"type":"string","enum":["neutral","happy","focused","calm","curious","warn"]}},"#,
    r#""required":["state"]}},"#,
    r#"{"name":"charm.haptic_pulse","#,
    r#""description":"Triggers a localized vibration pattern on the wearable motor","#,
    r#""inputSchema":{"type":"object","properties":{"#,
    r#""pattern":{"type":"string","enum":["tap","double_tap","alert","error"]},"#,
    r#""duration_ms":{"type":"integer","minimum":10,"maximum":500},"#,
    r#""intensity":{"type":"integer","minimum":1,"maximum":100}},"required":["pattern"]}},"#,
    r#"{"name":"charm.get_telemetry","#,
    r#""description":"Reads physical battery status, RSSI, and temperature from the peripheral","#,
    r#""inputSchema":{"type":"object","properties":{}}}]"#
);

fn parse_avatar_args(
    state_str: &str,
    emotion_str: &str,
) -> Result<(AvatarState, AvatarEmotion), ProtocolError> {
    let state = match state_str {
        "idle" => AvatarState::Idle,
        "listening" => AvatarState::Listening,
        "thinking" => AvatarState::Thinking,
        "speaking" => AvatarState::Speaking,
        "error" => AvatarState::Error,
        _ => return Err(ProtocolError::InvalidArguments),
    };
    let emotion = match emotion_str {
        "neutral" => AvatarEmotion::Neutral,
        "happy" => AvatarEmotion::Happy,
        "focused" => AvatarEmotion::Focused,
        "calm" => AvatarEmotion::Calm,
        "curious" => AvatarEmotion::Curious,
        "warn" => AvatarEmotion::Warn,
        _ => return Err(ProtocolError::InvalidArguments),
    };
    Ok((state, emotion))
}

pub fn write_tools_list(
    id: &serde_json::Value,
    out: &mut Body,
) -> Result<(), ProtocolError> {
    write!(
        out,
        r#"{{"jsonrpc":"2.0","id":{},"result":{{"tools":{}}}}}"#,
        id, TOOLS_SCHEMA
    )
    .map_err(|_| ProtocolError::BufferTooSmall)
}

pub fn execute_tool(
    dispatcher: &mut ProtocolDispatcher,
    params: &serde_json::Value,
    id: &serde_json::Value,
    avatar: &mut AvatarRenderer,
    out: &mut Body,
) -> Result<(), ProtocolError> {
    let tool_name = params
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or(ProtocolError::InvalidArguments)?;
    let args = params.get("arguments").unwrap_or(&serde_json::Value::Null);

    match tool_name {
        "charm.display_avatar" => exec_display_avatar(args, id, avatar, out),
        "charm.haptic_pulse" => exec_haptic_pulse(dispatcher, args, id, out),
        "charm.get_telemetry" => exec_get_telemetry(dispatcher, id, out),
        "charm.snapshot" => write_capability_error(id, out),
        _ => Err(ProtocolError::UnknownTool),
    }
}

fn exec_display_avatar(
    args: &serde_json::Value,
    id: &serde_json::Value,
    avatar: &mut AvatarRenderer,
    out: &mut Body,
) -> Result<(), ProtocolError> {
    let state_str = args
        .get("state")
        .and_then(|v| v.as_str())
        .ok_or(ProtocolError::InvalidArguments)?;
    let emotion_str = args
        .get("emotion")
        .and_then(|v| v.as_str())
        .unwrap_or("neutral");

    let (new_state, new_emotion) = parse_avatar_args(state_str, emotion_str)?;
    avatar.state = new_state;
    avatar.emotion = new_emotion;

    write!(
        out,
        concat!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"content":[{{"type":"text","#,
            r#""text":"Avatar state updated to {} ({})"}}],"isError":false}}}}"#
        ),
        id, state_str, emotion_str
    )
    .map_err(|_| ProtocolError::BufferTooSmall)
}

fn exec_haptic_pulse(
    dispatcher: &mut ProtocolDispatcher,
    args: &serde_json::Value,
    id: &serde_json::Value,
    out: &mut Body,
) -> Result<(), ProtocolError> {
    let pattern = args
        .get("pattern")
        .and_then(|v| v.as_str())
        .ok_or(ProtocolError::InvalidArguments)
        .and_then(HapticPattern::parse)?;
    let duration_ms = args.get("duration_ms").map_or(Some(50), |v| v.as_u64()).unwrap_or(0);
    let intensity = args.get("intensity").map_or(Some(80), |v| v.as_u64()).unwrap_or(0);
    if !(10..=500).contains(&duration_ms) || !(1..=100).contains(&intensity) {
        return Err(ProtocolError::InvalidArguments);
    }
    dispatcher.haptic = Some(HapticPulse { pattern, duration_ms: duration_ms as u16, intensity: intensity as u8 });

    write!(
        out,
        concat!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"content":[{{"type":"text","#,
            r#""text":"Haptic pulse dispatched successfully"}}],"isError":false}}}}"#
        ),
        id
    )
    .map_err(|_| ProtocolError::BufferTooSmall)
}

fn exec_get_telemetry(
    dispatcher: &ProtocolDispatcher,
    id: &serde_json::Value,
    out: &mut Body,
) -> Result<(), ProtocolError> {
    let telem = dispatcher.telemetry;
    write!(
        out,
        concat!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"content":[{{"type":"text","#,
            r#""text":"{{\"battery_pct\":{},\"is_charging\":{},"#,
            r#"\"rssi_dbm\":{},\"temp_c\":{}}}"}}],"isError":false}}}}"#
        ),
        id,
        telem.battery_pct,
        telem.is_charging,
        json_or_null(telem.rssi_dbm),
        json_or_null(telem.temp_c)
    )
    .map_err(|_| ProtocolError::BufferTooSmall)
}

/// Formats a reading with one decimal for floats, or `null` when the device has no source for it.
fn json_or_null<T: Display>(value: Option<T>) -> String<16> {
    let mut text = String::new();
    let _ = match value {
        Some(v) => write!(text, "{:.1}", v),
        None => text.push_str("null").map_err(|_| core::fmt::Error),
    };
    text
}

fn write_capability_error(
    id: &serde_json::Value,
    out: &mut Body,
) -> Result<(), ProtocolError> {
    write!(
        out,
        concat!(
            r#"{{"jsonrpc":"2.0","id":{},"error":{{"code":-32601,"#,
            r#""message":"Hardware capability not available on this device"}}}}"#
        ),
        id
    )
    .map_err(|_| ProtocolError::BufferTooSmall)
}

use crate::avatar::{AvatarEmotion, AvatarRenderer, AvatarState};
use crate::protocol::{ProtocolDispatcher, ProtocolError};
use core::fmt::Write;
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
    r#""inputSchema":{"type":"object","properties":{}}},"#,
    r#"{"name":"charm.snapshot","#,
    r#""description":"Captures a still image from an optional camera sensor","#,
    r#""inputSchema":{"type":"object","properties":{"#,
    r#""resolution":{"type":"string","enum":["160x120","320x240","640x480"]},"#,
    r#""format":{"type":"string","enum":["jpeg"]}}}}]"#
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
    out: &mut String<2048>,
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
    out: &mut String<2048>,
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
    out: &mut String<2048>,
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
    out: &mut String<2048>,
) -> Result<(), ProtocolError> {
    let pattern = args
        .get("pattern")
        .and_then(|v| v.as_str())
        .ok_or(ProtocolError::InvalidArguments)?;
    if !matches!(pattern, "tap" | "double_tap" | "alert" | "error") {
        return Err(ProtocolError::InvalidArguments);
    }

    let duration_ms = args.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(50);
    let intensity = args.get("intensity").and_then(|v| v.as_u64()).unwrap_or(80);
    if !(10..=500).contains(&duration_ms) || !(1..=100).contains(&intensity) {
        return Err(ProtocolError::InvalidArguments);
    }

    dispatcher.last_haptic_pattern.clear();
    let _ = dispatcher.last_haptic_pattern.push_str(pattern);
    dispatcher.last_haptic_duration = duration_ms as u32;
    dispatcher.last_haptic_intensity = intensity as u8;

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
    out: &mut String<2048>,
) -> Result<(), ProtocolError> {
    let telem = dispatcher.telemetry;
    let chg = if telem.is_charging { "true" } else { "false" };
    write!(
        out,
        concat!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"content":[{{"type":"text","#,
            r#""text":"{{\"battery_pct\":{},\"is_charging\":{},"#,
            r#"\"rssi_dbm\":{},\"temp_c\":{:.1}}}"}}],"isError":false}}}}"#
        ),
        id, telem.battery_pct, chg, telem.rssi_dbm, telem.temp_c
    )
    .map_err(|_| ProtocolError::BufferTooSmall)
}

fn write_capability_error(
    id: &serde_json::Value,
    out: &mut String<2048>,
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

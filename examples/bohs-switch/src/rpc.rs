use crate::output::Output;
use crate::tools;
use serde_json::{Value, json};

/// The largest JSON-RPC payload in one bohs.v1 frame, after the opcode byte.
pub const MAX_PAYLOAD: usize = 1023;

/// The switch as the gateway sees it: its name and the load it drives.
pub struct Switch<'a> {
    pub name: &'a str,
    pub out: &'a mut dyn Output,
}

/// Answers one `0x02` payload from the gateway, or nothing for a notification.
pub fn handle(payload: &[u8], sw: &mut Switch) -> Option<Vec<u8>> {
    let message: Value = match serde_json::from_slice(payload) {
        Ok(message) => message,
        Err(_) => return Some(error(&Value::Null, -32700, "parse error")),
    };
    let Some(object) = message.as_object() else {
        return Some(error(&Value::Null, -32600, "invalid request"));
    };
    let id = object.get("id")?;
    if !valid_id(id) {
        return Some(error(&Value::Null, -32600, "invalid request"));
    }
    let params = object.get("params").cloned().unwrap_or(Value::Null);
    let answer = match object.get("method").and_then(Value::as_str) {
        Some("tools/list") => Ok(tools::list()),
        Some("tools/call") => tools::call(&params, sw),
        _ => Err((-32601, "method not found".to_string())),
    };
    let body = match answer {
        Ok(result) => serde_json::to_vec(&json!({"jsonrpc": "2.0", "id": id, "result": result})).ok()?,
        Err((code, message)) => error(id, code, &message),
    };
    Some(if body.len() > MAX_PAYLOAD {
        error(id, -32603, "internal error")
    } else {
        body
    })
}

fn valid_id(id: &Value) -> bool {
    match id {
        Value::String(s) => s.len() <= 64,
        Value::Number(_) | Value::Null => true,
        _ => false,
    }
}

fn error(id: &Value, code: i32, message: &str) -> Vec<u8> {
    let body = json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}});
    serde_json::to_vec(&body).unwrap_or_default()
}

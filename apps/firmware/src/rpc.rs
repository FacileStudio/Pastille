use crate::protocol::ProtocolError;
use core::fmt::Write;
use serde_json::Value;

/// Longest string `id` the device echoes; longer ids get `-32600` so every reply fits its buffer.
pub const MAX_ID_LEN: usize = 64;

/// A JSON-RPC 2.0 message from a `0x02` or `0x03` frame, sorted by what the device must do with it.
pub enum Incoming {
    /// A request with a valid `id`: run it and answer.
    Call { value: Value, id: Value },
    /// A message without `id`: the device ignores it and never answers.
    Notification,
    /// Unparsable or malformed: answer with this error and `"id":null`.
    Invalid(ProtocolError),
}

/// Parses `json` and validates its `id`; only a string, a number or `null` is accepted.
pub fn parse_request(json: &str) -> Incoming {
    let Ok(value) = serde_json::from_str::<Value>(json) else {
        return Incoming::Invalid(ProtocolError::JsonParseError);
    };
    if !value.is_object() {
        return Incoming::Invalid(ProtocolError::InvalidRequest);
    }
    let id = match value.get("id") {
        None => return Incoming::Notification,
        Some(id) => id.clone(),
    };
    let valid = match &id {
        Value::String(s) => s.len() <= MAX_ID_LEN,
        Value::Number(_) | Value::Null => true,
        _ => false,
    };
    if !valid {
        return Incoming::Invalid(ProtocolError::InvalidRequest);
    }
    Incoming::Call { value, id }
}

/// Maps a dispatch failure to its JSON-RPC error code and message.
pub fn error_code(err: ProtocolError) -> (i32, &'static str) {
    match err {
        ProtocolError::JsonParseError => (-32700, "parse error"),
        ProtocolError::InvalidRequest => (-32600, "invalid request"),
        ProtocolError::UnknownMethod => (-32601, "method not found"),
        ProtocolError::UnknownTool => (-32602, "unknown tool"),
        ProtocolError::InvalidArguments => (-32602, "invalid params"),
        ProtocolError::Unauthorized => (-32001, "unauthorized"),
        _ => (-32603, "internal error"),
    }
}

/// Writes a JSON-RPC error response for `id` into `out`.
pub fn write_error<W: Write>(id: &Value, err: ProtocolError, out: &mut W) -> Result<(), ProtocolError> {
    let (code, message) = error_code(err);
    write!(
        out,
        r#"{{"jsonrpc":"2.0","id":{},"error":{{"code":{},"message":"{}"}}}}"#,
        id, code, message
    )
    .map_err(|_| ProtocolError::BufferTooSmall)
}

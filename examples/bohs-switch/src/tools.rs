use crate::rpc::Switch;
use serde_json::{Value, json};

type Answer = Result<Value, (i32, String)>;

/// Answers the `tools/list` result: the field profile's switch tools.
pub fn list() -> Value {
    json!({"tools": [
        {
            "name": "switch.set",
            "description": "Turns the load on or off",
            "inputSchema": {
                "type": "object",
                "properties": {"state": {"type": "string", "enum": ["on", "off", "toggle"]}},
                "required": ["state"]
            }
        },
        {
            "name": "switch.get",
            "description": "Reads whether the load is on or off",
            "inputSchema": {"type": "object", "properties": {}}
        }
    ]})
}

/// Runs one `tools/call` and answers its MCP result.
pub fn call(params: &Value, sw: &mut Switch) -> Answer {
    let args = params.get("arguments").cloned().unwrap_or_else(|| json!({}));
    match params.get("name").and_then(Value::as_str) {
        Some("switch.get") => Ok(text(sw)),
        Some("switch.set") => set(&args, sw),
        _ => Err((-32602, "unknown tool".into())),
    }
}

fn set(args: &Value, sw: &mut Switch) -> Answer {
    let on = match args.get("state").and_then(Value::as_str) {
        Some("on") => true,
        Some("off") => false,
        Some("toggle") => !sw.out.is_on(),
        _ => return Err((-32602, "invalid params".into())),
    };
    match sw.out.set(on) {
        Ok(()) => Ok(text(sw)),
        Err(reason) => Ok(json!({"content": [{"type": "text", "text": reason}], "isError": true})),
    }
}

fn text(sw: &Switch) -> Value {
    let state = if sw.out.is_on() { "on" } else { "off" };
    json!({"content": [{"type": "text", "text": format!("{} is {state}", sw.name)}], "isError": false})
}

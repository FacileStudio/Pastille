use crate::identity::from_text;
use crate::output::Memory;
use crate::rpc::{MAX_PAYLOAD, Switch, handle};
use serde_json::{Value, json};

fn ask(sw: &mut Switch, request: Value) -> Value {
    let reply = handle(request.to_string().as_bytes(), sw).expect("a request gets an answer");
    assert!(
        reply.len() <= MAX_PAYLOAD,
        "answer of {} bytes does not fit a frame",
        reply.len()
    );
    serde_json::from_slice(&reply).unwrap()
}

fn call(sw: &mut Switch, name: &str, args: Value) -> Value {
    ask(
        sw,
        json!({"jsonrpc": "2.0", "id": 7, "method": "tools/call", "params": {"name": name, "arguments": args}}),
    )
}

#[test]
fn lists_the_switch_tools() {
    let mut out = Memory::default();
    let mut sw = Switch {
        name: "Living Room Light",
        out: &mut out,
    };
    let reply = ask(
        &mut sw,
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {}}),
    );
    let names: Vec<&str> = reply["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["switch.set", "switch.get"]);
}

#[test]
fn switches_the_load_and_says_so() {
    let mut out = Memory::default();
    let mut sw = Switch {
        name: "Living Room Light",
        out: &mut out,
    };
    let reply = call(&mut sw, "switch.set", json!({"state": "on"}));
    assert_eq!(reply["id"], 7);
    assert_eq!(reply["result"]["content"][0]["text"], "Living Room Light is on");
    assert_eq!(
        call(&mut sw, "switch.set", json!({"state": "toggle"}))["result"]["content"][0]["text"],
        "Living Room Light is off"
    );
    assert_eq!(
        call(&mut sw, "switch.get", json!({}))["result"]["content"][0]["text"],
        "Living Room Light is off"
    );
}

#[test]
fn refuses_bad_tool_calls() {
    let mut out = Memory::default();
    let mut sw = Switch {
        name: "Lamp",
        out: &mut out,
    };
    assert_eq!(
        call(&mut sw, "switch.set", json!({"state": "dim"}))["error"]["code"],
        -32602
    );
    assert_eq!(call(&mut sw, "switch.fly", json!({}))["error"]["code"], -32602);
    assert_eq!(
        ask(&mut sw, json!({"jsonrpc": "2.0", "id": 2, "method": "ping"}))["error"]["code"],
        -32601
    );
}

#[test]
fn refuses_malformed_requests_and_ignores_notifications() {
    let mut out = Memory::default();
    let mut sw = Switch {
        name: "Lamp",
        out: &mut out,
    };
    assert_eq!(ask(&mut sw, json!([1]))["error"]["code"], -32600);
    let bad_id = json!({"jsonrpc": "2.0", "id": {"x": 1}, "method": "tools/list"});
    assert_eq!(ask(&mut sw, bad_id)["error"]["code"], -32600);
    let parse = handle(b"{not json", &mut sw).unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&parse).unwrap()["error"]["code"],
        -32700
    );
    let notification = json!({"jsonrpc": "2.0", "method": "tools/list"}).to_string();
    assert!(handle(notification.as_bytes(), &mut sw).is_none());
}

#[test]
fn derives_the_hardware_id_from_the_serial() {
    assert_eq!(from_text("00000000b8e1c0de\0").as_deref(), Some("b8e1c0de"));
    assert_eq!(
        from_text("Hardware\t: BCM2835\nSerial\t\t: 00000000B8E1C0DE\nModel\t: Pi").as_deref(),
        Some("b8e1c0de")
    );
    assert_eq!(from_text("0000000000000000"), None);
    assert_eq!(from_text("abc"), None);
}

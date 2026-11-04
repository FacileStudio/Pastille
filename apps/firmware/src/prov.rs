use crate::ble::{BleProvisioner, ProvisionConfig, TransportKind, DEVICE_TOKEN_LEN, DEVICE_TOKEN_PREFIX};
use crate::protocol::{write_framed_json, ProtocolError, OPCODE_PROVISION};
use crate::rpc::{parse_request, write_error, Incoming};
use core::fmt::Write;
use heapless::String;
use serde_json::Value;

const STATE_NAMES: [&str; 4] = ["unclaimed", "awaiting_tap", "authorized", "provisioned"];
const TRANSPORT_NAMES: [&str; 3] = ["ble", "wifi", "usb"];
const GATEWAY_SCHEME: &str = "wss://";

impl BleProvisioner {
    /// Handles a `0x03` JSON-RPC message (`prov.identify`, `prov.status`, `prov.configure`) that
    /// arrived over `link` and writes the `0x03` response frame. Every request gets `-32001` unless
    /// it came from a connected BLE central over an encrypted link. Notifications get no answer.
    pub fn dispatch_provision_rpc(&mut self, json_str: &str, link: TransportKind, out: &mut [u8]) -> Option<usize> {
        let allowed = link == TransportKind::BleRelay && self.link_encrypted && self.conn.is_some();
        let (id, result) = match parse_request(json_str) {
            Incoming::Notification => return None,
            Incoming::Invalid(err) => (Value::Null, Err(err)),
            Incoming::Call { id, .. } if !allowed => (id, Err(ProtocolError::Unauthorized)),
            Incoming::Call { value, id } => {
                let result = self.run_method(&value);
                (id, result)
            }
        };
        let mut body: String<256> = String::new();
        match result {
            Ok(false) => write_status(self, &id, &mut body),
            Ok(true) => write_configured(&id, &mut body),
            Err(err) => write_error(&id, err, &mut body),
        }
        .ok()?;
        write_framed_json(OPCODE_PROVISION, body.as_bytes(), out).ok()
    }

    fn run_method(&mut self, value: &Value) -> Result<bool, ProtocolError> {
        match value.get("method").and_then(Value::as_str) {
            Some("prov.identify") => {
                self.on_identify_request();
                Ok(false)
            }
            Some("prov.status") => Ok(false),
            Some("prov.configure") => {
                let cfg = parse_config(value.get("params").unwrap_or(&Value::Null))?;
                self.apply_config(cfg).map(|()| true)
            }
            _ => Err(ProtocolError::UnknownMethod),
        }
    }
}

/// Parses and validates `prov.configure` params; `wifi_ssid` and `wifi_psk` default to empty.
fn parse_config(params: &Value) -> Result<ProvisionConfig, ProtocolError> {
    let cfg = ProvisionConfig {
        gateway_url: copy_field(params, "gateway_url", true)?,
        device_token: copy_field(params, "device_token", true)?,
        wifi_ssid: copy_field(params, "wifi_ssid", false)?,
        wifi_psk: copy_field(params, "wifi_psk", false)?,
        prefer_ble: params.get("prefer_ble").and_then(Value::as_bool).ok_or(ProtocolError::InvalidArguments)?,
    };
    let wifi_missing = !cfg.prefer_ble && cfg.wifi_ssid.is_empty();
    let has_host = cfg.gateway_url.strip_prefix(GATEWAY_SCHEME).is_some_and(|rest| {
        rest.split(['/', '?', '#']).next().is_some_and(|host| !host.is_empty())
    });
    if !has_host || !valid_token(&cfg.device_token) || wifi_missing {
        return Err(ProtocolError::InvalidArguments);
    }
    Ok(cfg)
}

/// Copies a string field; a present field that is not a string is rejected, not treated as empty.
fn copy_field<const N: usize>(params: &Value, key: &str, required: bool) -> Result<String<N>, ProtocolError> {
    let raw = match params.get(key) {
        None => "",
        Some(v) => v.as_str().ok_or(ProtocolError::InvalidArguments)?,
    };
    if required && raw.is_empty() {
        return Err(ProtocolError::InvalidArguments);
    }
    String::try_from(raw).map_err(|_| ProtocolError::InvalidArguments)
}

fn valid_token(token: &str) -> bool {
    token.len() == DEVICE_TOKEN_LEN
        && token.starts_with(DEVICE_TOKEN_PREFIX)
        && token.bytes().skip(DEVICE_TOKEN_PREFIX.len()).all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn write_status(prov: &BleProvisioner, id: &Value, out: &mut String<256>) -> Result<(), ProtocolError> {
    write!(
        out,
        concat!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"state":"{}","#,
            r#""transport":"{}","link_encrypted":{},"hardware_id":"{:02x}{:02x}{:02x}{:02x}"}}}}"#
        ),
        id,
        STATE_NAMES[prov.state as usize],
        TRANSPORT_NAMES[prov.transport as usize],
        prov.link_encrypted,
        prov.hardware_id[0],
        prov.hardware_id[1],
        prov.hardware_id[2],
        prov.hardware_id[3]
    )
    .map_err(|_| ProtocolError::BufferTooSmall)
}

fn write_configured(id: &Value, out: &mut String<256>) -> Result<(), ProtocolError> {
    write!(out, r#"{{"jsonrpc":"2.0","id":{},"result":{{"status":"provisioned"}}}}"#, id)
        .map_err(|_| ProtocolError::BufferTooSmall)
}

use crate::ble::ProvisionState;
use crate::prov::state_name;
use crate::protocol::{write_framed_json, ProtocolDispatcher, ProtocolError, OPCODE_CONTROL, OPCODE_PROVISION};
use core::fmt::Write;
use heapless::String;

/// Touch gestures reported by `charm.tap_event`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapType {
    SingleTap,
    DoubleTap,
    LongPress,
}

/// Encodes the `prov.state` notification the device sends on `B004` after every state change.
pub fn encode_prov_state(state: ProvisionState, out: &mut [u8]) -> Result<usize, ProtocolError> {
    let mut buf: String<96> = String::new();
    write!(buf, r#"{{"jsonrpc":"2.0","method":"prov.state","params":{{"state":"{}"}}}}"#, state_name(state))
        .map_err(|_| ProtocolError::BufferTooSmall)?;
    write_framed_json(OPCODE_PROVISION, buf.as_bytes(), out)
}

impl ProtocolDispatcher {
    /// Encodes a `charm.tap_event` notification; `timestamp_ms` counts from boot.
    pub fn encode_tap_event(tap: TapType, timestamp_ms: u64, out: &mut [u8]) -> Result<usize, ProtocolError> {
        let kind = match tap {
            TapType::SingleTap => "single_tap",
            TapType::DoubleTap => "double_tap",
            TapType::LongPress => "long_press",
        };
        let mut buf: String<128> = String::new();
        write!(
            buf,
            r#"{{"jsonrpc":"2.0","method":"charm.tap_event","params":{{"type":"{}","timestamp_ms":{}}}}}"#,
            kind, timestamp_ms
        )
        .map_err(|_| ProtocolError::BufferTooSmall)?;
        write_framed_json(OPCODE_CONTROL, buf.as_bytes(), out)
    }

    /// Encodes a `charm.low_battery` notification frame.
    pub fn encode_low_battery_alert(battery_pct: u8, voltage_mv: u16, out: &mut [u8]) -> Result<usize, ProtocolError> {
        let mut buf: String<256> = String::new();
        write!(
            buf,
            concat!(
                r#"{{"jsonrpc":"2.0","method":"charm.low_battery","#,
                r#""params":{{"battery_pct":{},"voltage_mv":{}}}}}"#
            ),
            battery_pct, voltage_mv
        )
        .map_err(|_| ProtocolError::BufferTooSmall)?;
        write_framed_json(OPCODE_CONTROL, buf.as_bytes(), out)
    }
}

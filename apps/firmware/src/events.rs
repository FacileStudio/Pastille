use crate::protocol::{write_framed_json, ProtocolDispatcher, ProtocolError, OPCODE_CONTROL};
use core::fmt::Write;
use heapless::String;

impl ProtocolDispatcher {
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

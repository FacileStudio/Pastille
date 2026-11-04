use crate::avatar::AvatarRenderer;
use crate::tools::{execute_tool, write_tools_list};
use core::fmt::Write;
use heapless::String;

pub const OPCODE_AUDIO: u8 = 0x01;
pub const OPCODE_CONTROL: u8 = 0x02;

pub const FLAG_END_OF_UTTERANCE: u8 = 0x01;
pub const FLAG_VOICE_ACTIVITY: u8 = 0x02;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolError {
    EmptyPacket,
    InvalidOpcode(u8),
    TruncatedHeader,
    BufferTooSmall,
    InvalidUtf8,
    JsonParseError,
    UnknownMethod,
    UnknownTool,
    InvalidArguments,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BohsFrame<'a> {
    Audio {
        flags: u8,
        seq: u16,
        payload: &'a [u8],
    },
    Control {
        json: &'a str,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapType {
    SingleTap,
    DoubleTap,
    LongPress,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeviceTelemetry {
    pub battery_pct: u8,
    pub is_charging: bool,
    pub rssi_dbm: i8,
    pub temp_c: f32,
}

pub struct ProtocolDispatcher {
    pub telemetry: DeviceTelemetry,
    pub last_haptic_pattern: String<16>,
    pub last_haptic_duration: u32,
    pub last_haptic_intensity: u8,
}

impl ProtocolDispatcher {
    pub const fn new() -> Self {
        Self {
            telemetry: DeviceTelemetry {
                battery_pct: 87,
                is_charging: false,
                rssi_dbm: -54,
                temp_c: 31.8,
            },
            last_haptic_pattern: String::new(),
            last_haptic_duration: 0,
            last_haptic_intensity: 0,
        }
    }

    pub fn decode_packet<'a>(&self, packet: &'a [u8]) -> Result<BohsFrame<'a>, ProtocolError> {
        if packet.is_empty() {
            return Err(ProtocolError::EmptyPacket);
        }
        match packet[0] {
            OPCODE_AUDIO => {
                if packet.len() < 4 {
                    return Err(ProtocolError::TruncatedHeader);
                }
                let seq = u16::from_be_bytes([packet[2], packet[3]]);
                Ok(BohsFrame::Audio {
                    flags: packet[1],
                    seq,
                    payload: &packet[4..],
                })
            }
            OPCODE_CONTROL => {
                let json =
                    core::str::from_utf8(&packet[1..]).map_err(|_| ProtocolError::InvalidUtf8)?;
                Ok(BohsFrame::Control { json })
            }
            other => Err(ProtocolError::InvalidOpcode(other)),
        }
    }

    pub fn encode_audio_frame(
        flags: u8,
        seq: u16,
        payload: &[u8],
        out: &mut [u8],
    ) -> Result<usize, ProtocolError> {
        let total_len = 4 + payload.len();
        if out.len() < total_len {
            return Err(ProtocolError::BufferTooSmall);
        }
        out[0] = OPCODE_AUDIO;
        out[1] = flags;
        let seq_bytes = seq.to_be_bytes();
        out[2] = seq_bytes[0];
        out[3] = seq_bytes[1];
        out[4..total_len].copy_from_slice(payload);
        Ok(total_len)
    }

    pub fn encode_tap_event(
        tap: TapType,
        timestamp_ms: u64,
        out: &mut [u8],
    ) -> Result<usize, ProtocolError> {
        let kind = match tap {
            TapType::SingleTap => "single_tap",
            TapType::DoubleTap => "double_tap",
            TapType::LongPress => "long_press",
        };
        let mut buf: String<256> = String::new();
        write!(
            buf,
            concat!(
                r#"{{"jsonrpc":"2.0","method":"charm.tap_event","#,
                r#""params":{{"type":"{}","timestamp_ms":{}}}}}"#
            ),
            kind, timestamp_ms
        )
        .map_err(|_| ProtocolError::BufferTooSmall)?;
        write_control_frame(buf.as_bytes(), out)
    }

    pub fn encode_low_battery_alert(
        battery_pct: u8,
        voltage_mv: u16,
        out: &mut [u8],
    ) -> Result<usize, ProtocolError> {
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
        write_control_frame(buf.as_bytes(), out)
    }

    pub fn dispatch_json_rpc(
        &mut self,
        json_str: &str,
        avatar: &mut AvatarRenderer,
        out: &mut [u8],
    ) -> Result<usize, ProtocolError> {
        let value: serde_json::Value =
            serde_json::from_str(json_str).map_err(|_| ProtocolError::JsonParseError)?;
        let id = value.get("id").cloned().unwrap_or(serde_json::Value::Null);
        let method = value
            .get("method")
            .and_then(|v| v.as_str())
            .ok_or(ProtocolError::UnknownMethod)?;

        let mut heap_str: String<2048> = String::new();
        match method {
            "tools/list" => write_tools_list(&id, &mut heap_str)?,
            "tools/call" => {
                let params = value.get("params").ok_or(ProtocolError::InvalidArguments)?;
                execute_tool(self, params, &id, avatar, &mut heap_str)?;
            }
            _ => return Err(ProtocolError::UnknownMethod),
        }
        write_control_frame(heap_str.as_bytes(), out)
    }
}

fn write_control_frame(payload: &[u8], out: &mut [u8]) -> Result<usize, ProtocolError> {
    let total_len = 1 + payload.len();
    if out.len() < total_len {
        return Err(ProtocolError::BufferTooSmall);
    }
    out[0] = OPCODE_CONTROL;
    out[1..total_len].copy_from_slice(payload);
    Ok(total_len)
}

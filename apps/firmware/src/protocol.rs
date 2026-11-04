use crate::audio::BYTES_PER_FRAME;
use crate::avatar::AvatarRenderer;
use crate::haptic::HapticPulse;
use crate::rpc::{parse_request, write_error, Incoming};
use crate::tools::{execute_tool, write_tools_list};
use crate::MAX_PACKET_BYTES;
use heapless::String;
use serde_json::Value;

pub const OPCODE_AUDIO: u8 = 0x01;
pub const OPCODE_CONTROL: u8 = 0x02;
pub const OPCODE_PROVISION: u8 = 0x03;

pub const FLAG_END_OF_UTTERANCE: u8 = 0x01;
pub const FLAG_VOICE_ACTIVITY: u8 = 0x02;
/// Flag bits 2-7, which the spec reserves and requires to be zero.
pub const FLAG_RESERVED_MASK: u8 = !(FLAG_END_OF_UTTERANCE | FLAG_VOICE_ACTIVITY);
/// Length of an audio frame header: opcode, flags and the u16 sequence number.
pub const AUDIO_HEADER_BYTES: usize = 4;

/// Scratch buffer for one JSON-RPC message; it fills a frame once the opcode byte is added.
pub type Body = String<{ MAX_PACKET_BYTES - 1 }>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolError {
    EmptyPacket,
    InvalidOpcode(u8),
    TruncatedHeader,
    BufferTooSmall,
    InvalidUtf8,
    JsonParseError,
    InvalidRequest,
    UnknownMethod,
    UnknownTool,
    InvalidArguments,
    Unauthorized,
    InvalidFrameLength,
    ReservedFlags,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BohsFrame<'a> {
    Audio { flags: u8, seq: u16, payload: &'a [u8] },
    Control { json: &'a str },
    Provision { json: &'a str },
}

/// Latest sensor readings; `None` means the device has no source for that value.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DeviceTelemetry {
    pub battery_pct: u8,
    pub is_charging: bool,
    pub rssi_dbm: Option<i8>,
    pub temp_c: Option<f32>,
}

/// Reverse MCP server state: the telemetry it reports and the haptic pulse waiting for the motor.
#[derive(Default)]
pub struct ProtocolDispatcher {
    pub telemetry: DeviceTelemetry,
    pub haptic: Option<HapticPulse>,
}

impl ProtocolDispatcher {
    pub const fn new() -> Self {
        Self {
            telemetry: DeviceTelemetry { battery_pct: 0, is_charging: false, rssi_dbm: None, temp_c: None },
            haptic: None,
        }
    }

    /// Splits a frame into its opcode view. Audio frames must carry exactly 640 payload bytes
    /// and zero reserved flag bits; frames over 1024 bytes are rejected.
    pub fn decode_packet<'a>(&self, packet: &'a [u8]) -> Result<BohsFrame<'a>, ProtocolError> {
        let (&opcode, _) = packet.split_first().ok_or(ProtocolError::EmptyPacket)?;
        if packet.len() > MAX_PACKET_BYTES {
            return Err(ProtocolError::InvalidFrameLength);
        }
        match opcode {
            OPCODE_AUDIO => decode_audio(packet),
            OPCODE_CONTROL => Ok(BohsFrame::Control { json: json_body(packet)? }),
            OPCODE_PROVISION => Ok(BohsFrame::Provision { json: json_body(packet)? }),
            other => Err(ProtocolError::InvalidOpcode(other)),
        }
    }

    pub fn encode_audio_frame(flags: u8, seq: u16, payload: &[u8], out: &mut [u8]) -> Result<usize, ProtocolError> {
        let total_len = AUDIO_HEADER_BYTES + payload.len();
        if out.len() < total_len {
            return Err(ProtocolError::BufferTooSmall);
        }
        let [hi, lo] = seq.to_be_bytes();
        out[..AUDIO_HEADER_BYTES].copy_from_slice(&[OPCODE_AUDIO, flags, hi, lo]);
        out[AUDIO_HEADER_BYTES..total_len].copy_from_slice(payload);
        Ok(total_len)
    }

    /// Answers one `0x02` message and writes the response frame into `out`.
    /// Every request with an `id` gets a result or a JSON-RPC error; notifications get nothing.
    pub fn dispatch_json_rpc(&mut self, json_str: &str, avatar: &mut AvatarRenderer, out: &mut [u8]) -> Option<usize> {
        let mut body = Body::new();
        let (id, result) = match parse_request(json_str) {
            Incoming::Notification => return None,
            Incoming::Invalid(err) => (Value::Null, Err(err)),
            Incoming::Call { value, id } => {
                let result = self.run_method(&value, &id, avatar, &mut body);
                (id, result)
            }
        };
        if let Err(err) = result {
            body.clear();
            write_error(&id, err, &mut body).ok()?;
        }
        write_framed_json(OPCODE_CONTROL, body.as_bytes(), out).ok()
    }

    fn run_method(
        &mut self,
        value: &Value,
        id: &Value,
        avatar: &mut AvatarRenderer,
        body: &mut Body,
    ) -> Result<(), ProtocolError> {
        match value.get("method").and_then(Value::as_str) {
            Some("tools/list") => write_tools_list(id, body),
            Some("tools/call") => {
                let params = value.get("params").ok_or(ProtocolError::InvalidArguments)?;
                execute_tool(self, params, id, avatar, body)
            }
            _ => Err(ProtocolError::UnknownMethod),
        }
    }
}

fn decode_audio(packet: &[u8]) -> Result<BohsFrame<'_>, ProtocolError> {
    let Some(&[_, flags, hi, lo]) = packet.first_chunk::<AUDIO_HEADER_BYTES>() else {
        return Err(ProtocolError::TruncatedHeader);
    };
    if packet.len() != AUDIO_HEADER_BYTES + BYTES_PER_FRAME {
        return Err(ProtocolError::InvalidFrameLength);
    }
    if flags & FLAG_RESERVED_MASK != 0 {
        return Err(ProtocolError::ReservedFlags);
    }
    Ok(BohsFrame::Audio { flags, seq: u16::from_be_bytes([hi, lo]), payload: &packet[AUDIO_HEADER_BYTES..] })
}

fn json_body(packet: &[u8]) -> Result<&str, ProtocolError> {
    core::str::from_utf8(&packet[1..]).map_err(|_| ProtocolError::InvalidUtf8)
}

/// Writes a one-byte opcode followed by a JSON payload, shared by `0x02` and `0x03` frames.
pub fn write_framed_json(opcode: u8, payload: &[u8], out: &mut [u8]) -> Result<usize, ProtocolError> {
    let total_len = 1 + payload.len();
    if out.len() < total_len {
        return Err(ProtocolError::BufferTooSmall);
    }
    out[0] = opcode;
    out[1..total_len].copy_from_slice(payload);
    Ok(total_len)
}

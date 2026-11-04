#![allow(dead_code)]

use crate::avatar::{AvatarEmotion, AvatarRenderer, AvatarState};
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
    PayloadTooLarge,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeviceTelemetry {
    pub battery_pct: u8,
    pub is_charging: bool,
    pub rssi_dbm: i8,
    pub temp_c: f32,
}

impl Default for DeviceTelemetry {
    fn default() -> Self {
        Self {
            battery_pct: 100,
            is_charging: false,
            rssi_dbm: -50,
            temp_c: 25.0,
        }
    }
}

pub struct ProtocolDispatcher {
    telemetry: DeviceTelemetry,
    last_haptic_pattern: String<16>,
    last_haptic_duration: u32,
    last_haptic_intensity: u8,
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

    pub fn set_telemetry(&mut self, telemetry: DeviceTelemetry) {
        self.telemetry = telemetry;
    }

    pub fn telemetry(&self) -> DeviceTelemetry {
        self.telemetry
    }

    pub fn last_haptic(&self) -> (&str, u32, u8) {
        (
            self.last_haptic_pattern.as_str(),
            self.last_haptic_duration,
            self.last_haptic_intensity,
        )
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
                let flags = packet[1];
                let seq = u16::from_be_bytes([packet[2], packet[3]]);
                let payload = &packet[4..];
                Ok(BohsFrame::Audio {
                    flags,
                    seq,
                    payload,
                })
            }
            OPCODE_CONTROL => {
                let json_slice = &packet[1..];
                let json = core::str::from_utf8(json_slice).map_err(|_| ProtocolError::InvalidUtf8)?;
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

        let mut heap_str: heapless::String<2048> = heapless::String::new();

        match method {
            "tools/list" => {
                self.build_tools_list_response(&id, &mut heap_str)?;
            }
            "tools/call" => {
                let params = value.get("params").ok_or(ProtocolError::InvalidArguments)?;
                let tool_name = params
                    .get("name")
                    .and_then(|v| v.as_str())
                    .ok_or(ProtocolError::InvalidArguments)?;
                let arguments = params.get("arguments").unwrap_or(&serde_json::Value::Null);

                self.execute_tool(tool_name, arguments, &id, avatar, &mut heap_str)?;
            }
            _ => {
                return Err(ProtocolError::UnknownMethod);
            }
        }

        let resp_bytes = heap_str.as_bytes();
        let total_len = 1 + resp_bytes.len();
        if out.len() < total_len {
            return Err(ProtocolError::BufferTooSmall);
        }

        out[0] = OPCODE_CONTROL;
        out[1..total_len].copy_from_slice(resp_bytes);
        Ok(total_len)
    }

    fn execute_tool(
        &mut self,
        tool_name: &str,
        arguments: &serde_json::Value,
        id: &serde_json::Value,
        avatar: &mut AvatarRenderer,
        out: &mut heapless::String<2048>,
    ) -> Result<(), ProtocolError> {
        match tool_name {
            "charm.display_avatar" => {
                let state_str = arguments
                    .get("state")
                    .and_then(|v| v.as_str())
                    .unwrap_or("idle");

                let new_state = match state_str {
                    "idle" => AvatarState::Idle,
                    "listening" => AvatarState::Listening,
                    "thinking" => AvatarState::Thinking,
                    "speaking" => AvatarState::Speaking,
                    "error" => AvatarState::Error,
                    _ => AvatarState::Idle,
                };
                avatar.set_state(new_state);

                let emotion_str = arguments
                    .get("emotion")
                    .and_then(|v| v.as_str())
                    .unwrap_or("neutral");

                let new_emotion = match emotion_str {
                    "happy" => AvatarEmotion::Happy,
                    "focused" => AvatarEmotion::Focused,
                    "calm" => AvatarEmotion::Calm,
                    "curious" => AvatarEmotion::Curious,
                    "warn" => AvatarEmotion::Warn,
                    _ => AvatarEmotion::Neutral,
                };
                avatar.set_emotion(new_emotion);

                write!(
                    out,
                    r#"{{"jsonrpc":"2.0","id":{},"result":{{"content":[{{"type":"text","text":"Avatar state updated to {} ({})"}}],"isError":false}}}}"#,
                    id, state_str, emotion_str
                )
                .map_err(|_| ProtocolError::BufferTooSmall)?;
                Ok(())
            }
            "charm.haptic_pulse" => {
                let pattern = arguments
                    .get("pattern")
                    .and_then(|v| v.as_str())
                    .unwrap_or("tap");
                let duration_ms = arguments
                    .get("duration_ms")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(50) as u32;
                let intensity = arguments
                    .get("intensity")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(80) as u8;

                self.last_haptic_pattern.clear();
                let _ = self.last_haptic_pattern.push_str(pattern);
                self.last_haptic_duration = duration_ms;
                self.last_haptic_intensity = intensity;

                write!(
                    out,
                    r#"{{"jsonrpc":"2.0","id":{},"result":{{"content":[{{"type":"text","text":"Haptic pulse dispatched successfully"}}],"isError":false}}}}"#,
                    id
                )
                .map_err(|_| ProtocolError::BufferTooSmall)?;
                Ok(())
            }
            "charm.get_telemetry" => {
                let charging_str = if self.telemetry.is_charging {
                    "true"
                } else {
                    "false"
                };
                write!(
                    out,
                    r#"{{"jsonrpc":"2.0","id":{},"result":{{"content":[{{"type":"text","text":"{{\"battery_pct\":{},\"is_charging\":{},\"rssi_dbm\":{},\"temp_c\":{:.1}}}"}}],"isError":false}}}}"#,
                    id,
                    self.telemetry.battery_pct,
                    charging_str,
                    self.telemetry.rssi_dbm,
                    self.telemetry.temp_c
                )
                .map_err(|_| ProtocolError::BufferTooSmall)?;
                Ok(())
            }
            _ => Err(ProtocolError::UnknownTool),
        }
    }

    fn build_tools_list_response(
        &self,
        id: &serde_json::Value,
        out: &mut heapless::String<2048>,
    ) -> Result<(), ProtocolError> {
        write!(
            out,
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"tools":[{{"name":"charm.display_avatar","description":"Updates the face expression on the circular LCD display","inputSchema":{{"type":"object","properties":{{"state":{{"type":"string","enum":["idle","listening","thinking","speaking","error"]}},"emotion":{{"type":"string","enum":["neutral","happy","focused","calm","curious","warn"]}}}},"required":["state"]}}}},{{"name":"charm.haptic_pulse","description":"Triggers a localized vibration pattern on the wearable motor","inputSchema":{{"type":"object","properties":{{"pattern":{{"type":"string","enum":["tap","double_tap","alert","error"]}},"duration_ms":{{"type":"integer","minimum":10,"maximum":500}},"intensity":{{"type":"integer","minimum":1,"maximum":100}}}},"required":["pattern"]}}}},{{"name":"charm.get_telemetry","description":"Reads physical battery status, RSSI, and temperature from the peripheral","inputSchema":{{"type":"object","properties":{{}}}}}}]}}}}"#,
            id
        )
        .map_err(|_| ProtocolError::BufferTooSmall)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_encoding_decoding() {
        let payload = [0xAA, 0xBB, 0xCC, 0xDD];
        let mut buf = [0u8; 16];
        let len = ProtocolDispatcher::encode_audio_frame(
            FLAG_VOICE_ACTIVITY,
            0x1234,
            &payload,
            &mut buf,
        )
        .unwrap();

        assert_eq!(len, 8);
        assert_eq!(buf[0], OPCODE_AUDIO);
        assert_eq!(buf[1], FLAG_VOICE_ACTIVITY);
        assert_eq!(buf[2], 0x12);
        assert_eq!(buf[3], 0x34);
        assert_eq!(&buf[4..8], &payload);

        let dispatcher = ProtocolDispatcher::new();
        let frame = dispatcher.decode_packet(&buf[..len]).unwrap();
        match frame {
            BohsFrame::Audio { flags, seq, payload } => {
                assert_eq!(flags, FLAG_VOICE_ACTIVITY);
                assert_eq!(seq, 0x1234);
                assert_eq!(payload, &[0xAA, 0xBB, 0xCC, 0xDD]);
            }
            _ => panic!("Expected audio frame"),
        }
    }

    #[test]
    fn test_control_tools_list() {
        let mut dispatcher = ProtocolDispatcher::new();
        let mut avatar = AvatarRenderer::new();
        let json_req = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}"#;

        let mut out = [0u8; 2048];
        let len = dispatcher
            .dispatch_json_rpc(json_req, &mut avatar, &mut out)
            .unwrap();

        assert_eq!(out[0], OPCODE_CONTROL);
        let resp_str = core::str::from_utf8(&out[1..len]).unwrap();
        assert!(resp_str.contains("charm.display_avatar"));
        assert!(resp_str.contains("charm.haptic_pulse"));
        assert!(resp_str.contains("charm.get_telemetry"));
    }

    #[test]
    fn test_control_display_avatar() {
        let mut dispatcher = ProtocolDispatcher::new();
        let mut avatar = AvatarRenderer::new();
        let json_req = r#"{"jsonrpc":"2.0","id":42,"method":"tools/call","params":{"name":"charm.display_avatar","arguments":{"state":"speaking","emotion":"happy"}}}"#;

        let mut out = [0u8; 1024];
        let len = dispatcher
            .dispatch_json_rpc(json_req, &mut avatar, &mut out)
            .unwrap();

        assert_eq!(avatar.state(), AvatarState::Speaking);
        assert_eq!(avatar.emotion(), AvatarEmotion::Happy);

        let resp_str = core::str::from_utf8(&out[1..len]).unwrap();
        assert!(resp_str.contains("Avatar state updated to speaking (happy)"));
    }

    #[test]
    fn test_control_haptic_and_telemetry() {
        let mut dispatcher = ProtocolDispatcher::new();
        let mut avatar = AvatarRenderer::new();

        let haptic_req = r#"{"jsonrpc":"2.0","id":43,"method":"tools/call","params":{"name":"charm.haptic_pulse","arguments":{"pattern":"double_tap","duration_ms":60,"intensity":80}}}"#;
        let mut out = [0u8; 1024];
        dispatcher
            .dispatch_json_rpc(haptic_req, &mut avatar, &mut out)
            .unwrap();

        let (pat, dur, int) = dispatcher.last_haptic();
        assert_eq!(pat, "double_tap");
        assert_eq!(dur, 60);
        assert_eq!(int, 80);

        let telem_req = r#"{"jsonrpc":"2.0","id":44,"method":"tools/call","params":{"name":"charm.get_telemetry","arguments":{}}}"#;
        let len = dispatcher
            .dispatch_json_rpc(telem_req, &mut avatar, &mut out)
            .unwrap();
        let resp_str = core::str::from_utf8(&out[1..len]).unwrap();
        assert!(resp_str.contains("battery_pct"));
        assert!(resp_str.contains("87"));
        assert!(resp_str.contains("rssi_dbm"));
    }
}

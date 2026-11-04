use crate::ws::WsError;

pub const OP_BINARY: u8 = 0x2;
pub const OP_CLOSE: u8 = 0x8;
pub const OP_PING: u8 = 0x9;
pub const OP_PONG: u8 = 0xA;
/// Largest frame header a client writes: 2 bytes, a 16-bit length and the 4-byte mask.
pub const MAX_CLIENT_HEADER: usize = 8;

/// Header of one frame received from the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameHeader {
    pub fin: bool,
    pub opcode: u8,
    pub header_len: usize,
    pub payload_len: usize,
}

/// Writes one masked client frame with FIN set; payloads are at most 65535 bytes.
pub fn encode_frame(opcode: u8, payload: &[u8], mask: [u8; 4], out: &mut [u8]) -> Result<usize, WsError> {
    let len = payload.len();
    let header_len = if len < 126 { 6 } else { 8 };
    let total = header_len + len;
    if len > usize::from(u16::MAX) || out.len() < total {
        return Err(WsError::BufferTooSmall);
    }
    out[0] = 0x80 | opcode;
    if len < 126 {
        out[1] = 0x80 | len as u8;
    } else {
        out[1] = 0x80 | 126;
        out[2..4].copy_from_slice(&(len as u16).to_be_bytes());
    }
    out[header_len - 4..header_len].copy_from_slice(&mask);
    for (i, (dst, src)) in out[header_len..total].iter_mut().zip(payload).enumerate() {
        *dst = src ^ mask[i % 4];
    }
    Ok(total)
}

/// Parses a server frame header; `Ok(None)` while more bytes are needed.
/// Servers never mask, and frames larger than `max_payload` are a protocol error.
pub fn parse_header(buf: &[u8], max_payload: usize) -> Result<Option<FrameHeader>, WsError> {
    let (Some(&b0), Some(&b1)) = (buf.first(), buf.get(1)) else {
        return Ok(None);
    };
    if b1 & 0x80 != 0 || b0 & 0x70 != 0 {
        return Err(WsError::Protocol);
    }
    let (header_len, payload_len) = match b1 & 0x7F {
        126 => (4, buf.get(2..4).map(|b| usize::from(u16::from_be_bytes([b[0], b[1]])))),
        127 => (10, buf.get(2..10).map(|b| u64::from_be_bytes(b.try_into().unwrap_or([0xFF; 8])) as usize)),
        n => (2, Some(usize::from(n))),
    };
    let Some(payload_len) = payload_len else {
        return Ok(None);
    };
    if payload_len > max_payload {
        return Err(WsError::Protocol);
    }
    Ok(Some(FrameHeader { fin: b0 & 0x80 != 0, opcode: b0 & 0x0F, header_len, payload_len }))
}

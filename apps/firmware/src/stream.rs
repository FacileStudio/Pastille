use crate::protocol::ProtocolError;
use crate::MAX_PACKET_BYTES;

pub const LENGTH_PREFIX_BYTES: usize = 2;

/// Writes `frame` prefixed with its u16 big-endian length, as sent over BLE and USB byte streams.
pub fn encode_stream_frame(frame: &[u8], out: &mut [u8]) -> Result<usize, ProtocolError> {
    if frame.is_empty() || frame.len() > MAX_PACKET_BYTES {
        return Err(ProtocolError::InvalidFrameLength);
    }
    let total = LENGTH_PREFIX_BYTES + frame.len();
    if out.len() < total {
        return Err(ProtocolError::BufferTooSmall);
    }
    out[..LENGTH_PREFIX_BYTES].copy_from_slice(&(frame.len() as u16).to_be_bytes());
    out[LENGTH_PREFIX_BYTES..total].copy_from_slice(frame);
    Ok(total)
}

/// Rebuilds length-prefixed bohs.v1 frames from arbitrary stream chunks such as BLE notifications.
pub struct StreamReassembler {
    buf: [u8; LENGTH_PREFIX_BYTES + MAX_PACKET_BYTES],
    filled: usize,
}

impl StreamReassembler {
    pub const fn new() -> Self {
        Self {
            buf: [0; LENGTH_PREFIX_BYTES + MAX_PACKET_BYTES],
            filled: 0,
        }
    }

    /// Consumes bytes from the front of `input` and returns a frame once one is complete.
    /// Call it again while `input` is not empty. A zero or oversized length means the stream is
    /// broken: the buffer and the rest of `input` are discarded, and the caller drops the BLE link
    /// or, on USB, resumes with the next chunk the host writes after reopening the port.
    pub fn push(&mut self, input: &mut &[u8]) -> Result<Option<&[u8]>, ProtocolError> {
        let take = self.missing().min(input.len());
        self.buf[self.filled..self.filled + take].copy_from_slice(&input[..take]);
        self.filled += take;
        *input = &input[take..];
        let header_done = self.filled >= LENGTH_PREFIX_BYTES;
        if header_done && !(1..=MAX_PACKET_BYTES).contains(&self.frame_len()) {
            self.filled = 0;
            *input = &[];
            return Err(ProtocolError::InvalidFrameLength);
        }
        if !header_done || self.filled < LENGTH_PREFIX_BYTES + self.frame_len() {
            return Ok(None);
        }
        let end = self.filled;
        self.filled = 0;
        Ok(Some(&self.buf[LENGTH_PREFIX_BYTES..end]))
    }

    fn missing(&self) -> usize {
        if self.filled < LENGTH_PREFIX_BYTES {
            return LENGTH_PREFIX_BYTES - self.filled;
        }
        LENGTH_PREFIX_BYTES + self.frame_len() - self.filled
    }

    fn frame_len(&self) -> usize {
        u16::from_be_bytes([self.buf[0], self.buf[1]]) as usize
    }
}

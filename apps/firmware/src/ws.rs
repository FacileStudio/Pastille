use core::fmt::Write;
use sha1::{Digest, Sha1};

/// The subprotocol every BOHS-01 session negotiates.
pub const SUBPROTOCOL: &str = "bohs.v1";
const WS_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Why a WebSocket session could not open or had to close.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsError {
    BadUrl,
    BufferTooSmall,
    Handshake,
    Protocol,
}

/// A `wss://` gateway URL split into the parts the TCP, TLS and HTTP layers need.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GatewayUrl<'a> {
    pub host: &'a str,
    pub port: u16,
    pub path: &'a str,
}

impl<'a> GatewayUrl<'a> {
    /// Parses `wss://host[:port][/path]`; the port defaults to 443 and the path to `/`.
    pub fn parse(url: &'a str) -> Result<Self, WsError> {
        let rest = url.strip_prefix("wss://").ok_or(WsError::BadUrl)?;
        let split = rest.find(['/', '?']).unwrap_or(rest.len());
        let (authority, path) = rest.split_at(split);
        let (host, port) = match authority.rsplit_once(':') {
            Some((host, port)) => (host, port.parse().map_err(|_| WsError::BadUrl)?),
            None => (authority, 443),
        };
        let valid_host = !host.is_empty() && host.bytes().all(|b| b.is_ascii_alphanumeric() || b"-.".contains(&b));
        if !valid_host || port == 0 {
            return Err(WsError::BadUrl);
        }
        Ok(Self { host, port, path: if path.is_empty() { "/" } else { path } })
    }
}

/// Standard base64 with padding.
pub fn base64<const N: usize>(bytes: &[u8]) -> heapless::String<N> {
    let mut out = heapless::String::new();
    for chunk in bytes.chunks(3) {
        let n = chunk.iter().enumerate().fold(0u32, |acc, (i, &b)| acc | u32::from(b) << (16 - 8 * i));
        for i in 0..4 {
            let c = if i <= chunk.len() { B64[(n >> (18 - 6 * i) & 63) as usize] as char } else { '=' };
            let _ = out.push(c);
        }
    }
    out
}

/// The `Sec-WebSocket-Accept` value a server must return for `key`.
pub fn accept_key(key: &str) -> heapless::String<28> {
    let digest = Sha1::new().chain_update(key.as_bytes()).chain_update(WS_GUID.as_bytes()).finalize();
    base64(&digest)
}

/// Writes the HTTP upgrade request with the `bohs.v1` subprotocol and the device token.
pub fn write_handshake<W: Write>(url: &GatewayUrl, key: &str, token: &str, out: &mut W) -> Result<(), WsError> {
    write!(
        out,
        concat!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n",
            "Sec-WebSocket-Key: {}\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Protocol: {}\r\n",
            "Authorization: Bearer {}\r\n\r\n"
        ),
        url.path, url.host, key, SUBPROTOCOL, token
    )
    .map_err(|_| WsError::BufferTooSmall)
}

/// Checks the upgrade response and returns its length, or `None` while the headers are incomplete.
pub fn check_response(response: &[u8], key: &str) -> Result<Option<usize>, WsError> {
    let Some(end) = response.windows(4).position(|w| w == b"\r\n\r\n") else {
        return Ok(None);
    };
    let head = core::str::from_utf8(&response[..end]).map_err(|_| WsError::Handshake)?;
    let mut lines = head.split("\r\n");
    let status_ok = lines.next().is_some_and(|line| line.split(' ').nth(1) == Some("101"));
    let expected = accept_key(key);
    let (mut accepted, mut protocol) = (false, false);
    for (name, value) in lines.filter_map(|line| line.split_once(':')) {
        let value = value.trim();
        accepted |= name.eq_ignore_ascii_case("sec-websocket-accept") && value == expected.as_str();
        protocol |= name.eq_ignore_ascii_case("sec-websocket-protocol") && value == SUBPROTOCOL;
    }
    if status_ok && accepted && protocol {
        Ok(Some(end + 4))
    } else {
        Err(WsError::Handshake)
    }
}

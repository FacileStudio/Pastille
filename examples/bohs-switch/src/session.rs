use crate::config;
use crate::creds::Credentials;
use crate::rpc::{self, Switch};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use tungstenite::client::IntoClientRequest;
use tungstenite::handshake::client::Request;
use tungstenite::http::HeaderValue;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Error, HandshakeError, Message, WebSocket};

const OPCODE_CONTROL: u8 = 0x02;
const STATUS_REVOKED: u16 = 4401;

/// The gateway pings every 30 s; a socket silent for this long is dead.
const READ_TIMEOUT: Duration = Duration::from_secs(75);

/// The longest the connect, the TLS handshake or one write may take.
const STEP_TIMEOUT: Duration = Duration::from_secs(10);

/// How a gateway session ended.
pub enum End {
    /// The token is no longer valid: the device must pair again.
    Revoked,
    /// The link dropped; the caller reconnects after a pause.
    Dropped(String),
}

/// Opens the bohs.v1 socket and answers the gateway's requests until it closes.
pub fn run(creds: &Credentials, sw: &mut Switch, online: &mut dyn FnMut()) -> End {
    let mut socket = match connect(creds) {
        Ok(socket) => socket,
        Err(end) => return end,
    };
    online();
    loop {
        match socket.read() {
            Ok(Message::Binary(frame)) if frame.first() == Some(&OPCODE_CONTROL) => {
                let Some(reply) = rpc::handle(&frame[1..], sw) else {
                    continue;
                };
                let mut out = Vec::with_capacity(reply.len() + 1);
                out.push(OPCODE_CONTROL);
                out.extend_from_slice(&reply);
                if let Err(e) = socket.send(Message::binary(out)) {
                    return End::Dropped(e.to_string());
                }
            }
            Ok(Message::Close(Some(frame))) if u16::from(frame.code) == STATUS_REVOKED => return End::Revoked,
            Ok(Message::Close(_)) => return End::Dropped("the gateway closed the socket".into()),
            Ok(_) => {}
            Err(e) => return End::Dropped(e.to_string()),
        }
    }
}

type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

fn dial(request: &Request) -> Result<TcpStream, String> {
    let uri = request.uri();
    let host = uri.host().ok_or("the gateway socket URL has no host")?;
    let port = uri
        .port_u16()
        .unwrap_or(if uri.scheme_str() == Some("wss") { 443 } else { 80 });
    let addrs = (host.trim_matches(['[', ']']), port)
        .to_socket_addrs()
        .map_err(|e| format!("cannot resolve {host}: {e}"))?;
    let stream = addrs
        .into_iter()
        .find_map(|addr| TcpStream::connect_timeout(&addr, STEP_TIMEOUT).ok())
        .ok_or_else(|| format!("cannot connect to {host}:{port}"))?;
    stream
        .set_read_timeout(Some(STEP_TIMEOUT))
        .and_then(|()| stream.set_write_timeout(Some(STEP_TIMEOUT)))
        .map_err(|e| e.to_string())?;
    Ok(stream)
}

fn connect(creds: &Credentials) -> Result<Socket, End> {
    let mut request = creds
        .gateway_ws_url
        .as_str()
        .into_client_request()
        .map_err(|e| End::Dropped(e.to_string()))?;
    let plain = request.uri().scheme_str() == Some("ws");
    if plain && !request.uri().host().is_some_and(config::is_loopback) {
        return Err(End::Dropped(
            "refusing to send the token over a plain-text socket".into(),
        ));
    }
    let bearer = HeaderValue::from_str(&format!("Bearer {}", creds.device_token)).map_err(|_| End::Revoked)?;
    request.headers_mut().insert("Authorization", bearer);
    request
        .headers_mut()
        .insert("Sec-WebSocket-Protocol", HeaderValue::from_static("bohs.v1"));
    let stream = dial(&request).map_err(End::Dropped)?;
    let (socket, _) = tungstenite::client_tls(request, stream).map_err(|e| match e {
        HandshakeError::Failure(Error::Http(response)) if response.status() == 401 => End::Revoked,
        e => End::Dropped(e.to_string()),
    })?;
    let tcp = match socket.get_ref() {
        MaybeTlsStream::Plain(tcp) => tcp,
        MaybeTlsStream::Rustls(tls) => tls.get_ref(),
        _ => return Ok(socket),
    };
    let _ = tcp.set_read_timeout(Some(READ_TIMEOUT));
    Ok(socket)
}

use crate::creds::Credentials;
use crate::rpc::{self, Switch};
use std::net::TcpStream;
use std::time::Duration;
use tungstenite::client::IntoClientRequest;
use tungstenite::http::HeaderValue;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Error, Message, WebSocket};

const OPCODE_CONTROL: u8 = 0x02;
const STATUS_REVOKED: u16 = 4401;

/// The gateway pings every 30 s; a socket silent for this long is dead.
const READ_TIMEOUT: Duration = Duration::from_secs(75);

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

fn connect(creds: &Credentials) -> Result<Socket, End> {
    let mut request = creds
        .gateway_ws_url
        .as_str()
        .into_client_request()
        .map_err(|e| End::Dropped(e.to_string()))?;
    let bearer = HeaderValue::from_str(&format!("Bearer {}", creds.device_token)).map_err(|_| End::Revoked)?;
    request.headers_mut().insert("Authorization", bearer);
    request
        .headers_mut()
        .insert("Sec-WebSocket-Protocol", HeaderValue::from_static("bohs.v1"));
    let (socket, _) = tungstenite::connect(request).map_err(|e| match e {
        Error::Http(response) if response.status() == 401 => End::Revoked,
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

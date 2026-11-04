//! Wi-Fi direct transport: one `bohs.v1` WebSocket session over TLS 1.3 to the stored gateway URL,
//! reopened with exponential backoff.

use super::tls::GatewayProvider;
use super::wifi::{wants_wifi, MAX_BACKOFF, MIN_BACKOFF};
use super::wss_io::{read_loop, write_loop, Duplex};
use crate::ble::{ProvisionConfig, TransportKind};
use crate::link::{Inbound, CONFIG_WATCH, INBOUND_CHANNEL, OUTBOUND_WSS};
use crate::ws::{base64, check_response, write_handshake, GatewayUrl, WsError};
use embassy_futures::select::select;
use embassy_net::dns::DnsQueryType;
use embassy_net::tcp::TcpSocket;
use embassy_net::Stack;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Instant, Timer};
use embedded_io_async::{Read, Write};
use embedded_tls::{TlsConfig, TlsConnection, TlsContext};
use esp_hal::rng::Rng;

const TCP_BUFFER_BYTES: usize = 4096;
const TLS_READ_BYTES: usize = 16_640;
const TLS_WRITE_BYTES: usize = 4096;
/// A session that stayed up this long resets the backoff.
const STABLE_SESSION: Duration = Duration::from_secs(60);

/// Socket and TLS record buffers, kept in the task so they are allocated once.
pub struct Buffers {
    tcp_rx: [u8; TCP_BUFFER_BYTES],
    tcp_tx: [u8; TCP_BUFFER_BYTES],
    tls_read: [u8; TLS_READ_BYTES],
    tls_write: [u8; TLS_WRITE_BYTES],
}

/// Sends the upgrade request and returns how many bytes after the response belong to the first frames.
async fn upgrade<S: Read + Write>(
    tls: &mut S,
    url: &GatewayUrl<'_>,
    token: &str,
    buf: &mut [u8; 2048],
) -> Result<usize, WsError> {
    let mut nonce = [0u8; 16];
    Rng::new().read(&mut nonce);
    let key = base64::<24>(&nonce);
    let mut request: heapless::String<512> = heapless::String::new();
    write_handshake(url, &key, token, &mut request)?;
    tls.write_all(request.as_bytes()).await.map_err(|_| WsError::Handshake)?;
    tls.flush().await.map_err(|_| WsError::Handshake)?;
    let mut filled = 0;
    loop {
        let read = tls.read(&mut buf[filled..]).await.map_err(|_| WsError::Handshake)?;
        if read == 0 || filled + read == buf.len() {
            return Err(WsError::Handshake);
        }
        filled += read;
        if let Some(end) = check_response(&buf[..filled], &key)? {
            buf.copy_within(end..filled, 0);
            return Ok(filled - end);
        }
    }
}

async fn session(stack: Stack<'static>, cfg: &ProvisionConfig, bufs: &mut Buffers) -> Result<(), WsError> {
    let url = GatewayUrl::parse(&cfg.gateway_url)?;
    let address = *stack
        .dns_query(url.host, DnsQueryType::A)
        .await
        .map_err(|_| WsError::BadUrl)?
        .first()
        .ok_or(WsError::BadUrl)?;
    let mut socket = TcpSocket::new(stack, &mut bufs.tcp_rx, &mut bufs.tcp_tx);
    socket.set_timeout(Some(Duration::from_secs(90)));
    socket.connect((address, url.port)).await.map_err(|_| WsError::Handshake)?;
    let (rx, tx) = socket.split();
    let (rx, tx) = (Mutex::<NoopRawMutex, _>::new(rx), Mutex::<NoopRawMutex, _>::new(tx));
    let mut tls = TlsConnection::new(Duplex { rx: &rx, tx: &tx }, &mut bufs.tls_read, &mut bufs.tls_write);
    let config = TlsConfig::new().with_server_name(url.host).enable_rsa_signatures();
    tls.open(TlsContext::new(&config, GatewayProvider::new())).await.map_err(|_| WsError::Handshake)?;
    let mut frames = [0u8; 2048];
    let pending = upgrade(&mut tls, &url, &cfg.device_token, &mut frames).await?;
    while OUTBOUND_WSS.try_receive().is_ok() {}
    INBOUND_CHANNEL.send(Inbound::LinkUp(TransportKind::WifiDirect)).await;
    let (reader, writer) = tls.split();
    let pongs = embassy_sync::signal::Signal::new();
    select(read_loop(reader, &mut frames, pending, &pongs), write_loop(writer, &pongs)).await;
    Ok(())
}

/// Runs gateway sessions whenever the config selects Wi-Fi direct and the network is up.
#[embassy_executor::task]
pub async fn wss_task(stack: Stack<'static>) {
    let Some(mut configs) = CONFIG_WATCH.receiver() else { return };
    let mut bufs = Buffers {
        tcp_rx: [0; TCP_BUFFER_BYTES],
        tcp_tx: [0; TCP_BUFFER_BYTES],
        tls_read: [0; TLS_READ_BYTES],
        tls_write: [0; TLS_WRITE_BYTES],
    };
    let mut backoff = MIN_BACKOFF;
    loop {
        let cfg = configs.get().await;
        if !wants_wifi(&cfg) {
            let _ = configs.changed().await;
            continue;
        }
        stack.wait_config_up().await;
        let started = Instant::now();
        let _ = session(stack, &cfg, &mut bufs).await;
        INBOUND_CHANNEL.send(Inbound::LinkDown(TransportKind::WifiDirect)).await;
        if started.elapsed() >= STABLE_SESSION {
            backoff = MIN_BACKOFF;
        }
        Timer::after(backoff).await;
        backoff = (backoff * 2).min(MAX_BACKOFF);
    }
}

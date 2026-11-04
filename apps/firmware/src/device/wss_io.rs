//! The two halves of an open WebSocket session: frames from the gateway to the network task,
//! and queued frames from the network task to the gateway, with keepalive pings.

use crate::ble::TransportKind;
use crate::link::{Inbound, INBOUND_CHANNEL, OUTBOUND_WSS};
use crate::ws_frame::{encode_frame, parse_header, MAX_CLIENT_HEADER, OP_BINARY, OP_CLOSE, OP_PING, OP_PONG};
use crate::MAX_PACKET_BYTES;
use embassy_futures::select::{select3, Either3};
use embassy_net::tcp::{TcpReader, TcpWriter};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use embedded_io_async::{ErrorType, Read, Write};
use esp_hal::rng::Rng;

/// Largest control payload a ping may carry.
const CONTROL_BYTES: usize = 125;
const KEEPALIVE: Duration = Duration::from_secs(30);

pub type Pongs = Signal<NoopRawMutex, heapless::Vec<u8, CONTROL_BYTES>>;

/// A cloneable view of a split TCP socket, so the TLS reader and writer run concurrently.
#[derive(Clone, Copy)]
pub struct Duplex<'s, 'd> {
    pub rx: &'s Mutex<NoopRawMutex, TcpReader<'d>>,
    pub tx: &'s Mutex<NoopRawMutex, TcpWriter<'d>>,
}

impl ErrorType for Duplex<'_, '_> {
    type Error = embassy_net::tcp::Error;
}

impl Read for Duplex<'_, '_> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.rx.lock().await.read(buf).await
    }
}

impl Write for Duplex<'_, '_> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tx.lock().await.write(buf).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.tx.lock().await.flush().await
    }
}

/// Handles one complete frame; `false` ends the session.
async fn on_frame(opcode: u8, fin: bool, payload: &[u8], pongs: &Pongs) -> bool {
    match opcode {
        OP_BINARY if fin && !payload.is_empty() && payload.len() <= MAX_PACKET_BYTES => {
            let mut packet = [0u8; MAX_PACKET_BYTES];
            packet[..payload.len()].copy_from_slice(payload);
            INBOUND_CHANNEL.send(Inbound::Frame(TransportKind::WifiDirect, (packet, payload.len()))).await;
            true
        }
        OP_PING => {
            pongs.signal(heapless::Vec::from_slice(payload).unwrap_or_default());
            true
        }
        OP_PONG => true,
        _ => false,
    }
}

/// Reads frames until the gateway closes the session or breaks the protocol.
pub async fn read_loop<R: Read>(mut reader: R, buf: &mut [u8; 2048], mut filled: usize, pongs: &Pongs) {
    loop {
        match parse_header(&buf[..filled], MAX_PACKET_BYTES) {
            Err(_) => return,
            Ok(Some(h)) if filled >= h.header_len + h.payload_len => {
                let end = h.header_len + h.payload_len;
                if !on_frame(h.opcode, h.fin, &buf[h.header_len..end], pongs).await {
                    return;
                }
                buf.copy_within(end..filled, 0);
                filled -= end;
                continue;
            }
            Ok(_) => {}
        }
        match reader.read(&mut buf[filled..]).await {
            Ok(read) if read > 0 => filled += read,
            _ => return,
        }
    }
}

async fn send<W: Write>(writer: &mut W, opcode: u8, payload: &[u8]) -> bool {
    let mut wire = [0u8; MAX_CLIENT_HEADER + MAX_PACKET_BYTES];
    let Ok(len) = encode_frame(opcode, payload, Rng::new().random().to_le_bytes(), &mut wire) else {
        return true;
    };
    writer.write_all(&wire[..len]).await.is_ok() && writer.flush().await.is_ok()
}

/// Writes every queued frame as one binary message, answers pings and keeps the session alive.
pub async fn write_loop<W: Write>(mut writer: W, pongs: &Pongs) {
    loop {
        let sent = match select3(OUTBOUND_WSS.receive(), pongs.wait(), Timer::after(KEEPALIVE)).await {
            Either3::First((packet, len)) => send(&mut writer, OP_BINARY, &packet[..len]).await,
            Either3::Second(payload) => send(&mut writer, OP_PONG, &payload).await,
            Either3::Third(()) => send(&mut writer, OP_PING, &[]).await,
        };
        if !sent {
            let _ = send(&mut writer, OP_CLOSE, &[]).await;
            return;
        }
    }
}

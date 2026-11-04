//! One BLE connection: reassembles writes into frames for the network task, sends queued frames
//! as notifications or indications, and reports pairing and encryption to the provisioner.

use super::gatt::{Server, CHUNK_BYTES};
use crate::ble::TransportKind;
use crate::ble_link::info_record;
use crate::device::touch::PRESS_SIGNAL;
use crate::link::{
    device_info, Inbound, Overlay, StoreRequest, INBOUND_CHANNEL, OUTBOUND_BLE, OVERLAY_SIGNAL, STORE_CHANNEL,
};
use crate::protocol::{OPCODE_AUDIO, OPCODE_CONTROL, OPCODE_PROVISION};
use crate::stream::{encode_stream_frame, StreamReassembler, LENGTH_PREFIX_BYTES};
use crate::MAX_PACKET_BYTES;
use embassy_futures::select::{select, Either};
use embassy_time::{Duration, Timer};
use trouble_host::prelude::*;

/// How long the user has to confirm a numeric comparison code with a tap.
const CONFIRM_WINDOW: Duration = Duration::from_secs(30);

/// One reassembler per writable characteristic, each bound to the opcode it may carry.
struct Streams {
    audio: StreamReassembler,
    control: StreamReassembler,
    prov: StreamReassembler,
}

/// Serves one connection until the central disconnects.
pub async fn serve<C: Controller>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, DefaultPacketPool>,
    _stack: &Stack<'_, C, DefaultPacketPool>,
) {
    while OUTBOUND_BLE.try_receive().is_ok() {}
    INBOUND_CHANNEL.send(Inbound::BleConnected(conn.raw().handle().raw())).await;
    let _ = conn.raw().set_bondable(true);
    select(events(server, conn), writer(server, conn)).await;
    OVERLAY_SIGNAL.signal(Overlay::None);
    INBOUND_CHANNEL.send(Inbound::BleDisconnected).await;
}

async fn events(server: &Server<'_>, conn: &GattConnection<'_, '_, DefaultPacketPool>) {
    let mut streams =
        Streams { audio: StreamReassembler::new(), control: StreamReassembler::new(), prov: StreamReassembler::new() };
    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { .. } => return,
            GattConnectionEvent::PassKeyConfirm(key) => confirm(conn, key.value()).await,
            GattConnectionEvent::PassKeyDisplay(key) => OVERLAY_SIGNAL.signal(Overlay::Passkey(key.value())),
            GattConnectionEvent::PairingComplete { security_level, bond } => secured(security_level, bond).await,
            GattConnectionEvent::Encrypted { security_level, bond } => {
                let bonded = bond.is_some() || conn.raw().is_bonded_peer();
                INBOUND_CHANNEL.send(Inbound::BleSecured(security_level.encrypted() && bonded)).await;
            }
            GattConnectionEvent::PairingFailed(_) => INBOUND_CHANNEL.send(Inbound::BleSecured(false)).await,
            GattConnectionEvent::Gatt { event } => {
                if !gatt_event(server, conn, &mut streams, &event).await {
                    conn.raw().disconnect();
                }
                if let Ok(reply) = event.accept() {
                    reply.send().await;
                }
            }
            _ => {}
        }
    }
}

/// Shows the comparison code and confirms it when the user taps within the window.
async fn confirm(conn: &GattConnection<'_, '_, DefaultPacketPool>, code: u32) {
    OVERLAY_SIGNAL.signal(Overlay::Passkey(code));
    PRESS_SIGNAL.reset();
    let tapped = matches!(select(PRESS_SIGNAL.wait(), Timer::after(CONFIRM_WINDOW)).await, Either::First(()));
    let _ = if tapped { conn.pass_key_confirm() } else { conn.pass_key_cancel() };
    OVERLAY_SIGNAL.signal(Overlay::None);
}

async fn secured(level: SecurityLevel, bond: Option<BondInformation>) {
    if let Some(bond) = bond.as_ref().filter(|bond| bond.is_bonded) {
        let _ = STORE_CHANNEL.try_send(StoreRequest::Bond(super::bond::encode(bond)));
    }
    let bonded = bond.is_some_and(|bond| bond.is_bonded);
    INBOUND_CHANNEL.send(Inbound::BleSecured(level.encrypted() && bonded)).await;
}

/// Feeds a write into its stream and forwards complete frames; `false` means the stream broke.
async fn gatt_event(
    server: &Server<'_>,
    _conn: &GattConnection<'_, '_, DefaultPacketPool>,
    streams: &mut Streams,
    event: &GattEvent<'_, '_, DefaultPacketPool>,
) -> bool {
    let bohs = &server.bohs;
    let write = match event {
        GattEvent::Read(read) if read.handle() == bohs.info.handle => {
            let _ = server.set(&bohs.info, &info_record(&device_info()));
            return true;
        }
        GattEvent::Write(write) => write,
        _ => return true,
    };
    let handle = write.handle();
    let (stream, opcode) = match handle {
        h if h == bohs.audio.handle => (&mut streams.audio, OPCODE_AUDIO),
        h if h == bohs.control.handle => (&mut streams.control, OPCODE_CONTROL),
        h if h == bohs.prov.handle => (&mut streams.prov, OPCODE_PROVISION),
        _ => return true,
    };
    let mut chunk = [0u8; CHUNK_BYTES];
    let len = write.with_data(|_, data| {
        let len = data.len().min(CHUNK_BYTES);
        chunk[..len].copy_from_slice(&data[..len]);
        len
    });
    forward(stream, opcode, &chunk[..len]).await
}

async fn forward(stream: &mut StreamReassembler, opcode: u8, chunk: &[u8]) -> bool {
    let mut rest = chunk;
    while !rest.is_empty() {
        let mut packet = [0u8; MAX_PACKET_BYTES];
        let len = match stream.push(&mut rest) {
            Err(_) => return false,
            Ok(None) => continue,
            Ok(Some(frame)) if frame.first() == Some(&opcode) => {
                packet[..frame.len()].copy_from_slice(frame);
                frame.len()
            }
            Ok(Some(_)) => continue,
        };
        INBOUND_CHANNEL.send(Inbound::Frame(TransportKind::BleRelay, (packet, len))).await;
    }
    true
}

/// Sends each queued frame on the characteristic that matches its opcode, chunked to the MTU.
async fn writer(server: &Server<'_>, conn: &GattConnection<'_, '_, DefaultPacketPool>) {
    let bohs = &server.bohs;
    let mut wire = [0u8; LENGTH_PREFIX_BYTES + MAX_PACKET_BYTES];
    loop {
        let (packet, len) = OUTBOUND_BLE.receive().await;
        let Ok(total) = encode_stream_frame(&packet[..len], &mut wire) else { continue };
        let chunk_len = usize::from(conn.raw().att_mtu().saturating_sub(3)).clamp(20, CHUNK_BYTES);
        for chunk in wire[..total].chunks(chunk_len) {
            let sent = match packet[0] {
                OPCODE_AUDIO => bohs.audio.notify_raw(conn, chunk, false).await,
                OPCODE_CONTROL => bohs.control.indicate_raw(conn, chunk, false).await,
                _ => bohs.prov.indicate_raw(conn, chunk, false).await,
            };
            if sent.is_err() {
                break;
            }
        }
    }
}

//! USB CDC transport over the ESP32-S3 USB Serial/JTAG controller (`USB_DM` / `USB_DP`).

use crate::ble::TransportKind;
use super::net::INBOUND_CHANNEL;
use crate::link::OUTBOUND_PKT_CHANNEL;
use crate::stream::{encode_stream_frame, StreamReassembler, LENGTH_PREFIX_BYTES};
use crate::MAX_PACKET_BYTES;
use embedded_io_async::{Read, Write};
use esp_hal::usb::usb_serial_jtag::{UsbSerialJtagRx, UsbSerialJtagTx};
use esp_hal::Async;

/// Reassembles length-prefixed frames from the host and queues each one for the network task.
#[embassy_executor::task]
pub async fn usb_rx_task(mut rx: UsbSerialJtagRx<'static, Async>) {
    let mut reassembler = StreamReassembler::new();
    let mut chunk = [0u8; 64];
    loop {
        let Ok(read) = rx.read(&mut chunk).await;
        let mut rest = &chunk[..read];
        while !rest.is_empty() {
            let mut packet = [0u8; MAX_PACKET_BYTES];
            let Ok(Some(frame)) = reassembler.push(&mut rest) else {
                continue;
            };
            let len = frame.len();
            packet[..len].copy_from_slice(frame);
            INBOUND_CHANNEL.send((TransportKind::UsbSerial, (packet, len))).await;
        }
    }
}

/// Writes every outbound frame to the host with its `u16be` length prefix.
#[embassy_executor::task]
pub async fn usb_tx_task(mut tx: UsbSerialJtagTx<'static, Async>) {
    let mut wire = [0u8; LENGTH_PREFIX_BYTES + MAX_PACKET_BYTES];
    loop {
        let (packet, len) = OUTBOUND_PKT_CHANNEL.receive().await;
        let Ok(total) = encode_stream_frame(&packet[..len], &mut wire) else {
            continue;
        };
        if tx.write_all(&wire[..total]).await.is_ok() {
            let _ = tx.flush().await;
        }
    }
}

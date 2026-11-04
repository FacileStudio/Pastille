//! The protocol loop and the queues that feed it from the transports and sensors.

use crate::audio::AudioFrame;
use crate::ble::TransportKind;
use crate::link::{Mutex, Node, OUTBOUND_PKT_CHANNEL};
use crate::protocol::DeviceTelemetry;
use crate::{Packet, MAX_PACKET_BYTES};
use embassy_futures::select::{select3, Either3};
use embassy_sync::channel::Channel;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Instant, Timer};

/// Captured microphone frames waiting to go upstream; the I2S capture driver is not in v0.4.0.
pub static AUDIO_TX_CHANNEL: Channel<Mutex, AudioFrame, 4> = Channel::new();
/// Frames received from a transport, tagged with the link they came from.
pub static INBOUND_CHANNEL: Channel<Mutex, (TransportKind, Packet), 8> = Channel::new();
/// Latest sensor readings from the power task.
pub static TELEMETRY_SIGNAL: Signal<Mutex, DeviceTelemetry> = Signal::new();

/// Longest wait between two provisioning timer updates when no frame arrives.
const IDLE_TICK: Duration = Duration::from_millis(100);

/// Runs the protocol loop: answers every inbound frame as it arrives, sends captured audio,
/// and advances the provisioning window by the real elapsed time.
#[embassy_executor::task]
pub async fn network_task(hardware_id: [u8; 4]) {
    let mut node = Node::new(hardware_id);
    let mut last = Instant::now();
    loop {
        match select3(INBOUND_CHANNEL.receive(), AUDIO_TX_CHANNEL.receive(), Timer::after(IDLE_TICK)).await {
            Either3::First((link, (buf, len))) => node.handle_inbound(link, &buf[..len.min(MAX_PACKET_BYTES)]),
            Either3::Second(frame) => {
                let mut out = [0u8; MAX_PACKET_BYTES];
                if let Some(len) = node.encode_audio(&frame, &mut out) {
                    let _ = OUTBOUND_PKT_CHANNEL.try_send((out, len));
                }
            }
            Either3::Third(()) => {}
        }
        if let Some(telemetry) = TELEMETRY_SIGNAL.try_take() {
            node.dispatcher.telemetry = telemetry;
        }
        let now = Instant::now();
        node.prov.tick(u32::try_from((now - last).as_millis()).unwrap_or(u32::MAX));
        last = now;
    }
}

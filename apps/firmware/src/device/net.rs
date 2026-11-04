//! The protocol loop and the sensor queues that feed it.

use crate::audio::AudioFrame;
use crate::ble::ProvisionConfig;
use crate::link::{Mutex, INBOUND_CHANNEL};
use crate::node::Node;
use crate::protocol::DeviceTelemetry;
use embassy_futures::select::{select3, Either3};
use embassy_sync::channel::Channel;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Instant, Timer};

/// Captured microphone frames waiting to go upstream; the I2S capture driver is not in v0.4.0.
pub static AUDIO_TX_CHANNEL: Channel<Mutex, AudioFrame, 4> = Channel::new();
/// Latest sensor readings from the power task.
pub static TELEMETRY_SIGNAL: Signal<Mutex, DeviceTelemetry> = Signal::new();

/// Longest wait between two provisioning timer updates when no event arrives.
const IDLE_TICK: Duration = Duration::from_millis(100);

/// Runs the protocol loop: handles every inbound event as it arrives, sends captured audio,
/// and advances the provisioning window by the real elapsed time.
#[embassy_executor::task]
pub async fn network_task(hardware_id: [u8; 4], stored: Option<ProvisionConfig>) {
    let mut node = Node::new(hardware_id, stored);
    node.tick(0);
    let mut last = Instant::now();
    loop {
        match select3(INBOUND_CHANNEL.receive(), AUDIO_TX_CHANNEL.receive(), Timer::after(IDLE_TICK)).await {
            Either3::First(event) => node.handle(event),
            Either3::Second(frame) => node.send_audio(&frame),
            Either3::Third(()) => {}
        }
        if let Some(telemetry) = TELEMETRY_SIGNAL.try_take() {
            node.dispatcher.telemetry = telemetry;
        }
        let now = Instant::now();
        node.tick(u32::try_from((now - last).as_millis()).unwrap_or(u32::MAX));
        last = now;
    }
}

use crate::audio::AudioFrame;
use crate::avatar::{AvatarEmotion, AvatarState};
use crate::ble::{ProvisionConfig, ProvisionState, TransportKind};
use crate::events::TapType;
use crate::haptic::HapticPulse;
use crate::Packet;
use core::cell::Cell;
use core::sync::atomic::AtomicBool;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex as BlockingMutex;
use embassy_sync::channel::Channel;
use embassy_sync::signal::Signal;
use embassy_sync::watch::Watch;

/// The lock every cross-task channel and signal uses.
pub type Mutex = CriticalSectionRawMutex;
/// One outbound queue per transport.
pub type Outbound = Channel<Mutex, Packet, 8>;

/// Everything the network task reacts to, in arrival order. Frames travel inline: the channel
/// slots are static, so boxing them would only add heap traffic.
#[allow(clippy::large_enum_variant)]
pub enum Inbound {
    /// A complete bohs.v1 frame and the link it arrived on.
    Frame(TransportKind, Packet),
    /// A BLE central connected with this connection handle.
    BleConnected(u16),
    /// The BLE link changed security; `true` means encrypted with a bonded LE Secure Connections key.
    BleSecured(bool),
    /// The BLE central disconnected.
    BleDisconnected,
    /// The user touched the front pad.
    Tap(TapType, u64),
    /// A gateway session opened on this transport.
    LinkUp(TransportKind),
    /// A gateway session on this transport closed.
    LinkDown(TransportKind),
    /// The battery crossed a low threshold: charge percent and cell voltage.
    LowBattery(u8, u16),
}

/// What the display draws on top of the avatar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overlay {
    None,
    /// The `prov.identify` window is open.
    PairingRing,
    /// LE Secure Connections numeric comparison code waiting for a tap.
    Passkey(u32),
}

/// Requests for the flash storage task.
#[allow(clippy::large_enum_variant)]
pub enum StoreRequest {
    Config(ProvisionConfig),
    Bond([u8; crate::store::BOND_BYTES]),
}

/// State the BLE task advertises and serves in the `B001` record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceInfo {
    pub state: ProvisionState,
    pub transport: TransportKind,
    pub battery_pct: u8,
}

pub static INBOUND_CHANNEL: Channel<Mutex, Inbound, 8> = Channel::new();
/// Downstream frames and their sequence numbers, waiting for the speaker's jitter buffer.
pub static AUDIO_RX_CHANNEL: Channel<Mutex, (u16, AudioFrame), 8> = Channel::new();
/// Whether the microphone should run: a gateway session exists and the device is not asleep.
pub static CAPTURE_ENABLED: AtomicBool = AtomicBool::new(false);
pub static OUTBOUND_USB: Outbound = Channel::new();
pub static OUTBOUND_BLE: Outbound = Channel::new();
pub static OUTBOUND_WSS: Outbound = Channel::new();
/// Latest avatar face; a newer value replaces one the display has not drawn yet.
pub static AVATAR_SIGNAL: Signal<Mutex, (AvatarState, AvatarEmotion)> = Signal::new();
/// Latest haptic request for the motor driver.
pub static HAPTIC_SIGNAL: Signal<Mutex, HapticPulse> = Signal::new();
pub static OVERLAY_SIGNAL: Signal<Mutex, Overlay> = Signal::new();
pub static STORE_CHANNEL: Channel<Mutex, StoreRequest, 2> = Channel::new();
/// The current config, for the Wi-Fi and WSS tasks; set at boot and after every `prov.configure`.
pub static CONFIG_WATCH: Watch<Mutex, ProvisionConfig, 2> = Watch::new();
/// Raised whenever `DEVICE_INFO` changes, so the BLE task refreshes its advertising.
pub static INFO_CHANGED: Signal<Mutex, ()> = Signal::new();
pub static DEVICE_INFO: BlockingMutex<Mutex, Cell<DeviceInfo>> = BlockingMutex::new(Cell::new(DeviceInfo {
    state: ProvisionState::Unclaimed,
    transport: TransportKind::BleRelay,
    battery_pct: 0,
}));

/// The outbound queue of a transport.
pub fn outbound(link: TransportKind) -> &'static Outbound {
    match link {
        TransportKind::BleRelay => &OUTBOUND_BLE,
        TransportKind::WifiDirect => &OUTBOUND_WSS,
        TransportKind::UsbSerial => &OUTBOUND_USB,
    }
}

/// Reads the shared device snapshot.
pub fn device_info() -> DeviceInfo {
    DEVICE_INFO.lock(Cell::get)
}

/// Replaces the shared device snapshot and wakes the BLE task when it changed.
pub fn set_device_info(info: DeviceInfo) {
    if DEVICE_INFO.lock(|cell| cell.replace(info)) != info {
        INFO_CHANGED.signal(());
    }
}

use crate::ble::{BleProvisioner, ProvisionConfig, ProvisionState, DEVICE_CAPS};
use crate::link::DeviceInfo;
use core::fmt;

impl BleProvisioner {
    /// Records a new BLE central. A new link starts unencrypted and holds no authorization.
    pub fn on_connected(&mut self, handle: u16) {
        self.expire_auth();
        self.conn = Some(handle);
        self.link_encrypted = false;
    }

    /// Forgets the central and cancels any tap window it opened.
    pub fn on_disconnect(&mut self) {
        self.expire_auth();
        self.conn = None;
        self.link_encrypted = false;
    }

    /// Records the BLE link encryption status; losing it cancels any pending authorization.
    pub fn on_link_encrypted(&mut self, encrypted: bool) {
        self.link_encrypted = encrypted;
        if !encrypted {
            self.expire_auth();
        }
    }
}

impl fmt::Debug for ProvisionConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProvisionConfig")
            .field("gateway_url", &self.gateway_url)
            .field("device_token", &"<redacted>")
            .field("wifi_ssid", &self.wifi_ssid)
            .field("wifi_psk", &"<redacted>")
            .field("prefer_ble", &self.prefer_ble)
            .finish()
    }
}

/// The firmware version from `Cargo.toml`, as reported in the `B001` record.
pub fn firmware_version() -> [u8; 3] {
    [env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH")]
        .map(|part| part.parse().unwrap_or(u8::MAX))
}

/// Builds the 8-byte `B001` device info record.
pub fn info_record(info: &DeviceInfo) -> [u8; 8] {
    let [major, minor, patch] = firmware_version();
    [0x01, info.state as u8, DEVICE_CAPS, info.battery_pct, major, minor, patch, info.transport as u8]
}

/// Builds the 6-byte service data: state, capability bits, then the 4-byte hardware id.
pub fn service_data(state: ProvisionState, hardware_id: [u8; 4]) -> [u8; 6] {
    let [a, b, c, d] = hardware_id;
    [state as u8, DEVICE_CAPS, a, b, c, d]
}

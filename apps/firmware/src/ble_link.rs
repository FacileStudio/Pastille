use crate::ble::{BleProvisioner, ProvisionConfig};
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

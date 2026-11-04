use crate::protocol::ProtocolError;
use heapless::String;

/// Full 128-bit GATT UUIDs; the GATT server that registers them ships with the radio stack,
/// so outside tests nothing references them yet.
#[cfg_attr(not(test), allow(dead_code))]
pub mod gatt {
    /// BOHS-01 GATT service advertised by every charm.
    pub const BLE_SERVICE_UUID: u128 = 0xb0010000_f4c1_4e00_b001_000000000001;
    /// Read-only device information characteristic.
    pub const BLE_CHAR_INFO_UUID: u128 = 0xb0010001_f4c1_4e00_b001_000000000001;
    /// Audio characteristic carrying length-prefixed `0x01` frames (Notify / WriteNoRsp).
    pub const BLE_CHAR_AUDIO_UUID: u128 = 0xb0010002_f4c1_4e00_b001_000000000001;
    /// Control characteristic carrying length-prefixed `0x02` frames (Indicate / Write).
    pub const BLE_CHAR_CONTROL_UUID: u128 = 0xb0010003_f4c1_4e00_b001_000000000001;
    /// Provisioning characteristic carrying length-prefixed `0x03` frames (Indicate / Write).
    pub const BLE_CHAR_PROV_UUID: u128 = 0xb0010004_f4c1_4e00_b001_000000000001;
}

pub const AUTH_WINDOW_MS: u32 = 60_000;
pub const CAP_LCD: u8 = 0x04;
pub const CAP_HAP: u8 = 0x08;
/// Capabilities this firmware can run: the display and the haptic motor. Audio is not in v0.4.0.
pub const DEVICE_CAPS: u8 = CAP_LCD | CAP_HAP;
pub const DEVICE_TOKEN_PREFIX: &str = "bohs_tok_";
pub const DEVICE_TOKEN_LEN: usize = 41;

/// Link the charm uses to reach the gateway once provisioned.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransportKind {
    #[default]
    BleRelay = 0,
    WifiDirect = 1,
    UsbSerial = 2,
}

/// Pairing lifecycle; the discriminant is the first byte of the advertised service data.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProvisionState {
    #[default]
    Unclaimed = 0,
    AwaitingTap = 1,
    Authorized = 2,
    Provisioned = 3,
}

/// Settings written by `prov.configure`. Its `Debug` output redacts the token and the PSK.
#[derive(Clone, PartialEq, Eq)]
pub struct ProvisionConfig {
    pub gateway_url: String<96>,
    pub device_token: String<48>,
    pub wifi_ssid: String<32>,
    pub wifi_psk: String<64>,
    pub prefer_ble: bool,
}

/// Transport-independent BLE provisioning state machine.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BleProvisioner {
    pub state: ProvisionState,
    pub transport: TransportKind,
    pub auth_timer_ms: u32,
    pub link_encrypted: bool,
    pub config: Option<ProvisionConfig>,
    pub hardware_id: [u8; 4],
    /// Handle of the connected BLE central, if any.
    pub conn: Option<u16>,
    /// Connection that opened the current tap window; only it may send `prov.configure`.
    pub auth_conn: Option<u16>,
}

impl BleProvisioner {
    /// Builds an unclaimed provisioner for the charm whose hardware id is `hardware_id`.
    pub const fn new(hardware_id: [u8; 4]) -> Self {
        Self {
            state: ProvisionState::Unclaimed,
            transport: TransportKind::BleRelay,
            auth_timer_ms: 0,
            link_encrypted: false,
            config: None,
            hardware_id,
            conn: None,
            auth_conn: None,
        }
    }

    /// Restores a config loaded from flash at boot.
    pub fn restore(&mut self, cfg: ProvisionConfig) {
        self.transport = if cfg.prefer_ble { TransportKind::BleRelay } else { TransportKind::WifiDirect };
        self.config = Some(cfg);
        self.state = ProvisionState::Provisioned;
    }

    /// Opens the physical-tap window for the current connection. A window that is already open
    /// stays as it is, so a repeated `prov.identify` can neither extend nor reset it.
    pub fn on_identify_request(&mut self) {
        if matches!(self.state, ProvisionState::AwaitingTap | ProvisionState::Authorized) {
            return;
        }
        self.state = ProvisionState::AwaitingTap;
        self.auth_timer_ms = AUTH_WINDOW_MS;
        self.auth_conn = self.conn;
    }

    /// Authorizes the pending pairing when the user taps the pad inside the window.
    pub fn on_physical_tap(&mut self) {
        if self.state == ProvisionState::AwaitingTap {
            self.state = ProvisionState::Authorized;
        }
    }

    /// Advances the authorization window and cancels the pairing when it runs out.
    pub fn tick(&mut self, elapsed_ms: u32) {
        if !matches!(self.state, ProvisionState::AwaitingTap | ProvisionState::Authorized) {
            return;
        }
        self.auth_timer_ms = self.auth_timer_ms.saturating_sub(elapsed_ms);
        if self.auth_timer_ms == 0 {
            self.expire_auth();
        }
    }

    /// Stores a configuration; only the authorized connection, over an encrypted link, may do so.
    pub fn apply_config(&mut self, cfg: ProvisionConfig) -> Result<(), ProtocolError> {
        let same_conn = self.conn.is_some() && self.auth_conn == self.conn;
        if self.state != ProvisionState::Authorized || !self.link_encrypted || !same_conn {
            return Err(ProtocolError::Unauthorized);
        }
        self.restore(cfg);
        self.auth_timer_ms = 0;
        self.auth_conn = None;
        Ok(())
    }

    /// Closes the tap window and returns to the state held before `prov.identify`.
    pub(crate) fn expire_auth(&mut self) {
        if !matches!(self.state, ProvisionState::AwaitingTap | ProvisionState::Authorized) {
            return;
        }
        self.auth_timer_ms = 0;
        self.auth_conn = None;
        self.state = if self.config.is_some() {
            ProvisionState::Provisioned
        } else {
            ProvisionState::Unclaimed
        };
    }
}

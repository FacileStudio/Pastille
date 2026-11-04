use crate::ble::TransportKind;

pub const SLEEP_GRACE_TIMEOUT_MS: u32 = 30_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PowerState {
    #[default]
    DeepSleep,
    BootWake,
    BleFastConnect,
    WifiFastAssociate,
    StreamFullDuplex,
    SleepGrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PowerManager {
    pub state: PowerState,
    pub silence_ms: u32,
}

impl PowerManager {
    pub const fn new() -> Self {
        Self {
            state: PowerState::DeepSleep,
            silence_ms: 0,
        }
    }

    pub fn on_wake_trigger(&mut self) {
        self.silence_ms = 0;
        self.state = match self.state {
            PowerState::DeepSleep => PowerState::BootWake,
            PowerState::SleepGrace => PowerState::StreamFullDuplex,
            other => other,
        };
    }

    /// Leaves `BootWake` for the radio that matches the selected transport.
    /// BLE advertising stays on for relay and USB modes, so only `WifiDirect` associates Wi-Fi.
    pub fn on_hardware_ready(&mut self, transport: TransportKind) {
        if self.state == PowerState::BootWake {
            self.state = match transport {
                TransportKind::WifiDirect => PowerState::WifiFastAssociate,
                _ => PowerState::BleFastConnect,
            };
        }
    }

    /// Enters full-duplex streaming once a BLE central has connected.
    pub fn on_ble_connected(&mut self) {
        if matches!(
            self.state,
            PowerState::BootWake | PowerState::BleFastConnect | PowerState::WifiFastAssociate
        ) {
            self.silence_ms = 0;
            self.state = PowerState::StreamFullDuplex;
        }
    }

    pub fn on_ws_connected(&mut self) {
        if self.state == PowerState::WifiFastAssociate {
            self.silence_ms = 0;
            self.state = PowerState::StreamFullDuplex;
        }
    }

    pub fn on_audio_activity(&mut self, active: bool, elapsed_ms: u32) -> PowerState {
        if active {
            self.silence_ms = 0;
            if self.state == PowerState::SleepGrace {
                self.state = PowerState::StreamFullDuplex;
            }
            return self.state;
        }

        match self.state {
            PowerState::StreamFullDuplex => {
                self.silence_ms = elapsed_ms;
                self.state = PowerState::SleepGrace;
            }
            PowerState::SleepGrace => {
                self.silence_ms = self.silence_ms.saturating_add(elapsed_ms);
                if self.silence_ms >= SLEEP_GRACE_TIMEOUT_MS {
                    self.state = PowerState::DeepSleep;
                }
            }
            _ => {}
        }
        self.state
    }
}

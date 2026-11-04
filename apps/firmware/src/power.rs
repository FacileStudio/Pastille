pub const SLEEP_GRACE_TIMEOUT_MS: u32 = 30_000;
pub const LOW_BATTERY_WARN_PCT: u8 = 15;
pub const LOW_BATTERY_CRIT_PCT: u8 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PowerState {
    #[default]
    DeepSleep,
    BootWake,
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

    pub fn on_hardware_ready(&mut self) {
        if self.state == PowerState::BootWake {
            self.state = PowerState::WifiFastAssociate;
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

    pub const fn is_amp_active(&self) -> bool {
        matches!(self.state, PowerState::StreamFullDuplex)
    }

    pub fn battery_pct_from_mv(voltage_mv: u16) -> u8 {
        if voltage_mv <= 3300 {
            return 0;
        }
        if voltage_mv >= 4200 {
            return 100;
        }
        (((voltage_mv - 3300) as u32 * 100) / 900) as u8
    }

    pub fn low_battery_threshold(voltage_mv: u16) -> Option<u8> {
        let pct = Self::battery_pct_from_mv(voltage_mv);
        if pct <= LOW_BATTERY_CRIT_PCT || pct <= LOW_BATTERY_WARN_PCT {
            Some(pct)
        } else {
            None
        }
    }
}

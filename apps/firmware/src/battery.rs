use crate::power::PowerManager;

pub const LOW_BATTERY_WARN_PCT: u8 = 15;
pub const LOW_BATTERY_CRIT_PCT: u8 = 5;
/// Charge points above an alerted threshold before that threshold can alert again.
pub const LOW_BATTERY_REARM_PCT: u8 = 3;

impl PowerManager {
    /// Converts a single-cell LiPo voltage into a 0-100 charge percentage.
    pub fn battery_pct_from_mv(voltage_mv: u16) -> u8 {
        if voltage_mv <= 3300 {
            return 0;
        }
        if voltage_mv >= 4200 {
            return 100;
        }
        (((voltage_mv - 3300) as u32 * 100) / 900) as u8
    }
}

/// Thresholds that raise `charm.low_battery`, lowest first.
const LOW_BATTERY_THRESHOLDS: [u8; 2] = [LOW_BATTERY_CRIT_PCT, LOW_BATTERY_WARN_PCT];

/// Edge detector for `charm.low_battery`: each threshold alerts once when the charge drops
/// below it, and re-arms after the charge climbs back past it by `LOW_BATTERY_REARM_PCT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryMonitor {
    armed: [bool; 2],
}

impl BatteryMonitor {
    pub const fn new() -> Self {
        Self { armed: [true; 2] }
    }

    /// Feeds one sample and returns the lowest threshold it just crossed downwards, if any.
    pub fn update(&mut self, pct: u8) -> Option<u8> {
        let mut crossed = None;
        for (armed, threshold) in self.armed.iter_mut().zip(LOW_BATTERY_THRESHOLDS) {
            if pct >= threshold.saturating_add(LOW_BATTERY_REARM_PCT) {
                *armed = true;
            } else if pct < threshold && *armed {
                *armed = false;
                crossed = crossed.or(Some(threshold));
            }
        }
        crossed
    }
}

use crate::power::PowerManager;

pub const LOW_BATTERY_WARN_PCT: u8 = 15;
pub const LOW_BATTERY_CRIT_PCT: u8 = 5;

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

    /// Returns the charge percentage when it sits at or below a low-battery threshold.
    pub fn low_battery_threshold(voltage_mv: u16) -> Option<u8> {
        let pct = Self::battery_pct_from_mv(voltage_mv);
        if pct <= LOW_BATTERY_CRIT_PCT || pct <= LOW_BATTERY_WARN_PCT {
            Some(pct)
        } else {
            None
        }
    }
}

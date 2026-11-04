use crate::events::TapType;

/// Samples averaged into the boot baseline.
pub const CALIBRATION_SAMPLES: usize = 16;
/// Rise over the baseline, in thousandths, that counts as a press. The ESP32-S3 count grows with touch.
pub const PRESS_PERMILLE: u32 = 20;
/// Rise over the baseline, in thousandths, below which a press ends.
pub const RELEASE_PERMILLE: u32 = 10;
/// Samples a press may last before the baseline is reset to the current count (10 s at 50 Hz).
pub const STUCK_PRESS_SAMPLES: u32 = 500;
/// A press held this long is a long press.
pub const LONG_PRESS_MS: u64 = 800;
/// A second tap released within this delay of the first makes a double tap.
pub const DOUBLE_TAP_MS: u64 = 350;

/// Edge detector on the raw touch count, with a baseline calibrated at boot that follows slow drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TouchDetector {
    pub baseline: u32,
    pub pressed: bool,
    held_samples: u32,
}

impl TouchDetector {
    /// Calibrates the baseline from untouched samples; `None` when the pad reads zero.
    pub fn calibrate(samples: &[u32]) -> Option<Self> {
        let count = u64::try_from(samples.len()).ok().filter(|&n| n > 0)?;
        let mean = samples.iter().map(|&s| u64::from(s)).sum::<u64>() / count;
        let baseline = u32::try_from(mean).ok().filter(|&b| b > 0)?;
        Some(Self { baseline, pressed: false, held_samples: 0 })
    }

    fn above(&self, raw: u32, permille: u32) -> bool {
        u64::from(raw) * 1000 > u64::from(self.baseline) * u64::from(1000 + permille)
    }

    /// Feeds one raw count and returns `Some(true)` on a press edge, `Some(false)` on a release edge.
    /// The baseline follows counts below the release level, and a press that lasts longer than
    /// `STUCK_PRESS_SAMPLES` is taken as drift: the baseline jumps to the current count.
    pub fn update(&mut self, raw: u32) -> Option<bool> {
        let was = self.pressed;
        self.pressed = if was { self.above(raw, RELEASE_PERMILLE) } else { self.above(raw, PRESS_PERMILLE) };
        self.held_samples = if self.pressed { self.held_samples + 1 } else { 0 };
        if self.held_samples > STUCK_PRESS_SAMPLES {
            *self = Self { baseline: raw.max(1), pressed: false, held_samples: 0 };
        } else if !self.above(raw, RELEASE_PERMILLE) {
            let diff = i64::from(raw) - i64::from(self.baseline);
            self.baseline = (i64::from(self.baseline) + diff / 64).max(1) as u32;
        }
        (self.pressed != was).then_some(self.pressed)
    }
}

/// Turns press and release edges into single taps, double taps and long presses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TapClassifier {
    pressed_at: Option<u64>,
    pending_tap_at: Option<u64>,
}

impl TapClassifier {
    /// Records a press or release edge at `now_ms` and returns a finished gesture, if any.
    pub fn edge(&mut self, pressed: bool, now_ms: u64) -> Option<TapType> {
        if pressed {
            self.pressed_at = Some(now_ms);
            return None;
        }
        let held = now_ms.saturating_sub(self.pressed_at.take()?);
        if held >= LONG_PRESS_MS {
            self.pending_tap_at = None;
            return Some(TapType::LongPress);
        }
        if self.pending_tap_at.take().is_some() {
            return Some(TapType::DoubleTap);
        }
        self.pending_tap_at = Some(now_ms);
        None
    }

    /// Reports a single tap once no second tap followed it in time.
    pub fn poll(&mut self, now_ms: u64) -> Option<TapType> {
        let at = self.pending_tap_at?;
        if now_ms.saturating_sub(at) <= DOUBLE_TAP_MS {
            return None;
        }
        self.pending_tap_at = None;
        Some(TapType::SingleTap)
    }
}

use crate::protocol::ProtocolError;

/// Vibration patterns accepted by `charm.haptic_pulse`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HapticPattern {
    Tap,
    DoubleTap,
    Alert,
    Error,
}

/// One validated `charm.haptic_pulse` request, ready for the motor driver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HapticPulse {
    pub pattern: HapticPattern,
    pub duration_ms: u16,
    pub intensity: u8,
}

impl HapticPattern {
    /// Parses the `pattern` argument of `charm.haptic_pulse`.
    pub fn parse(name: &str) -> Result<Self, ProtocolError> {
        match name {
            "tap" => Ok(Self::Tap),
            "double_tap" => Ok(Self::DoubleTap),
            "alert" => Ok(Self::Alert),
            "error" => Ok(Self::Error),
            _ => Err(ProtocolError::InvalidArguments),
        }
    }

    /// Number of motor pulses the pattern plays, each `duration_ms` long.
    pub const fn pulse_count(self) -> u8 {
        match self {
            Self::Tap => 1,
            Self::DoubleTap => 2,
            Self::Alert => 3,
            Self::Error => 4,
        }
    }
}

impl HapticPulse {
    /// Maps the 1-100 intensity to the DRV2605L unsigned real-time-playback level.
    pub const fn rtp_level(&self) -> u8 {
        ((self.intensity as u16 * 255) / 100) as u8
    }
}

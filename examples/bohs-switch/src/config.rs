use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Where the daemon reads its settings when no `--config` is given.
pub const DEFAULT_PATH: &str = "/etc/bohs-switch.toml";

/// The daemon's settings. Every field has a default, so the file is optional.
#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// The Bulle server, as an https URL with no path.
    pub gateway: String,
    /// The name the device suggests when it pairs. The owner can change it.
    pub name: String,
    /// Where the device token is kept, readable by this process only.
    pub state_dir: PathBuf,
    /// The GPIO character device.
    pub chip: String,
    /// The GPIO line offset. On a Raspberry Pi it is the BCM number: 17 is header pin 11.
    pub line: u32,
    /// Set for relay boards that switch on when the pin is low.
    pub active_low: bool,
    /// The state the load takes at boot.
    pub initial_on: bool,
    /// A console to draw the pairing code on, such as `/dev/tty1` for an HDMI screen.
    pub tty: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gateway: "https://bulle.facile.studio".into(),
            name: "Living Room Light".into(),
            state_dir: "/var/lib/bohs-switch".into(),
            chip: "/dev/gpiochip0".into(),
            line: 17,
            active_low: false,
            initial_on: false,
            tty: None,
        }
    }
}

impl Config {
    /// Reads `path`, or answers the defaults when the file does not exist.
    pub fn load(path: &Path) -> Result<Self, String> {
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(e) => return Err(format!("cannot read {}: {e}", path.display())),
        };
        let config: Self = toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        if !config.gateway.starts_with("https://") && !config.gateway.starts_with("http://") {
            return Err(format!("{}: gateway must be an http(s) URL", path.display()));
        }
        Ok(config)
    }

    /// Answers the gateway URL with no trailing slash.
    pub fn base(&self) -> &str {
        self.gateway.trim_end_matches('/')
    }
}

use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

/// What the gateway handed the device when it was paired.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub device_id: String,
    pub name: String,
    pub device_token: String,
    pub gateway_ws_url: String,
}

fn file(dir: &Path) -> PathBuf {
    dir.join("credentials.json")
}

/// Reads the saved credentials, or none when the device is not paired.
pub fn load(dir: &Path) -> Option<Credentials> {
    let text = fs::read_to_string(file(dir)).ok()?;
    serde_json::from_str(&text).ok()
}

/// Writes the credentials with mode 0600 in a 0700 directory, replacing the old file at once.
pub fn save(dir: &Path, creds: &Credentials) -> Result<(), String> {
    fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?;
    let tmp = dir.join("credentials.json.tmp");
    let mut out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&tmp)
        .map_err(|e| format!("{}: {e}", tmp.display()))?;
    let body = serde_json::to_vec(creds).map_err(|e| e.to_string())?;
    out.write_all(&body)
        .and_then(|()| out.sync_all())
        .map_err(|e| format!("{}: {e}", tmp.display()))?;
    fs::rename(&tmp, file(dir)).map_err(|e| format!("{}: {e}", dir.display()))
}

/// Deletes the credentials of a revoked device, so it pairs again.
pub fn forget(dir: &Path) {
    let _ = fs::remove_file(file(dir));
}

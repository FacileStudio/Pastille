use crate::config::Config;
use crate::creds::Credentials;
use crate::display;
use serde::Deserialize;
use serde_json::json;
use std::thread::sleep;
use std::time::{Duration, Instant};

const HTTP_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Deserialize)]
struct Started {
    user_code: String,
    device_code: String,
    expires_in: u64,
    interval: u64,
}

#[derive(Deserialize)]
struct Polled {
    status: String,
    device: Option<Device>,
    device_token: Option<String>,
    gateway_ws_url: Option<String>,
}

#[derive(Deserialize)]
struct Device {
    id: String,
    name: String,
}

/// Asks the gateway for a code, shows it, and waits until the owner enters it in Bulle.
/// Answers none when the code expired, so the caller asks for a fresh one.
pub fn pair(cfg: &Config, hardware_id: &str) -> Result<Option<Credentials>, String> {
    let body = json!({"hardware_id": hardware_id, "device_class": "field", "name": cfg.name});
    let agent = agent();
    let started: Started = agent
        .post(format!("{}/api/hardware/pair", cfg.base()))
        .send_json(&body)
        .and_then(|mut r| r.body_mut().read_json())
        .map_err(|e| format!("cannot reach {}: {e}", cfg.base()))?;
    display::code(&started.user_code, hardware_id, started.expires_in, cfg);
    let deadline = Instant::now() + Duration::from_secs(started.expires_in);
    let interval = Duration::from_secs(started.interval.max(1));
    while Instant::now() < deadline {
        sleep(interval);
        match poll(&agent, cfg, &started.device_code) {
            Ok(Some(creds)) => return Ok(Some(creds)),
            Ok(None) => {}
            Err(Expired) => return Ok(None),
        }
    }
    Ok(None)
}

struct Expired;

fn agent() -> ureq::Agent {
    ureq::Agent::new_with_config(ureq::Agent::config_builder().timeout_global(Some(HTTP_TIMEOUT)).build())
}

fn poll(agent: &ureq::Agent, cfg: &Config, device_code: &str) -> Result<Option<Credentials>, Expired> {
    let answer = agent
        .post(format!("{}/api/hardware/pair/poll", cfg.base()))
        .send_json(json!({"device_code": device_code}))
        .and_then(|mut r| r.body_mut().read_json::<Polled>());
    let polled = match answer {
        Ok(polled) => polled,
        Err(ureq::Error::StatusCode(404)) => return Err(Expired),
        Err(e) => {
            eprintln!("poll failed, retrying: {e}");
            return Ok(None);
        }
    };
    if polled.status != "claimed" {
        return Ok(None);
    }
    match (polled.device, polled.device_token, polled.gateway_ws_url) {
        (Some(d), Some(token), Some(url)) => Ok(Some(Credentials {
            device_id: d.id,
            name: d.name,
            device_token: token,
            gateway_ws_url: url,
        })),
        _ => Err(Expired),
    }
}

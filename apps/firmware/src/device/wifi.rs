//! Wi-Fi station: joins the network from the stored config and rejoins with backoff.

use crate::ble::ProvisionConfig;
use crate::link::CONFIG_WATCH;
use embassy_futures::select::{select, Either};
use embassy_net::Runner;
use embassy_time::{Duration, Timer};
use esp_radio::wifi::sta::StationConfig;
use esp_radio::wifi::{AuthenticationMethodConfig, Config, Interface, WifiController};

/// First retry delay after a failed or lost association; it doubles up to `MAX_BACKOFF`.
pub const MIN_BACKOFF: Duration = Duration::from_secs(1);
pub const MAX_BACKOFF: Duration = Duration::from_secs(60);

/// Whether this config asks for the direct Wi-Fi transport.
pub fn wants_wifi(cfg: &ProvisionConfig) -> bool {
    !cfg.prefer_ble && !cfg.wifi_ssid.is_empty()
}

fn station(cfg: &ProvisionConfig) -> Option<Config> {
    let auth = if cfg.wifi_psk.is_empty() {
        AuthenticationMethodConfig::Open
    } else {
        AuthenticationMethodConfig::Wpa2Personal(cfg.wifi_psk.as_str().try_into().ok()?)
    };
    let ssid = cfg.wifi_ssid.as_str().try_into().ok()?;
    Some(Config::Station(StationConfig::default().with_ssid(ssid).with_authentication(auth)))
}

/// Keeps the station associated whenever the config selects Wi-Fi direct.
#[embassy_executor::task]
pub async fn wifi_task(mut controller: WifiController<'static>) {
    let Some(mut configs) = CONFIG_WATCH.receiver() else { return };
    let mut backoff = MIN_BACKOFF;
    loop {
        let cfg = configs.get().await;
        let Some(config) = station(&cfg).filter(|_| wants_wifi(&cfg)) else {
            let _ = configs.changed().await;
            continue;
        };
        let joined = controller.set_config(&config).is_ok() && controller.connect_async().await.is_ok();
        if joined {
            backoff = MIN_BACKOFF;
            if let Either::Second(_) = select(controller.wait_for_disconnect_async(), configs.changed()).await {
                let _ = controller.disconnect_async().await;
            }
            continue;
        }
        Timer::after(backoff).await;
        backoff = (backoff * 2).min(MAX_BACKOFF);
    }
}

/// Drives the embassy-net stack.
#[embassy_executor::task]
pub async fn net_runner_task(mut runner: Runner<'static, Interface>) {
    runner.run().await
}


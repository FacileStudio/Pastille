//! Power task: samples the battery divider and the charger status line, publishes telemetry
//! and raises `charm.low_battery` alerts.

use super::board_pin;
use crate::battery::BatteryMonitor;
use super::net::TELEMETRY_SIGNAL;
use crate::link::{device_info, set_device_info, DeviceInfo, Inbound, INBOUND_CHANNEL};
use crate::pins::POWER_PINS;
use crate::power::PowerManager;
use crate::protocol::DeviceTelemetry;
use embassy_time::{Duration, Timer};
use esp_hal::analog::adc::{Adc, AdcCalCurve, AdcConfig, AdcPin, Attenuation};
use esp_hal::gpio::{Input, InputConfig};
use esp_hal::peripherals::{ADC1, GPIO1};
use esp_hal::Async;

/// The ADC needs the typed pin, so the battery sense GPIO is checked against `pins.rs` at build time.
const _: () = assert!(POWER_PINS.batt_sense == 1, "BATT_SENSE moved: update the ADC pin in sensors.rs");

/// The 100k / 100k divider halves the cell voltage before the ADC.
const DIVIDER_RATIO: u16 = 2;
const SAMPLE_PERIOD: Duration = Duration::from_secs(10);

type BatteryPin = AdcPin<GPIO1<'static>, ADC1<'static>, AdcCalCurve<ADC1<'static>>>;

async fn battery_mv(adc: &mut Adc<'static, ADC1<'static>, Async>, pin: &mut BatteryPin) -> u16 {
    adc.read_oneshot(pin).await.saturating_mul(DIVIDER_RATIO)
}

/// Samples the sensors every 10 s and publishes the readings to the network task.
#[embassy_executor::task]
pub async fn power_task(adc1: ADC1<'static>, batt_sense: GPIO1<'static>) {
    let mut config = AdcConfig::new();
    let mut pin = config.enable_pin_with_cal::<_, AdcCalCurve<ADC1>>(batt_sense, Attenuation::_11dB);
    let mut adc = Adc::new(adc1, config).into_async();
    let charging = Input::new(board_pin(POWER_PINS.chg_stat), InputConfig::default());
    let mut monitor = BatteryMonitor::new();
    loop {
        let voltage_mv = battery_mv(&mut adc, &mut pin).await;
        let battery_pct = PowerManager::battery_pct_from_mv(voltage_mv);
        TELEMETRY_SIGNAL.signal(DeviceTelemetry {
            battery_pct,
            is_charging: charging.is_low(),
            rssi_dbm: None,
            temp_c: None,
        });
        set_device_info(DeviceInfo { battery_pct, ..device_info() });
        if monitor.update(battery_pct).is_some() {
            let _ = INBOUND_CHANNEL.try_send(Inbound::LowBattery(battery_pct, voltage_mv));
        }
        Timer::after(SAMPLE_PERIOD).await;
    }
}

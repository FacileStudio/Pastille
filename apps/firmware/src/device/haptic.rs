//! DRV2605L haptic driver on `I2C_SDA` / `I2C_SCL`, driving the LRA in open-loop real-time playback.

use super::board_pin;
use crate::haptic::HapticPulse;
use crate::link::HAPTIC_SIGNAL;
use crate::pins::BOARD_PINS;
use embassy_time::{Duration, Timer};
use esp_hal::i2c::master::{Config, Error, I2c};
use esp_hal::peripherals::I2C0;
use esp_hal::time::Rate;
use esp_hal::Async;

/// Gap between two pulses of one pattern.
const PULSE_GAP_MS: u64 = 60;
const DRV2605_ADDR: u8 = 0x5A;
const REG_MODE: u8 = 0x01;
const REG_RTP_INPUT: u8 = 0x02;
const REG_FEEDBACK: u8 = 0x1A;
const REG_CONTROL3: u8 = 0x1D;
const MODE_RTP: u8 = 0x05;
const MODE_STANDBY: u8 = 0x40;
/// `N_ERM_LRA` set: the actuator is an LRA.
const FEEDBACK_LRA: u8 = 0xB6;
/// Default `CONTROL3` with unsigned RTP data and LRA open-loop drive.
const CONTROL3_OPEN_LOOP_UNSIGNED: u8 = 0xA9;

/// Builds the async I2C master on the pins `pins.rs` assigns to the haptic bus.
pub fn bus(i2c: I2C0<'static>) -> Option<I2c<'static, Async>> {
    let config = Config::default().with_frequency(Rate::from_khz(400));
    let bus = I2c::new(i2c, config).ok()?;
    let sda = board_pin(BOARD_PINS.general.i2c_sda);
    let scl = board_pin(BOARD_PINS.general.i2c_scl);
    Some(bus.with_sda(sda).with_scl(scl).into_async())
}

async fn write(i2c: &mut I2c<'static, Async>, reg: u8, value: u8) -> Result<(), Error> {
    i2c.write_async(DRV2605_ADDR, &[reg, value]).await
}

async fn configure(i2c: &mut I2c<'static, Async>) -> Result<(), Error> {
    write(i2c, REG_FEEDBACK, FEEDBACK_LRA).await?;
    write(i2c, REG_CONTROL3, CONTROL3_OPEN_LOOP_UNSIGNED).await?;
    write(i2c, REG_MODE, MODE_STANDBY).await
}

async fn play(i2c: &mut I2c<'static, Async>, pulse: HapticPulse) -> Result<(), Error> {
    write(i2c, REG_MODE, MODE_RTP).await?;
    for n in 0..pulse.pattern.pulse_count() {
        if n > 0 {
            Timer::after(Duration::from_millis(PULSE_GAP_MS)).await;
        }
        write(i2c, REG_RTP_INPUT, pulse.rtp_level()).await?;
        Timer::after(Duration::from_millis(pulse.duration_ms.into())).await;
        write(i2c, REG_RTP_INPUT, 0).await?;
    }
    write(i2c, REG_MODE, MODE_STANDBY).await
}

/// Plays each requested pulse; stops for good when the driver does not answer at boot.
#[embassy_executor::task]
pub async fn haptic_task(mut i2c: I2c<'static, Async>) {
    if configure(&mut i2c).await.is_err() {
        return;
    }
    loop {
        let pulse = HAPTIC_SIGNAL.wait().await;
        if play(&mut i2c, pulse).await.is_err() {
            let _ = write(&mut i2c, REG_MODE, MODE_STANDBY).await;
        }
    }
}

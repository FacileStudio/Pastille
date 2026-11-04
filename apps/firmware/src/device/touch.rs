//! Capacitive touch on the `TOUCH_WAKE` pad, driven through the RTC touch sensor registers
//! because esp-hal has no ESP32-S3 touch driver. The sequence follows ESP-IDF v5.3
//! `touch_sensor_ll.h` for the ESP32-S3, in software-triggered mode.

use crate::link::{Inbound, Mutex, INBOUND_CHANNEL};
use crate::pins::BOARD_PINS;
use crate::touch::{TapClassifier, TouchDetector, CALIBRATION_SAMPLES};
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Instant, Timer};
use esp_hal::peripherals::{LPWR, RTC_IO, SENS};

/// Raised on every press edge; the BLE task waits on it to confirm a numeric comparison code.
pub static PRESS_SIGNAL: Signal<Mutex, ()> = Signal::new();

/// On the ESP32-S3, touch channel `n` is GPIO `n`; the per-pad slope field exists for channels 1 to 9.
const PAD: usize = BOARD_PINS.general.touch_wake as usize;
const _: () = assert!(PAD >= 1 && PAD <= 9, "TOUCH_WAKE must be on touch channel 1 to 9 (GPIO1 to GPIO9)");
const SAMPLE_PERIOD: Duration = Duration::from_millis(20);
const MEASURE_CYCLES: u16 = 500;
const THRESHOLD_MAX: u32 = 0x1F_FFFF;
const SLOPE_FAST: u32 = 7;

fn configure_controller() {
    let rtc = LPWR::regs();
    rtc.touch_ctrl2().modify(|_, w| w.touch_start_en().clear_bit().touch_slp_timer_en().clear_bit());
    rtc.touch_ctrl1().modify(|_, w| unsafe { w.touch_meas_num().bits(MEASURE_CYCLES).touch_sleep_cycles().bits(0xF) });
    rtc.touch_ctrl2().modify(|_, w| unsafe {
        w.touch_xpd_wait().bits(0xFF).touch_dbias().set_bit().touch_drefh().bits(3).touch_drefl().bits(0);
        w.touch_drange().bits(2).touch_start_force().set_bit().touch_start_fsm_en().set_bit();
        w.touch_clkgate_en().set_bit()
    });
    rtc.touch_ctrl2().modify(|_, w| unsafe { w.touch_timer_force_done().bits(3) });
    rtc.touch_ctrl2().modify(|_, w| unsafe { w.touch_timer_force_done().bits(0) });
    rtc.touch_scan_ctrl().modify(|r, w| unsafe {
        w.touch_inactive_connection().set_bit().touch_scan_pad_map().bits(r.touch_scan_pad_map().bits() | 1 << PAD)
    });
    let shift = 29 - 3 * PAD as u32;
    rtc.touch_dac().modify(|r, w| unsafe { w.bits((r.bits() & !(7 << shift)) | SLOPE_FAST << shift) });
}

fn configure_pad() {
    let sens = SENS::regs();
    sens.sar_peri_clk_gate_conf().modify(|_, w| w.iomux_clk_en().set_bit());
    RTC_IO::regs().touch_pad(PAD).modify(|_, w| unsafe {
        w.mux_sel().set_bit().fun_sel().bits(0).rue().clear_bit().rde().clear_bit().tie_opt().clear_bit()
    });
    sens.sar_touch_thres(PAD - 1).modify(|_, w| unsafe { w.threshold().bits(THRESHOLD_MAX) });
    sens.sar_touch_conf().modify(|r, w| unsafe {
        w.sar_touch_data_sel().bits(0).sar_touch_outen().bits(r.sar_touch_outen().bits() | 1 << PAD)
    });
    sens.sar_touch_chn_st().modify(|_, w| unsafe { w.sar_touch_channel_clr().bits(0x7FFF) });
}

/// Starts one software measurement and returns the raw count, or `None` if it never completes.
async fn measure() -> Option<u32> {
    let rtc = LPWR::regs();
    rtc.touch_ctrl2().modify(|_, w| w.touch_start_en().clear_bit());
    rtc.touch_ctrl2().modify(|_, w| w.touch_start_en().set_bit());
    for _ in 0..10 {
        Timer::after(Duration::from_millis(1)).await;
        let sens = SENS::regs();
        if sens.sar_touch_chn_st().read().sar_touch_meas_done().bit_is_set() {
            return Some(sens.sar_touch_status(PAD - 1).read().data().bits());
        }
    }
    None
}

async fn calibrate() -> Option<TouchDetector> {
    let mut samples = [0u32; CALIBRATION_SAMPLES];
    for sample in &mut samples {
        *sample = measure().await?;
        Timer::after(SAMPLE_PERIOD).await;
    }
    TouchDetector::calibrate(&samples)
}

/// Samples the pad at 50 Hz and reports every gesture; stops when the sensor does not answer at boot.
#[embassy_executor::task]
pub async fn touch_task() {
    configure_controller();
    configure_pad();
    let Some(mut pad) = calibrate().await else {
        return;
    };
    let mut taps = TapClassifier::default();
    loop {
        Timer::after(SAMPLE_PERIOD).await;
        let now = Instant::now().as_millis();
        let edge = measure().await.and_then(|raw| pad.update(raw));
        if edge == Some(true) {
            PRESS_SIGNAL.signal(());
        }
        let gesture = edge.and_then(|pressed| taps.edge(pressed, now)).or_else(|| taps.poll(now));
        if let Some(kind) = gesture {
            let _ = INBOUND_CHANNEL.try_send(Inbound::Tap(kind, now));
        }
    }
}

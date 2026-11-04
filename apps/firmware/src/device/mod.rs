//! On-device entry point: brings up the clocks, heap and scheduler, then spawns every task.

mod display;
mod haptic;
mod net;
mod sensors;
mod usb;

use net::network_task;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::AnyPin;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::usb::usb_serial_jtag::UsbSerialJtag;

esp_bootloader_esp_idf::esp_app_desc!();

/// Internal-RAM heap for `serde_json`; a 1 KiB JSON-RPC message needs a few KiB. No PSRAM is used.
const HEAP_BYTES: usize = 48 * 1024;

/// Takes the GPIO that `pins.rs` assigns to a net.
///
/// Safety: `pins.rs` gives every net its own GPIO, and each net is taken once, at boot, through
/// this function. No typed `GPIOn` peripheral with the same number is used anywhere else.
fn board_pin(gpio: u8) -> AnyPin<'static> {
    unsafe { AnyPin::steal(gpio) }
}

/// The last 4 bytes of the eFuse base MAC; the first 3 are the vendor OUI and repeat across charms.
fn hardware_id() -> [u8; 4] {
    let mac = esp_hal::efuse::base_mac_address();
    let mut id = [0u8; 4];
    id.copy_from_slice(&mac.as_bytes()[2..6]);
    id
}

#[esp_hal::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    esp_alloc::heap_allocator!(size: HEAP_BYTES);
    let timg0 = TimerGroup::new(p.TIMG0);
    esp_rtos::start(timg0.timer0, p.FROM_CPU_INTR0);

    let (rx, tx) = UsbSerialJtag::new(p.USB_DEVICE).into_async().split();
    spawner.spawn(usb::usb_rx_task(rx).unwrap());
    spawner.spawn(usb::usb_tx_task(tx).unwrap());
    spawner.spawn(network_task(hardware_id()).unwrap());
    spawner.spawn(sensors::power_task(p.ADC1, p.GPIO1).unwrap());
    if let Some(i2c) = haptic::bus(p.I2C0) {
        spawner.spawn(haptic::haptic_task(i2c).unwrap());
    }
    if let Some(panel) = display::panel(p.SPI2) {
        spawner.spawn(display::display_task(panel).unwrap());
    }
}

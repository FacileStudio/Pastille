//! On-device entry point: brings up the clocks, heap and scheduler, then spawns every task.

mod ble;
mod display;
mod flash;
mod haptic;
mod net;
mod rng;
mod sensors;
mod tls;
pub(crate) mod touch;
mod usb;
mod wifi;
mod wss;
mod wss_io;

use crate::link::CONFIG_WATCH;
use embassy_executor::Spawner;
use embassy_net::{Stack, StackResources};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::AnyPin;
use esp_hal::peripherals::{ADC1, GPIO1, I2C0, SPI2, USB_DEVICE};
use esp_hal::rng::Rng;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::usb::usb_serial_jtag::UsbSerialJtag;
use esp_radio::ble::controller::BleConnector;
use esp_radio::wifi::{ControllerConfig, Interface, WifiController};
use net::network_task;
use static_cell::StaticCell;

esp_bootloader_esp_idf::esp_app_desc!();

/// Internal-RAM heap for the radio drivers, `serde_json` and RSA certificate checks. No PSRAM is used.
const HEAP_BYTES: usize = 64 * 1024;
/// Extra heap in the RAM the bootloader hands back after boot.
const RECLAIMED_HEAP_BYTES: usize = 64 * 1024;
const NET_SOCKETS: usize = 3;

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

/// A static random BLE address derived from the MAC, so bonds survive reboots.
fn ble_address() -> [u8; 6] {
    let mut address = [0u8; 6];
    address.copy_from_slice(esp_hal::efuse::base_mac_address().as_bytes());
    address[5] |= 0xC0;
    address
}

/// Starts Wi-Fi and the network stack; `None` when the radio does not come up.
fn start_wifi(spawner: &Spawner, wifi: esp_hal::peripherals::WIFI<'static>) -> Option<Stack<'static>> {
    static RESOURCES: StaticCell<StackResources<NET_SOCKETS>> = StaticCell::new();
    let controller = WifiController::new(wifi, ControllerConfig::default()).ok()?;
    let rng = Rng::new();
    let seed = u64::from(rng.random()) << 32 | u64::from(rng.random());
    let config = embassy_net::Config::dhcpv4(Default::default());
    let (stack, runner) = embassy_net::new(Interface::station(), config, RESOURCES.init(StackResources::new()), seed);
    spawner.spawn(wifi::wifi_task(controller).ok()?);
    spawner.spawn(wifi::net_runner_task(runner).ok()?);
    Some(stack)
}

fn spawn_sensors(spawner: &Spawner, usb: USB_DEVICE<'static>, adc1: ADC1<'static>, batt_sense: GPIO1<'static>) {
    let (rx, tx) = UsbSerialJtag::new(usb).into_async().split();
    spawner.spawn(usb::usb_rx_task(rx).unwrap());
    spawner.spawn(usb::usb_tx_task(tx).unwrap());
    spawner.spawn(sensors::power_task(adc1, batt_sense).unwrap());
    spawner.spawn(touch::touch_task().unwrap());
}

fn spawn_outputs(spawner: &Spawner, i2c0: I2C0<'static>, spi2: SPI2<'static>) {
    if let Some(i2c) = haptic::bus(i2c0) {
        spawner.spawn(haptic::haptic_task(i2c).unwrap());
    }
    if let Some(panel) = display::panel(spi2) {
        spawner.spawn(display::display_task(panel).unwrap());
    }
}

#[esp_hal::main]
async fn main(spawner: Spawner) {
    let p = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: RECLAIMED_HEAP_BYTES);
    esp_alloc::heap_allocator!(size: HEAP_BYTES);
    let timg0 = TimerGroup::new(p.TIMG0);
    esp_rtos::start(timg0.timer0, p.FROM_CPU_INTR0);

    let settings = flash::Settings::open(p.FLASH);
    let stored = settings.as_ref().map(|s| s.stored.clone()).unwrap_or_default();
    if let Some(cfg) = stored.config.clone() {
        CONFIG_WATCH.sender().send(cfg);
    }
    if let Some(settings) = settings {
        spawner.spawn(flash::storage_task(settings).unwrap());
    }
    spawner.spawn(network_task(hardware_id(), stored.config).unwrap());
    if let Ok(connector) = BleConnector::new(p.BT, Default::default()) {
        spawner.spawn(ble::ble_task(connector, hardware_id(), ble_address(), stored.bond).unwrap());
    }
    spawn_sensors(&spawner, p.USB_DEVICE, p.ADC1, p.GPIO1);
    spawn_outputs(&spawner, p.I2C0, p.SPI2);
    if let Some(stack) = start_wifi(&spawner, p.WIFI) {
        spawner.spawn(wss::wss_task(stack).unwrap());
    }
}

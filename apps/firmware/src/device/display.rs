//! GC9A01 240x240 round LCD on the `LCD_*` nets, with the backlight switched by `LCD_BL`.

use super::board_pin;
use crate::avatar::AvatarRenderer;
use crate::link::AVATAR_SIGNAL;
use crate::pins::BOARD_PINS;
use alloc::boxed::Box;
use embassy_time::{Duration, Timer};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::peripherals::SPI2;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::time::Rate;
use esp_hal::Blocking;
use mipidsi::interface::SpiInterface;
use mipidsi::models::GC9A01;
use mipidsi::options::ColorInversion;
use mipidsi::Builder;

/// One frame every 100 ms: a full redraw takes about 25 ms of blocking SPI at 40 MHz.
const FRAME_PERIOD: Duration = Duration::from_millis(100);

type LcdBus = ExclusiveDevice<Spi<'static, Blocking>, Output<'static>, Delay>;
type Lcd = mipidsi::Display<SpiInterface<'static, LcdBus, Output<'static>>, GC9A01, Output<'static>>;

/// The initialized panel and the backlight output that keeps it lit.
pub struct Panel {
    lcd: Lcd,
    _backlight: Output<'static>,
}

fn output(gpio: u8, level: Level) -> Output<'static> {
    Output::new(board_pin(gpio), level, OutputConfig::default())
}

/// Initializes the panel and turns the backlight on; `None` when the SPI bus cannot be set up.
pub fn panel(spi2: SPI2<'static>) -> Option<Panel> {
    let pins = BOARD_PINS.general;
    let spi = Spi::new(spi2, Config::default().with_frequency(Rate::from_mhz(40)))
        .ok()?
        .with_sck(board_pin(pins.lcd_sclk))
        .with_mosi(board_pin(pins.lcd_mosi));
    let device = ExclusiveDevice::new(spi, output(pins.lcd_cs, Level::High), Delay::new()).ok()?;
    let buffer = Box::leak(Box::new([0u8; 512]));
    let interface = SpiInterface::new(device, output(pins.lcd_dc, Level::Low), buffer);
    let lcd = Builder::new(GC9A01, interface)
        .reset_pin(output(pins.lcd_rst, Level::High))
        .invert_colors(ColorInversion::Inverted)
        .init(&mut Delay::new())
        .ok()?;
    Some(Panel { lcd, _backlight: output(pins.lcd_bl, Level::High) })
}

/// Draws the avatar, switching face as soon as the network task signals a new one.
#[embassy_executor::task]
pub async fn display_task(mut panel: Panel) {
    let mut renderer = AvatarRenderer::new();
    loop {
        if let Some((state, emotion)) = AVATAR_SIGNAL.try_take() {
            renderer.state = state;
            renderer.emotion = emotion;
        }
        renderer.step();
        let _ = renderer.render(&mut panel.lcd);
        Timer::after(FRAME_PERIOD).await;
    }
}

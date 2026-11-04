use crate::link::Overlay;
use core::fmt::Write;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Text};

const CENTER: Point = Point::new(120, 120);
const RING_COLOR: Rgb565 = Rgb565::new(0, 50, 31);

/// Draws the pairing ring or the numeric comparison code on top of the avatar.
pub fn draw_overlay<D: DrawTarget<Color = Rgb565>>(overlay: Overlay, display: &mut D) -> Result<(), D::Error> {
    match overlay {
        Overlay::None => Ok(()),
        Overlay::PairingRing => Circle::with_center(CENTER, 228)
            .into_styled(PrimitiveStyle::with_stroke(RING_COLOR, 4))
            .draw(display),
        Overlay::Passkey(code) => {
            let mut digits: heapless::String<8> = heapless::String::new();
            let _ = write!(digits, "{:06}", code % 1_000_000);
            Rectangle::with_center(CENTER, Size::new(100, 34))
                .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
                .draw(display)?;
            let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
            Text::with_alignment(&digits, CENTER + Point::new(0, 7), style, Alignment::Center).draw(display)?;
            Ok(())
        }
    }
}

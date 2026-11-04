use crate::trig::{cos_deg, sin_deg};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    Drawable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarState {
    #[default]
    Idle,
    Listening,
    Thinking,
    Speaking,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarEmotion {
    #[default]
    Neutral,
    Happy,
    Focused,
    Calm,
    Curious,
    Warn,
}

#[derive(Default)]
pub struct AvatarRenderer {
    pub state: AvatarState,
    pub emotion: AvatarEmotion,
    pub tick: u32,
    pub audio_level: u8,
}

impl AvatarRenderer {
    pub const fn new() -> Self {
        Self {
            state: AvatarState::Idle,
            emotion: AvatarEmotion::Neutral,
            tick: 0,
            audio_level: 0,
        }
    }

    pub fn step(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    pub fn render<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        let bg = PrimitiveStyle::with_fill(Rgb565::BLACK);
        Rectangle::new(Point::new(0, 0), Size::new(240, 240))
            .into_styled(bg)
            .draw(display)?;

        match self.state {
            AvatarState::Idle => self.render_idle(display),
            AvatarState::Listening => self.render_listening(display),
            AvatarState::Thinking => self.render_thinking(display),
            AvatarState::Speaking => self.render_speaking(display),
            AvatarState::Error => self.render_error(display),
        }
    }

    fn render_idle<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        let center = Point::new(120, 120);
        let pulse = (sin_deg((self.tick * 3) as i32) * 8) / 1000;
        let outer_color = match self.emotion {
            AvatarEmotion::Happy => Rgb565::new(10, 55, 30),
            AvatarEmotion::Warn => Rgb565::new(31, 30, 5),
            _ => Rgb565::new(4, 45, 28),
        };
        let halo_style = PrimitiveStyleBuilder::new().stroke_color(outer_color).stroke_width(2).build();
        Circle::with_center(center, ((48 + pulse) * 2) as u32).into_styled(halo_style).draw(display)?;
        Circle::with_center(center, ((36 + pulse) * 2) as u32)
            .into_styled(PrimitiveStyle::with_fill(outer_color))
            .draw(display)?;
        let eye_style = PrimitiveStyle::with_fill(Rgb565::WHITE);
        let eye_d = if (self.tick % 150) < 6 { 2 } else { 8 };
        Circle::with_center(Point::new(108, 116), eye_d).into_styled(eye_style).draw(display)?;
        Circle::with_center(Point::new(132, 116), eye_d).into_styled(eye_style).draw(display)
    }

    fn render_listening<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        let center = Point::new(120, 120);
        let ring_color = Rgb565::new(0, 48, 31);
        for i in 0..3 {
            let radius = ((self.tick * 2 + i * 40) % 90) + 20;
            let stroke = if radius > 80 { 1 } else { 2 };
            let style = PrimitiveStyleBuilder::new().stroke_color(ring_color).stroke_width(stroke).build();
            Circle::with_center(center, radius * 2).into_styled(style).draw(display)?;
        }
        let core_r = 28 + (self.audio_level as u32) / 8;
        Circle::with_center(center, core_r * 2)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::new(4, 58, 31)))
            .draw(display)?;
        let eye_style = PrimitiveStyle::with_fill(Rgb565::WHITE);
        Circle::with_center(Point::new(112, 120), 6).into_styled(eye_style).draw(display)?;
        Circle::with_center(Point::new(128, 120), 6).into_styled(eye_style).draw(display)
    }

    fn render_thinking<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        let center = Point::new(120, 120);
        let orbit_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::new(10, 30, 25))
            .stroke_width(1)
            .build();
        Circle::with_center(center, 120).into_styled(orbit_style).draw(display)?;
        Circle::with_center(center, 40)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::new(4, 40, 20)))
            .draw(display)?;
        let planet_style = PrimitiveStyle::with_fill(Rgb565::new(25, 60, 31));
        for i in 0..3 {
            let angle = ((self.tick * 6 + i * 120) % 360) as i32;
            let pos = Point::new(120 + (cos_deg(angle) * 60) / 1000, 120 + (sin_deg(angle) * 60) / 1000);
            Circle::with_center(pos, 12).into_styled(planet_style).draw(display)?;
        }
        Ok(())
    }

    fn render_speaking<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        let center = Point::new(120, 120);
        Circle::with_center(center, 80)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::new(15, 45, 25)))
            .draw(display)?;
        let eye_style = PrimitiveStyle::with_fill(Rgb565::WHITE);
        Circle::with_center(Point::new(108, 110), 8).into_styled(eye_style).draw(display)?;
        Circle::with_center(Point::new(132, 110), 8).into_styled(eye_style).draw(display)?;
        let bar_style = PrimitiveStyle::with_stroke(Rgb565::new(31, 45, 10), 3);
        for i in 0..5 {
            let x = 104 + i * 8;
            let phase = (self.tick * 15 + i as u32 * 45) as i32;
            let h = 4 + ((sin_deg(phase).abs() * 16) / 1000);
            Line::new(Point::new(x, 135 - h / 2), Point::new(x, 135 + h / 2))
                .into_styled(bar_style)
                .draw(display)?;
        }
        Ok(())
    }

    fn render_error<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        let center = Point::new(120, 120);
        let pulse = (sin_deg((self.tick * 8) as i32) * 6) / 1000;
        let halo_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::new(31, 6, 6))
            .stroke_width(3)
            .build();
        Circle::with_center(center, ((44 + pulse) * 2) as u32).into_styled(halo_style).draw(display)?;
        Circle::with_center(center, 60)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::new(20, 4, 4)))
            .draw(display)?;
        let cross_style = PrimitiveStyle::with_stroke(Rgb565::WHITE, 3);
        Line::new(Point::new(110, 110), Point::new(130, 130)).into_styled(cross_style).draw(display)?;
        Line::new(Point::new(130, 110), Point::new(110, 130)).into_styled(cross_style).draw(display)
    }
}

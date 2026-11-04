#![allow(dead_code)]

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
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

pub struct AvatarRenderer {
    state: AvatarState,
    emotion: AvatarEmotion,
    tick: u32,
    audio_level: u8,
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

    pub fn set_state(&mut self, state: AvatarState) {
        self.state = state;
    }

    pub fn state(&self) -> AvatarState {
        self.state
    }

    pub fn set_emotion(&mut self, emotion: AvatarEmotion) {
        self.emotion = emotion;
    }

    pub fn emotion(&self) -> AvatarEmotion {
        self.emotion
    }

    pub fn set_audio_level(&mut self, level: u8) {
        self.audio_level = level;
    }

    pub fn tick(&self) -> u32 {
        self.tick
    }

    pub fn step(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    pub fn render<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        let bg = PrimitiveStyle::with_fill(Rgb565::BLACK);
        Rectangle::new(Point::new(0, 0), embedded_graphics::geometry::Size::new(240, 240))
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
        let outer_radius = 48 + pulse;
        let inner_radius = 36 + pulse;

        let outer_color = match self.emotion {
            AvatarEmotion::Happy => Rgb565::new(10, 55, 30),
            AvatarEmotion::Warn => Rgb565::new(31, 30, 5),
            _ => Rgb565::new(4, 45, 28),
        };

        let halo_style = PrimitiveStyleBuilder::new()
            .stroke_color(outer_color)
            .stroke_width(2)
            .build();

        Circle::with_center(center, (outer_radius * 2) as u32)
            .into_styled(halo_style)
            .draw(display)?;

        let core_style = PrimitiveStyle::with_fill(outer_color);
        Circle::with_center(center, (inner_radius * 2) as u32)
            .into_styled(core_style)
            .draw(display)?;

        let is_blinking = (self.tick % 150) < 6;
        let eye_color = Rgb565::WHITE;

        if is_blinking {
            let eye_style = PrimitiveStyle::with_stroke(eye_color, 2);
            Line::new(Point::new(102, 116), Point::new(114, 116))
                .into_styled(eye_style)
                .draw(display)?;
            Line::new(Point::new(126, 116), Point::new(138, 116))
                .into_styled(eye_style)
                .draw(display)?;
        } else {
            let eye_style = PrimitiveStyle::with_fill(eye_color);
            Circle::with_center(Point::new(108, 116), 8)
                .into_styled(eye_style)
                .draw(display)?;
            Circle::with_center(Point::new(132, 116), 8)
                .into_styled(eye_style)
                .draw(display)?;
        }

        Ok(())
    }

    fn render_listening<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        let center = Point::new(120, 120);
        let ring_color = Rgb565::new(0, 48, 31);
        let core_color = Rgb565::new(4, 58, 31);

        for i in 0..3 {
            let offset = i * 40;
            let radius = ((self.tick * 2 + offset) % 90) + 20;
            let stroke_width = if radius > 80 { 1 } else { 2 };
            let style = PrimitiveStyleBuilder::new()
                .stroke_color(ring_color)
                .stroke_width(stroke_width)
                .build();

            Circle::with_center(center, radius * 2)
                .into_styled(style)
                .draw(display)?;
        }

        let audio_boost = (self.audio_level as u32) / 8;
        let core_radius = 28 + audio_boost;
        let core_style = PrimitiveStyle::with_fill(core_color);
        Circle::with_center(center, core_radius * 2)
            .into_styled(core_style)
            .draw(display)?;

        let eye_style = PrimitiveStyle::with_fill(Rgb565::WHITE);
        Circle::with_center(Point::new(112, 120), 6)
            .into_styled(eye_style)
            .draw(display)?;
        Circle::with_center(Point::new(128, 120), 6)
            .into_styled(eye_style)
            .draw(display)?;

        Ok(())
    }

    fn render_thinking<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        let center = Point::new(120, 120);
        let orbit_color = Rgb565::new(10, 30, 25);
        let dot_color = Rgb565::new(25, 60, 31);

        let orbit_style = PrimitiveStyleBuilder::new()
            .stroke_color(orbit_color)
            .stroke_width(1)
            .build();

        Circle::with_center(center, 120)
            .into_styled(orbit_style)
            .draw(display)?;

        let core_style = PrimitiveStyle::with_fill(Rgb565::new(4, 40, 20));
        Circle::with_center(center, 40)
            .into_styled(core_style)
            .draw(display)?;

        for i in 0..3 {
            let angle = ((self.tick * 6 + i * 120) % 360) as i32;
            let dx = (cos_deg(angle) * 60) / 1000;
            let dy = (sin_deg(angle) * 60) / 1000;
            let planet_pos = Point::new(120 + dx, 120 + dy);

            let planet_style = PrimitiveStyle::with_fill(dot_color);
            Circle::with_center(planet_pos, 12)
                .into_styled(planet_style)
                .draw(display)?;
        }

        Ok(())
    }

    fn render_speaking<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        let center = Point::new(120, 120);
        let mouth_color = Rgb565::new(31, 45, 10);
        let face_color = Rgb565::new(15, 45, 25);

        let face_style = PrimitiveStyle::with_fill(face_color);
        Circle::with_center(center, 80)
            .into_styled(face_style)
            .draw(display)?;

        let eye_style = PrimitiveStyle::with_fill(Rgb565::WHITE);
        Circle::with_center(Point::new(108, 110), 8)
            .into_styled(eye_style)
            .draw(display)?;
        Circle::with_center(Point::new(132, 110), 8)
            .into_styled(eye_style)
            .draw(display)?;

        let bar_style = PrimitiveStyle::with_stroke(mouth_color, 3);
        for i in 0..5 {
            let x = 104 + i * 8;
            let phase = (self.tick * 15 + i as u32 * 45) as i32;
            let height = 4 + ((sin_deg(phase).abs() * 16) / 1000);
            let y_top = 135 - height / 2;
            let y_bottom = 135 + height / 2;

            Line::new(Point::new(x, y_top), Point::new(x, y_bottom))
                .into_styled(bar_style)
                .draw(display)?;
        }

        Ok(())
    }

    fn render_error<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        let center = Point::new(120, 120);
        let pulse = (sin_deg((self.tick * 8) as i32) * 6) / 1000;
        let radius = 44 + pulse;

        let error_red = Rgb565::new(31, 6, 6);
        let halo_style = PrimitiveStyleBuilder::new()
            .stroke_color(error_red)
            .stroke_width(3)
            .build();

        Circle::with_center(center, (radius * 2) as u32)
            .into_styled(halo_style)
            .draw(display)?;

        let core_style = PrimitiveStyle::with_fill(Rgb565::new(20, 4, 4));
        Circle::with_center(center, 60)
            .into_styled(core_style)
            .draw(display)?;

        let cross_style = PrimitiveStyle::with_stroke(Rgb565::WHITE, 3);
        Line::new(Point::new(110, 110), Point::new(130, 130))
            .into_styled(cross_style)
            .draw(display)?;
        Line::new(Point::new(130, 110), Point::new(110, 130))
            .into_styled(cross_style)
            .draw(display)?;

        Ok(())
    }
}

fn sin_deg(deg: i32) -> i32 {
    let d = deg.rem_euclid(360);
    match d {
        0..=90 => sin_quarter(d),
        91..=180 => sin_quarter(180 - d),
        181..=270 => -sin_quarter(d - 180),
        _ => -sin_quarter(360 - d),
    }
}

fn cos_deg(deg: i32) -> i32 {
    sin_deg(deg + 90)
}

fn sin_quarter(d: i32) -> i32 {
    const TABLE: [i32; 7] = [0, 259, 500, 707, 866, 966, 1000];
    const ANGLES: [i32; 7] = [0, 15, 30, 45, 60, 75, 90];

    for i in 0..6 {
        if d >= ANGLES[i] && d <= ANGLES[i + 1] {
            let span = ANGLES[i + 1] - ANGLES[i];
            let frac = d - ANGLES[i];
            let val_span = TABLE[i + 1] - TABLE[i];
            return TABLE[i] + (val_span * frac) / span;
        }
    }
    1000
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::{geometry::Size, Pixel};

    struct TestBuffer {
        drawn_pixels: usize,
    }

    impl TestBuffer {
        fn new() -> Self {
            Self { drawn_pixels: 0 }
        }
    }

    impl DrawTarget for TestBuffer {
        type Color = Rgb565;
        type Error = core::convert::Infallible;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            for _ in pixels {
                self.drawn_pixels += 1;
            }
            Ok(())
        }
    }

    impl embedded_graphics::geometry::Dimensions for TestBuffer {
        fn bounding_box(&self) -> Rectangle {
            Rectangle::new(Point::new(0, 0), Size::new(240, 240))
        }
    }

    #[test]
    fn test_avatar_states_render() {
        let mut renderer = AvatarRenderer::new();
        assert_eq!(renderer.state(), AvatarState::Idle);

        let mut buf = TestBuffer::new();
        renderer.render(&mut buf).unwrap();
        assert!(buf.drawn_pixels > 0);

        for state in [
            AvatarState::Idle,
            AvatarState::Listening,
            AvatarState::Thinking,
            AvatarState::Speaking,
            AvatarState::Error,
        ] {
            renderer.set_state(state);
            renderer.step();
            let mut b = TestBuffer::new();
            renderer.render(&mut b).unwrap();
            assert!(b.drawn_pixels > 0);
        }
    }

    #[test]
    fn test_sin_cos() {
        assert_eq!(sin_deg(0), 0);
        assert_eq!(sin_deg(90), 1000);
        assert_eq!(sin_deg(180), 0);
        assert_eq!(sin_deg(270), -1000);
        assert_eq!(cos_deg(0), 1000);
        assert_eq!(cos_deg(90), 0);
    }
}

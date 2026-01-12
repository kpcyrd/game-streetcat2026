use crate::gfx;
use embedded_graphics::{
    Drawable,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    text::{Baseline, renderer::TextRenderer},
};

// Create style once as a const
const STYLE: MonoTextStyle<'static, BinaryColor> = MonoTextStyle::new(&gfx::FONT, BinaryColor::On);

pub struct Text<'a> {
    /// The string.
    pub text: &'a str,
    /// The position.
    pub position: Point,
}

impl<'a> Text<'a> {
    pub const fn new(text: &'a str, position: Point) -> Self {
        Text { text, position }
    }
}

impl Drawable for Text<'_> {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D: DrawTarget<Color = Self::Color>>(
        &self,
        display: &mut D,
    ) -> Result<Self::Output, D::Error> {
        STYLE.draw_string(self.text, self.position, Baseline::Top, display)?;
        Ok(())
    }
}

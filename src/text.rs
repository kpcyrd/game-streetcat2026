use crate::gfx;
use embedded_graphics::{
    Drawable,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    text::{Baseline, renderer::TextRenderer},
};

pub struct Text(embedded_graphics::text::Text<'static, MonoTextStyle<'static, BinaryColor>>);

impl Text {
    pub const fn with_baseline(
        text: &'static str,
        position: Point,
        _style: MonoTextStyle<'static, BinaryColor>,
        _baseline: Baseline,
    ) -> Self {
        let style = MonoTextStyle::new(&gfx::FONT, BinaryColor::On);
        let baseline = Baseline::Top;

        Text(embedded_graphics::text::Text::with_baseline(
            text, position, style, baseline,
        ))
    }
}

impl Drawable for Text {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D: DrawTarget<Color = Self::Color>>(
        &self,
        display: &mut D,
    ) -> Result<Self::Output, D::Error> {
        self.0
            .character_style
            .draw_string(self.0.text, self.0.position, Baseline::Top, display)?;
        Ok(())
    }
}

use crate::gfx;
use core::fmt;
use embedded_graphics::{
    Drawable,
    image::Image,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    text::{Baseline, Text},
};

pub struct Fishing {}

impl Fishing {
    pub const fn new() -> Self {
        Fishing {}
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: fmt::Debug,
    {
        Text::with_baseline(
            "Fishing!",
            Point::new(64, 0),
            MonoTextStyle::new(&gfx::FONT, BinaryColor::On),
            Baseline::Top,
        )
        .draw(display)
        .unwrap();

        Image::new(&gfx::CAT, Point::new(4, 16))
            .draw(display)
            .unwrap();
    }
}

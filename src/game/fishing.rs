use crate::{
    game::{Game, campaign::Campaign},
    gfx,
    input::Event,
};
use core::fmt;
use embedded_graphics::{
    Drawable,
    image::Image,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    text::{Baseline, Text},
};
use embedded_savegame::storage::Flash;

pub struct Fishing {}

impl Fishing {
    pub const fn new() -> Self {
        Fishing {}
    }

    pub fn event<F: Flash>(&mut self, event: Event, campaign: &mut Campaign<F>) {
        match event {
            Event::Up => (),
            Event::Down => (),
            Event::A => {
                campaign.money = campaign.money.saturating_add(10);
                campaign.write_savegame();
            }
            Event::B => {
                campaign.next_scene = Some(Game::shop());
            }
        }
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>, F: Flash>(
        &self,
        display: &mut D,
        campaign: &Campaign<F>,
    ) where
        <D as DrawTarget>::Error: fmt::Debug,
    {
        let mut buf = itoa::Buffer::new();
        let txt = buf.format(campaign.money);

        Text::with_baseline(
            txt,
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

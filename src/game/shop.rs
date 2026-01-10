use crate::{
    game::{Game, campaign::Campaign, fishing},
    gfx,
    input::Event,
    text::Text,
};
use embedded_graphics::{
    Drawable,
    image::Image,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    text::Baseline,
};
use embedded_savegame::storage::Flash;

pub struct Shop {}

impl Shop {
    pub const fn new() -> Self {
        Shop {}
    }

    pub const fn event<F: Flash>(&mut self, event: Event, campaign: &mut Campaign<F>) {
        match event {
            Event::Up => (),
            Event::Down => (),
            Event::A => (),
            Event::B => {
                campaign.next_scene = Some(Game::fishing(fishing::Timer::Random));
            }
        }
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D) {
        Text::with_baseline(
            "shop!",
            Point::new(64, 0),
            MonoTextStyle::new(&gfx::FONT, BinaryColor::On),
            Baseline::Top,
        )
        .draw(display)
        .ok();

        Image::new(&gfx::CAT, Point::new(4, 16)).draw(display).ok();
    }
}

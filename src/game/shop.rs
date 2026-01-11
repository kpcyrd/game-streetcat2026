use crate::{
    game::{Game, campaign::Campaign, fishing},
    gfx,
    input::Event,
    text::Text,
};
use embedded_graphics::{
    Drawable,
    image::Image,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
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
        Text::new("shop!", Point::new(64, 0)).draw(display).ok();

        Image::new(&gfx::CAT, Point::new(4, 16)).draw(display).ok();
    }
}

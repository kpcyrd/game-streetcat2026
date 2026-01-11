use crate::{
    game::{Unlocks, campaign::Campaign},
    input::Event,
    text::Text,
};
use embedded_graphics::{
    Drawable,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
};
use embedded_savegame::storage::Flash;

pub struct Story {
    lines: &'static str,
    unlock: Unlocks,
}

impl Story {
    pub const fn new(lines: &'static str, unlock: Unlocks) -> Self {
        Story { lines, unlock }
    }

    // State machine functions
    pub fn event<F: Flash>(&self, event: Event, campaign: &mut Campaign<F>) {
        match event {
            Event::Up => (),
            Event::Down => (),
            Event::A => {
                campaign.unlocks.insert(self.unlock);
                campaign.init_next();
            }
            Event::B => {}
        }
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D) {
        Text::new(self.lines, Point::zero())
            .draw(display)
            .ok();
    }
}

use crate::{
    game::{Unlocks, campaign::Campaign},
    gfx::{self, FONT_HEIGHT},
    input::Event,
};
use core::fmt;
use embedded_graphics::{
    Drawable,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    text::{Baseline, Text},
};
use embedded_savegame::storage::Flash;

pub struct Story {
    lines: &'static [&'static str],
    unlock: Unlocks,
}

impl Story {
    pub const fn new(lines: &'static [&'static str], unlock: Unlocks) -> Self {
        Story { lines, unlock }
    }

    // State machine functions
    pub fn event<F: Flash>(&mut self, event: Event, campaign: &mut Campaign<F>) {
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

    pub fn render<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: fmt::Debug,
    {
        for (i, line) in self.lines.iter().enumerate() {
            let point = Point::new(0, FONT_HEIGHT * i as i32);
            Text::with_baseline(
                line,
                point,
                MonoTextStyle::new(&gfx::FONT, BinaryColor::On),
                Baseline::Top,
            )
            .draw(display)
            .unwrap();
        }
    }
}

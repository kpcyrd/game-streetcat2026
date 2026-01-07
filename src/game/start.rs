use crate::{
    game::campaign::Campaign,
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

const CURSOR_LEFT_PAD: i32 = (gfx::FONT.character_size.width as i32) * 2;
const ITEM_LEFT_PAD: i32 = CURSOR_LEFT_PAD + (gfx::FONT.character_size.width as i32) * 2;
const ITEM_1: Point = Point::new(ITEM_LEFT_PAD, FONT_HEIGHT);
const ITEM_2: Point = Point::new(ITEM_LEFT_PAD, FONT_HEIGHT * 2);

pub struct Start {
    item: bool,
}

impl Start {
    pub const fn new() -> Self {
        Start { item: false }
    }

    fn toggle(&mut self) {
        self.item = !self.item;
    }

    // State machine functions
    pub fn event<F: Flash>(&mut self, event: Event, campaign: &mut Campaign<F>) {
        match event {
            Event::Up => self.toggle(),
            Event::Down => self.toggle(),
            Event::A => {
                if !self.item {
                    // Continue selected
                } else {
                    // New Game selected
                    campaign.save_slot = None;
                }

                campaign.init();
            }
            Event::B => {}
        }
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: fmt::Debug,
    {
        // Continue
        Text::with_baseline(
            "Continue",
            ITEM_1,
            MonoTextStyle::new(&gfx::FONT, BinaryColor::On),
            Baseline::Top,
        )
        .draw(display)
        .unwrap();

        // New Game
        Text::with_baseline(
            "New Game",
            ITEM_2,
            MonoTextStyle::new(&gfx::FONT, BinaryColor::On),
            Baseline::Top,
        )
        .draw(display)
        .unwrap();

        // Cursor
        let point = if !self.item {
            Point::new(CURSOR_LEFT_PAD, ITEM_1.y)
        } else {
            Point::new(CURSOR_LEFT_PAD, ITEM_2.y)
        };
        Text::with_baseline(
            ">",
            point,
            MonoTextStyle::new(&gfx::FONT, BinaryColor::On),
            Baseline::Top,
        )
        .draw(display)
        .unwrap();

        // Help
        Text::with_baseline(
            "B: Cancel | A: Select",
            Point::new(2, 64 - FONT_HEIGHT),
            MonoTextStyle::new(&gfx::FONT, BinaryColor::On),
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    }
}

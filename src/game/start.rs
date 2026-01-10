use crate::{
    game::campaign::Campaign,
    gfx::{self, FONT_HEIGHT},
    input::Event,
    text::Text,
};
use embedded_graphics::{
    Drawable,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    text::Baseline,
};
use embedded_savegame::storage::Flash;

const CURSOR_LEFT_PAD: i32 = (gfx::FONT.character_size.width as i32) * 2;
const ITEM_LEFT_PAD: i32 = CURSOR_LEFT_PAD + (gfx::FONT.character_size.width as i32) * 2;
const ITEM_1: Point = Point::new(ITEM_LEFT_PAD, FONT_HEIGHT);
const ITEM_2: Point = Point::new(ITEM_LEFT_PAD, FONT_HEIGHT * 2);

pub struct Start {
    new_game: bool,
}

impl Start {
    pub const fn new() -> Self {
        Start { new_game: false }
    }

    const fn toggle(&mut self) {
        self.new_game = !self.new_game;
    }

    // State machine functions
    pub fn event<F: Flash>(&mut self, event: Event, campaign: &mut Campaign<F>) {
        match event {
            Event::Up => self.toggle(),
            Event::Down => self.toggle(),
            Event::A => {
                if self.new_game {
                    campaign.save_slot = None;
                }
                campaign.init();
            }
            Event::B => {}
        }
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D) {
        // Continue
        Text::with_baseline(
            "Continue",
            ITEM_1,
            MonoTextStyle::new(&gfx::FONT, BinaryColor::On),
            Baseline::Top,
        )
        .draw(display)
        .ok();

        // New Game
        Text::with_baseline(
            "New Game",
            ITEM_2,
            MonoTextStyle::new(&gfx::FONT, BinaryColor::On),
            Baseline::Top,
        )
        .draw(display)
        .ok();

        // Cursor
        let point = if !self.new_game {
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
        .ok();

        // Help
        Text::with_baseline(
            "B: Cancel | A: Select",
            Point::new(2, 64 - FONT_HEIGHT),
            MonoTextStyle::new(&gfx::FONT, BinaryColor::On),
            Baseline::Top,
        )
        .draw(display)
        .ok();
    }
}

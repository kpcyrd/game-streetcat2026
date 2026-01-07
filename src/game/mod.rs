pub mod campaign;
pub mod fishing;
pub mod shop;
pub mod start;
pub mod story;

use crate::{game::campaign::Campaign, input::Event};
use bitflags::bitflags;
use core::fmt;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};
use embedded_savegame::storage::Flash;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Unlocks: u32 {
        const STORY_INTRO = 0b1 << 0;
        /*
        const FOO = 0b1 << 0;
        const BAR = 0b1 << 1;
        const ABC = 0b1 << 2;
        const DEF = 0b1 << 3;
        const XYZ = 0b1 << 4;
        */
    }
}

pub enum Game {
    Start(start::Start),
    Fishing(fishing::Fishing),
    Shop(shop::Shop),
    Story(story::Story),
}

impl Game {
    pub const fn start() -> Self {
        Game::Start(start::Start::new())
    }

    pub const fn fishing() -> Self {
        Game::Fishing(fishing::Fishing::new())
    }

    pub const fn shop() -> Self {
        Game::Shop(shop::Shop::new())
    }

    pub fn event<F: Flash>(&mut self, event: Event, campaign: &mut Campaign<F>) {
        match self {
            Game::Start(s) => s.event(event, campaign),
            Game::Fishing(f) => f.event(event, campaign),
            Game::Shop(s) => s.event(event, campaign),
            Game::Story(s) => s.event(event, campaign),
        }
    }

    pub fn tick<F: Flash>(&mut self, campaign: &mut Campaign<F>) {
        match self {
            Game::Start(_s) => (),
            Game::Fishing(_f) => (),
            Game::Shop(_s) => (),
            Game::Story(_s) => (),
        }
        campaign.feed_rng();
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>, F: Flash>(
        &self,
        display: &mut D,
        campaign: &Campaign<F>,
    ) where
        <D as DrawTarget>::Error: fmt::Debug,
    {
        match self {
            Game::Start(s) => s.render(display),
            Game::Fishing(f) => f.render(display, campaign),
            Game::Shop(s) => s.render(display),
            Game::Story(s) => s.render(display),
        }
    }
}

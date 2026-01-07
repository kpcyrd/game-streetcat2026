pub mod campaign;
pub mod fishing;
pub mod shop;
pub mod start;

use crate::{game::campaign::Campaign, input::Event};
use bitflags::bitflags;
use core::fmt;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};
use embedded_savegame::storage::Flash;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Unlocks: u32 {
        const FOO = 0b1 << 0;
        const BAR = 0b1 << 1;
        const ABC = 0b1 << 2;
        const DEF = 0b1 << 3;
        const XYZ = 0b1 << 4;
    }
}

pub enum Game {
    Start(start::Start),
    Fishing(fishing::Fishing),
    Shop(shop::Shop),
}

impl Game {
    pub const fn start() -> Self {
        Game::Start(start::Start::new())
    }

    pub const fn fishing() -> Self {
        Game::Fishing(fishing::Fishing::new())
    }

    pub fn event<F: Flash>(&mut self, event: Event, campaign: &mut Campaign<F>) {
        match self {
            Game::Start(s) => s.event(event, campaign),
            Game::Fishing(_f) => {}
            Game::Shop(_s) => {}
        }
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: fmt::Debug,
    {
        match self {
            Game::Start(s) => s.render(display),
            Game::Fishing(f) => f.render(display),
            Game::Shop(s) => s.render(display),
        }
    }
}

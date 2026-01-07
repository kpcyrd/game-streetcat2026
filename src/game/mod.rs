pub mod campaign;
pub mod fishing;
pub mod shop;
pub mod start;

use crate::{game::campaign::Campaign, input::Event};
use core::fmt;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};
use embedded_savegame::storage::Flash;

pub enum Game {
    Start(start::Start),
    Fishing(fishing::Fishing),
    Shop(shop::Shop),
}

impl Game {
    pub const fn start() -> Self {
        Game::Start(start::Start::new())
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

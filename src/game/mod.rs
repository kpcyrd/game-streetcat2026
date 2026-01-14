pub mod campaign;
pub mod fishing;
pub mod plot;
pub mod shop;
pub mod start;
pub mod story;

use crate::{game::campaign::Campaign, input::Event};
use bitflags::bitflags;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};
use embedded_savegame::storage::Flash;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Unlocks: u16 {
        // The first dialogue
        const STORY_INTRO = 0b1 << 0;
        // The email minigame
        const STORY_ESCAPED_CORPORATE = 0b1 << 1;

        // Upgraded rod
        const SHOP_UPGRADED_ROD = 0b1 << 3;
        const BOUGHT_UPGRADED_ROD = 0b1 << 4;

        // Upgraded bait
        const SHOP_BASIC_BAIT = 0b1 << 5;
        const BOUGHT_BASIC_BAIT = 0b1 << 6;

        const SHOP_TASTY_BAIT = 0b1 << 9;
        const BOUGHT_TASTY_BAIT = 0b1 << 10;

        const SHOP_PREMIUM_BAIT = 0b1 << 11;
        const BOUGHT_PREMIUM_BAIT = 0b1 << 12;

        const SHOP_HEAVENLY_BAIT = 0b1 << 13;
        const BOUGHT_HEAVENLY_BAIT = 0b1 << 14;

        // Upgraded rates
        const SHOP_BETTER_RATES = 0b1 << 7;
        const BOUGHT_BETTER_RATES = 0b1 << 8;

        // Final objective
        const KING_STATUS = 0b1 << 15;
    }
}

impl Unlocks {
    /// The first unlocked shop item
    pub const fn first_shop_unlock() -> Unlocks {
        Unlocks::SHOP_UPGRADED_ROD
    }

    pub const fn next_unlock(&self) -> Option<Unlocks> {
        if !self.contains(Unlocks::SHOP_UPGRADED_ROD) {
            Some(Unlocks::SHOP_UPGRADED_ROD)
        } else if !self.contains(Unlocks::SHOP_BASIC_BAIT) {
            Some(Unlocks::SHOP_BASIC_BAIT)
        } else if !self.contains(Unlocks::SHOP_BETTER_RATES) {
            Some(Unlocks::SHOP_BETTER_RATES)
        } else {
            None
        }
    }

    pub fn unlock_next(&mut self) {
        if let Some(unlock) = self.next_unlock() {
            self.insert(unlock);
        }
    }
}

pub enum Game {
    Start(start::Start),
    Fishing(fishing::Fishing),
    Shop(shop::Shop),
    Story(&'static story::Story),
}

impl Game {
    pub const fn start() -> Self {
        Game::Start(start::Start::new())
    }

    pub const fn fishing(timer: fishing::Timer) -> Self {
        Game::Fishing(fishing::Fishing::new(timer))
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
            Game::Fishing(f) => f.tick(campaign),
            Game::Shop(_s) => (),
            Game::Story(_s) => (),
        }
        campaign.feed_rng();
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>, F: Flash>(
        &self,
        display: &mut D,
        campaign: &Campaign<F>,
    ) {
        match self {
            Game::Start(s) => s.render(display),
            Game::Fishing(f) => f.render(display, campaign),
            Game::Shop(s) => s.render(display, campaign),
            Game::Story(s) => s.render(display),
        }
    }
}

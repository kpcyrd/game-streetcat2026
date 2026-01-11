use crate::{
    game::{Game, Unlocks, campaign::Campaign},
    gfx,
    input::Event,
    text::Text,
};
use core::cmp;
use embedded_graphics::{
    Drawable,
    image::Image,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
};
use embedded_savegame::storage::Flash;

const ESCAPE_THRESHOLD: i16 = -40;
// Align the max with the RNG mask for good distribution
const RNG_MASK: u32 = 0xFF;
// const MAX_WAIT_DURATION: i16 = 120;
const MAX_WAIT_DURATION: i16 = 12;

// With i16::MIN the fish would immediately escape, resetting the timer
// But with the email mingame, we want to start with a reasonable timer.
//
// Returning from the shop explicitly uses i16::MIN to prevent cheating.
const GOOD_START_VALUE: i16 = 35;

// The default position if we don't render any extras
const STANDARD_CAT_POSITION: Point = Point::new(8, 16);
// The size of the cat image
const CAT_HEIGHT: i32 = 30;
// The office cat offset (higher to make room for the necktie)
const OFFICE_CAT_OFFSET: Point = Point::new(0, 10);
// The necktie position relative to the office cat
const NECKTIE_OFFSET: Point = Point::new(17, CAT_HEIGHT + 1);

pub enum Timer {
    Random,
    Onboarding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Loot {
    Key,
    Bones,
    Fish,
}

impl Loot {
    pub const fn reward(&self) -> u16 {
        match self {
            Loot::Key => 0,
            Loot::Bones => 5,
            Loot::Fish => 10,
        }
    }

    pub const fn description(&self) -> &'static str {
        match self {
            Loot::Key => "Key!",
            Loot::Bones => "+5",
            Loot::Fish => "+10",
        }
    }
}

pub struct Fishing {
    spawn_timer: i16,
    caught: Option<Loot>,
}

impl Fishing {
    pub const fn new(timer: Timer) -> Self {
        Fishing {
            spawn_timer: match timer {
                Timer::Random => u16::MAX as i16,
                Timer::Onboarding => GOOD_START_VALUE,
            },
            caught: None,
        }
    }

    pub fn setup_spawn_timer<F: Flash>(&mut self, campaign: &mut Campaign<F>) {
        campaign.feed_rng();
        let num = ((campaign.rng & RNG_MASK) as i16 + 1) << 1;
        self.spawn_timer = cmp::min(num, MAX_WAIT_DURATION);
    }

    fn add_reward<F: Flash>(&mut self, reward: u16, campaign: &mut Campaign<F>) {
        campaign.money = campaign.money.saturating_add(reward);
        campaign.write_savegame();
        self.setup_spawn_timer(campaign);
    }

    pub fn event<F: Flash>(&mut self, event: Event, campaign: &mut Campaign<F>) {
        match event {
            Event::Up => (),
            Event::Down => (),
            Event::A => {
                // If we showed our successful catch, remove it now
                if let Some(loot) = self.caught.take() {
                    // Add reward, start new timer
                    if loot == Loot::Key {
                        campaign.unlocks.unlock_next();
                    }
                    self.add_reward(loot.reward(), campaign);
                } else if self.spawn_timer <= 0 {
                    // Caught fish!

                    if campaign.escaped_corporate() {
                        // TODO: randomize money reward
                        self.caught = Some(Loot::Bones);
                    } else {
                        self.add_reward(10, campaign);
                    }
                }
            }
            Event::B => {
                if campaign.escaped_corporate() {
                    campaign.next_scene = Some(Game::shop());
                } else if campaign.can_escape_corporate() {
                    // Escape corporate
                    campaign.money = 0;
                    campaign.unlocks.insert(Unlocks::STORY_ESCAPED_CORPORATE);
                    campaign.init_next();
                }
            }
        }
    }

    pub fn tick<F: Flash>(&mut self, campaign: &mut Campaign<F>) {
        if self.caught.is_some() {
            // Waiting for player to acknowledge catch
        } else if self.spawn_timer < ESCAPE_THRESHOLD && campaign.escaped_corporate() {
            // Fish escaped
            self.setup_spawn_timer(campaign);
        } else {
            // Decrease timer
            self.spawn_timer = self.spawn_timer.saturating_sub(1);
        }
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>, F: Flash>(
        &self,
        display: &mut D,
        campaign: &Campaign<F>,
    ) {
        let cat_point = if campaign.escaped_corporate() {
            if let Some(loot) = self.caught {
                // Show caught loot
                Text::new(loot.description(), Point::new(15, 0))
                    .draw(display)
                    .ok();
            }

            /*
            let mut buf = itoa::Buffer::new();
            // let txt = buf.format(campaign.money);
            let txt = buf.format(self.spawn_timer);
            */
            let txt = "TODO";
            Text::new(txt, Point::new(64, 0))
                .draw(display)
                .ok();

            // Fishing rod
            let mut point = Point::new(64, 16);
            if self.spawn_timer <= 0 && self.spawn_timer & 4 == 4 {
                point += Point::new(0, 4);
            }
            Image::new(&gfx::FISHING, point).draw(display).ok();

            // The cat position
            STANDARD_CAT_POSITION
        } else {
            // Email
            if self.spawn_timer <= 0 {
                Text::new("New Email!", Point::new(60, 8))
                    .draw(display)
                    .ok();
            }

            if campaign.can_escape_corporate() {
                Text::new("B: Escape", Point::new(2, 64 - gfx::FONT_HEIGHT))
                    .draw(display)
                    .ok();
            }

            Text::new("| A: Delete", Point::new(62, 64 - gfx::FONT_HEIGHT))
                .draw(display)
                .ok();

            // The cat position (slightly higher)
            let cat_position = STANDARD_CAT_POSITION - OFFICE_CAT_OFFSET;

            // Draw the necktie (height = 15px, width = 7px)
            let necktie_point = cat_position + NECKTIE_OFFSET;
            Image::new(&gfx::NECKTIE, necktie_point).draw(display).ok();

            // Return the postion and proceed rendering the cat
            cat_position
        };

        // Draw cat
        Image::new(&gfx::CAT, cat_point).draw(display).ok();
    }
}

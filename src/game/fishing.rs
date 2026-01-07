use crate::{
    game::{Game, campaign::Campaign},
    gfx,
    input::Event,
};
use core::{cmp, fmt};
use embedded_graphics::{
    Drawable,
    image::Image,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    text::{Baseline, Text},
};
use embedded_savegame::storage::Flash;

const ESCAPE_THRESHOLD: i16 = -40;
// Align the max with the RNG mask for good distribution
const RNG_MASK: u32 = 0xFF;
const MAX_WAIT_DURATION: i16 = 120;

pub struct Fishing {
    spawn_timer: i16,
}

impl Fishing {
    pub const fn new() -> Self {
        Fishing {
            spawn_timer: i16::MIN,
        }
    }

    pub fn setup_spawn_timer<F: Flash>(&mut self, campaign: &mut Campaign<F>) {
        campaign.feed_rng();
        let num = ((campaign.rng & RNG_MASK) as i16 + 1) << 1;
        self.spawn_timer = cmp::min(num, MAX_WAIT_DURATION);
    }

    pub fn event<F: Flash>(&mut self, event: Event, campaign: &mut Campaign<F>) {
        match event {
            Event::Up => (),
            Event::Down => (),
            Event::A => {
                if self.spawn_timer <= 0 {
                    // Caught fish!
                    // TODO: randomize money reward
                    campaign.money = campaign.money.saturating_add(10);
                    campaign.write_savegame();
                    self.setup_spawn_timer(campaign);
                }
            }
            Event::B => {
                campaign.next_scene = Some(Game::shop());
            }
        }
    }

    pub fn tick<F: Flash>(&mut self, campaign: &mut Campaign<F>) {
        if self.spawn_timer < ESCAPE_THRESHOLD {
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
        _campaign: &Campaign<F>,
    ) where
        <D as DrawTarget>::Error: fmt::Debug,
    {
        let mut buf = itoa::Buffer::new();
        // let txt = buf.format(campaign.money);
        let txt = buf.format(self.spawn_timer);

        Text::with_baseline(
            txt,
            Point::new(64, 0),
            MonoTextStyle::new(&gfx::FONT, BinaryColor::On),
            Baseline::Top,
        )
        .draw(display)
        .unwrap();

        Image::new(&gfx::CAT, Point::new(4, 16))
            .draw(display)
            .unwrap();
    }
}

use crate::{
    game::{Unlocks, campaign::Campaign},
    gfx,
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
    lines: &'static [&'static str],
    unlock: Unlocks,
}

impl Story {
    pub const fn new(lines: &'static [&'static str], unlock: Unlocks) -> Self {
        Story { lines, unlock }
    }

    /// This is used for compile-time sanity checks
    pub const fn longest_line(&self) -> usize {
        // Neither cmp::max nor iterators are const yet
        let mut max = 0;
        let mut i = 0;
        while i < self.lines.len() {
            let line_len = self.lines[i].len();
            if line_len > max {
                max = line_len;
            }
            i += 1;
        }
        max
    }

    /// This is used for compile-time sanity checks
    pub const fn num_lines(&self) -> usize {
        self.lines.len()
    }

    // State machine functions
    pub fn event<F: Flash>(&self, event: Event, campaign: &mut Campaign<F>) {
        match event {
            Event::Up => (),
            Event::Down => (),
            Event::A => {
                campaign.unlocks.insert(self.unlock);
                campaign.acknowledged_scenes.insert(self.unlock);
                campaign.init_next();
            }
            Event::B => {}
        }
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D) {
        let mut point = Point::new(0, 0);

        for line in self.lines {
            Text::new(line, point).draw(display).ok();
            point += Point::new(0, gfx::FONT_HEIGHT);
        }
    }
}

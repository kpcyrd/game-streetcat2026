use crate::{
    game::{Game, campaign::Campaign, fishing},
    gfx,
    input::Event,
    text::Text,
};
use embedded_graphics::{
    Drawable,
    // image::Image,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
};
use embedded_savegame::storage::Flash;

const MENU_LIMIT: usize = 3;
const CURSOR_LEFT_PAD: i32 = gfx::FONT_WIDTH * 2;

pub struct Shop {
    idx: usize,
}

impl Shop {
    pub const fn new() -> Self {
        Shop { idx: 0 }
    }

    pub fn event<F: Flash>(&mut self, event: Event, campaign: &mut Campaign<F>) {
        match event {
            Event::Up => self.idx = self.idx.checked_sub(1).unwrap_or(MENU_LIMIT - 1),
            Event::Down => self.idx = (self.idx + 1) % MENU_LIMIT,
            Event::A => (),
            Event::B => {
                campaign.next_scene = Some(Game::fishing(fishing::Timer::Random));
            }
        }
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D) {
        Text::new("Shop!", Point::new(0, 0)).draw(display).ok();

        let mut point = Point::new(0, gfx::FONT_HEIGHT * 2);
        for n in 0..MENU_LIMIT {
            if n == self.idx {
                Text::new(">", point).draw(display).ok();
            }

            let item_text = match n {
                0 => "Upgraded Rod - $100",
                1 => "Bait - $50",
                2 => "Better Rates - $200",
                _ => continue,
            };
            Text::new(item_text, point + Point::new(CURSOR_LEFT_PAD, 0))
                .draw(display)
                .ok();
            point.y += gfx::FONT_HEIGHT;
        }

        // Help
        Text::new("| A: Buy", Point::new(62, 64 - gfx::FONT_HEIGHT))
            .draw(display)
            .ok();

        // Image::new(&gfx::CAT, Point::new(4, 16)).draw(display).ok();
    }
}

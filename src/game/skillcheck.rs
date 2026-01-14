use crate::gfx;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Size},
    primitives::{Rectangle, StyledDrawable},
};

const OFFSET: Point = Point::new(112, 5);
const SIZE: Size = Size::new(8, 40);
const INNER_WIDTH: u32 = SIZE.width - 2;

pub struct Skillcheck {
    speed: u8,
    cursor: u8,
    size: u8,
    bottom_offset: u8,
}

impl Skillcheck {
    pub const fn new(speed: u8, size: u8) -> Self {
        Skillcheck {
            speed,
            cursor: 0,
            bottom_offset: (OFFSET.y as u32 + SIZE.height - size as u32 - 1) as u8,
            size,
        }
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D) {
        // Render box
        Rectangle::new(OFFSET, SIZE)
            .draw_styled(&gfx::WHITE, display)
            .ok();
        Rectangle::new(
            OFFSET + Point::new(1, 1),
            Size::new(INNER_WIDTH, self.size as u32),
        )
        .draw_styled(&gfx::BLACK, display)
        .ok();
        Rectangle::new(
            Point::new(OFFSET.x + 1, self.bottom_offset as i32),
            Size::new(INNER_WIDTH, self.size as u32),
        )
        .draw_styled(&gfx::BLACK, display)
        .ok();

        // Render cursor
        Rectangle::new(Point::new(122, 5), Size::new(6, 1))
            .draw_styled(&gfx::WHITE, display)
            .ok();
    }
}

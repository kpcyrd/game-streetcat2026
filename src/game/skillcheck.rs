use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};

const OFFSET: Point = Point::new(112, 5);
const SIZE: Size = Size::new(8, 40);
const INNER_HEIGHT: u32 = SIZE.height - 2;
const INNER_WIDTH: u32 = SIZE.width - 2;

pub const EASY: Skillcheck = Skillcheck::new(5, 14);
pub const MEDIUM: Skillcheck = Skillcheck::new(6, 15);
pub const HARD: Skillcheck = Skillcheck::new(4, 18);
pub const IMPOSSIBLE: Skillcheck = Skillcheck::new(8, 18);

pub struct Skillcheck {
    speed: u8,
    cursor: u8,
    padding: u8,
}

impl Skillcheck {
    pub const fn new(speed: u8, padding: u8) -> Self {
        // TODO: this is slightly risky because it may execute at runtime
        assert!(padding as u32 * 2 <= SIZE.height);
        Skillcheck {
            speed,
            cursor: 0,
            padding,
        }
    }

    pub const fn try_catch(&self) -> bool {
        // Keep in mind, the rendered box is offset by 1
        self.cursor > self.padding && self.cursor <= (SIZE.height as u8 - self.padding)
    }

    pub fn tick(&mut self) {
        self.cursor = (self.cursor + self.speed) % (SIZE.height as u8);
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D) {
        // Render box
        display
            .fill_solid(&Rectangle::new(OFFSET, SIZE), BinaryColor::On)
            .ok();
        display
            .fill_solid(
                &Rectangle::new(
                    OFFSET + Point::new(1, 1),
                    Size::new(INNER_WIDTH, INNER_HEIGHT),
                ),
                BinaryColor::Off,
            )
            .ok();

        // Render hitbox
        display
            .fill_solid(
                &Rectangle::new(
                    OFFSET + Point::new(1, 1 + self.padding as i32),
                    Size::new(INNER_WIDTH, INNER_HEIGHT - (self.padding as u32 * 2)),
                ),
                BinaryColor::On,
            )
            .ok();

        // Render cursor
        display
            .fill_solid(
                &Rectangle::new(Point::new(122, (5 + self.cursor) as i32), Size::new(6, 1)),
                BinaryColor::On,
            )
            .ok();
    }
}

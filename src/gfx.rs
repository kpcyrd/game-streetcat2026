use crate::text::Text;
use embedded_graphics::{
    Drawable,
    image::ImageRaw,
    mono_font::{MonoFont, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
};

pub const CAT: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../video/cat.raw"), 36);
pub const FISHING: ImageRaw<BinaryColor> =
    ImageRaw::new(include_bytes!("../video/fishingrod.raw"), 30);
pub const NECKTIE: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../video/necktie.raw"), 7);

pub const KEY: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../video/key.raw"), 10);
pub const FISHBONES: ImageRaw<BinaryColor> =
    ImageRaw::new(include_bytes!("../video/fishbones.raw"), 10);
pub const FISH: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../video/fish.raw"), 10);

pub const FONT: MonoFont = FONT_6X10;
pub const FONT_HEIGHT: i32 = FONT.character_size.height as i32;
pub const FONT_WIDTH: i32 = FONT.character_size.width as i32;

pub const LAST_LINE: i32 = 64 - FONT_HEIGHT;
pub const LAST_LINE_A: Point = Point::new(62, LAST_LINE);
pub const LAST_LINE_B: Point = Point::new(2, LAST_LINE);

// The position to render your current balance
const MONEY_POSITION: Point = Point::new(64, 0);

pub fn render_currency<D: DrawTarget<Color = BinaryColor>>(display: &mut D, point: Point) {
    Text::new("$", point).draw(display).ok();
}

pub fn render_balance<D: DrawTarget<Color = BinaryColor>>(display: &mut D, balance: u16) {
    render_currency(display, MONEY_POSITION);

    let mut buf = itoa::Buffer::new();
    let txt = buf.format(balance);
    Text::new(txt, MONEY_POSITION + Point::new(FONT_WIDTH, 0))
        .draw(display)
        .ok();
}

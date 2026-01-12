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

pub const FONT: MonoFont = FONT_6X10;
pub const FONT_HEIGHT: i32 = FONT.character_size.height as i32;
pub const FONT_WIDTH: i32 = FONT.character_size.width as i32;

pub fn render_currency<D: DrawTarget<Color = BinaryColor>>(display: &mut D, point: Point) {
    Text::new("$", point).draw(display).ok();
}

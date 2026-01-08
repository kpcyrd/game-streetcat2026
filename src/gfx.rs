use embedded_graphics::{
    image::ImageRaw,
    mono_font::{MonoFont, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
};

pub const CAT: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../video/cat.raw"), 36);
pub const FISHING: ImageRaw<BinaryColor> =
    ImageRaw::new(include_bytes!("../video/fishingrod.raw"), 30);
pub const FONT: MonoFont = FONT_6X10;
pub const FONT_HEIGHT: i32 = FONT.character_size.height as i32;

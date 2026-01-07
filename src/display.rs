use ch32_hal::{self as hal};
use ssd1306::{I2CDisplayInterface, Ssd1306, mode::BasicMode, prelude::*};

pub type Display<I2C> = Ssd1306<I2CInterface<I2C>, DisplaySize128x64, BasicMode>;

pub fn setup<I2C: embedded_hal::i2c::I2c>(i2c: I2C) -> Display<I2C> {
    hal::println!("Setting up oled");
    let interface = I2CDisplayInterface::new(i2c);
    Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180)
}

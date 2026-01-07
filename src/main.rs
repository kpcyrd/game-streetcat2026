#![no_std]
#![no_main]

mod display;
mod savegame;

use ch32_hal::{
    self as hal,
    delay::Delay,
    gpio::{self, Input},
    i2c::I2c,
    time::Hertz,
};
use core::cell::RefCell;
use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::Point,
    text::{Baseline, Text},
};
use embedded_hal_bus::i2c::RefCellDevice;
use panic_halt as _;
use ssd1306::prelude::*;
// use qingke::riscv;

#[qingke_rt::entry]
fn main() -> ! {
    // hal::debug::SDIPrint::enable();
    let p = hal::init(hal::Config::default());
    let mut delay = Delay;

    // wait for serial to connect
    delay.delay_ms(100);
    hal::println!("Hello world!");

    hal::println!("Setting up i2c!");
    let scl = p.PC2;
    let sda = p.PC1;
    let i2c = I2c::new_blocking(p.I2C1, scl, sda, Hertz::hz(400_000), Default::default());

    let i2c = RefCell::new(i2c);
    let i2c_a = RefCellDevice::new(&i2c);
    let i2c_b = RefCellDevice::new(&i2c);

    savegame::setup(i2c_a);

    // Setup screen
    let mut display = display::setup(i2c_b).into_buffered_graphics_mode();
    hal::println!("Init oled");
    display.init().unwrap();

    // Buttons
    let btn1 = Input::new(p.PA1, gpio::Pull::Up);
    let btn2 = Input::new(p.PA2, gpio::Pull::Up);
    let btn3 = Input::new(p.PC0, gpio::Pull::Up);
    let btn4 = Input::new(p.PC4, gpio::Pull::Up);

    loop {
        display.clear_buffer();

        Text::with_baseline(
            "ohai!",
            Point::new(0, 0),
            MonoTextStyle::new(&FONT_6X10, BinaryColor::On),
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();

        display.flush().unwrap();

        if btn1.is_low() {
            hal::println!("btn1 pressed");
        }
        if btn2.is_low() {
            hal::println!("btn2 pressed");
        }
        if btn3.is_low() {
            hal::println!("btn3 pressed");
        }
        if btn4.is_low() {
            hal::println!("btn4 pressed");
        }

        /*
        hal::println!("Done rendering");
        delay.delay_ms(2_000);
        hal::println!("Turning off display");
        display.set_display_on(false);
        delay.delay_ms(2_000);
        hal::println!("Turning on display");
        display.set_display_on(true);
        */

        /*
        led.toggle();
        delay.delay_ms(1000);
        // hal::println!("toggle!");

        // wait for interrupt
        riscv::asm::wfi();
        */

        hal::println!("ohai!");
        delay.delay_ms(250);
    }
}

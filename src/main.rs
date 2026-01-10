#![no_std]
#![no_main]

mod display;
mod game;
mod gfx;
mod input;
mod savegame;

use crate::{
    game::{Game, campaign::Campaign},
    input::{Action, Button, Event},
};
use ch32_hal::{
    self as hal,
    // delay::Delay,
    gpio::{self, Input},
    i2c::I2c,
    time::Hertz,
};
use core::cell::RefCell;
use embedded_hal_bus::i2c::RefCellDevice;
use panic_halt as _;
use ssd1306::prelude::*;
// use qingke::riscv;

#[qingke_rt::entry]
fn main() -> ! {
    // hal::debug::SDIPrint::enable();
    let p = hal::init(hal::Config::default());
    // let mut delay = Delay;

    // Setup I2C
    let scl = p.PC2;
    let sda = p.PC1;
    let i2c = I2c::new_blocking(p.I2C1, scl, sda, Hertz::hz(400_000), Default::default());

    let i2c = RefCell::new(i2c);
    let i2c_a = RefCellDevice::new(&i2c);
    let i2c_b = RefCellDevice::new(&i2c);

    let flash = savegame::setup(i2c_a);

    // Setup screen
    let mut display = display::setup(i2c_b).into_buffered_graphics_mode();
    display.init().ok();

    // Buttons
    let mut btn_up = Button::new(Input::new(p.PA1, gpio::Pull::Up));
    let mut btn_down = Button::new(Input::new(p.PA2, gpio::Pull::Up));
    let mut btn_b = Button::new(Input::new(p.PC0, gpio::Pull::Up));
    let mut btn_a = Button::new(Input::new(p.PC4, gpio::Pull::Up));

    let mut game = Game::start();
    let mut campaign = Campaign::new(flash);
    campaign.scan_savegames();

    loop {
        // Render
        display.clear_buffer();
        game.render(&mut display, &campaign);
        display.flush().ok();

        // Inputs
        match btn_up.probe() {
            Some(Action::Pressed) => game.event(Event::Up, &mut campaign),
            None => (),
        }

        match btn_down.probe() {
            Some(Action::Pressed) => game.event(Event::Down, &mut campaign),
            None => (),
        }

        match btn_b.probe() {
            Some(Action::Pressed) => game.event(Event::B, &mut campaign),
            None => (),
        }

        match btn_a.probe() {
            Some(Action::Pressed) => game.event(Event::A, &mut campaign),
            None => (),
        }

        game = campaign.next_scene.take().unwrap_or(game);
        game.tick(&mut campaign);

        /*
        // wait for interrupt
        riscv::asm::wfi();
        */

        // delay.delay_ms(250);
    }
}

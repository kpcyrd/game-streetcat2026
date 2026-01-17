#![no_std]
#![no_main]

mod display;
mod game;
mod gfx;
mod input;
mod savegame;
mod text;

use crate::game::{Game, Unlocks, campaign::Campaign};
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
    let mut btns = input::Buttons::new(
        Input::new(p.PA1, gpio::Pull::Up),
        Input::new(p.PA2, gpio::Pull::Up),
        Input::new(p.PC4, gpio::Pull::Up),
        Input::new(p.PC0, gpio::Pull::Up),
    );

    // Setup game
    let mut game = Game::start();
    let mut campaign = Campaign::new(flash);
    campaign.scan_savegames();

    // Allow booting directly into fishing for dev purposes
    if option_env!("BOOT_GAME").is_some() {
        game = Game::fishing(game::fishing::Timer::Onboarding);
        campaign.unlocks.insert(Unlocks::STORY_ESCAPED_CORPORATE);
        campaign.unlocks.insert(Unlocks::BOUGHT_KING_STATUS);
        campaign
            .acknowledged_scenes
            .insert(Unlocks::STORY_ESCAPED_CORPORATE);
    }

    loop {
        // Render
        display.clear_buffer();
        game.render(&mut display, &campaign);
        display.flush().ok();

        // Inputs
        if let Some(event) = btns.scan() {
            game.event(event, &mut campaign);
        }

        // Update game state
        game = campaign.next_scene.take().unwrap_or(game);
        game.tick(&mut campaign);
    }
}

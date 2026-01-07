use ch32_hal::{self as hal};
use eeprom24x::{Eeprom24x, SlaveAddr};
use embedded_savegame::storage::Storage;

const SLOT_SIZE: usize = 64;
const SLOT_COUNT: usize = 256;

pub fn setup<I2C: embedded_hal::i2c::I2c>(i2c: I2C) {
    hal::println!("Setting up eeprom");
    let addr = SlaveAddr::Default;
    let eeprom = Eeprom24x::new_24x256(i2c, addr);
    let mut flash = Storage::<_, SLOT_SIZE, SLOT_COUNT>::new(eeprom);
    let save_slot = flash.scan().unwrap();
    hal::println!("Found save slot: {:x?}", save_slot);
}

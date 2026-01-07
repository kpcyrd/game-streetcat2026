use eeprom24x::{Eeprom24x, SlaveAddr, addr_size::TwoBytes, page_size::B64, unique_serial};
use embedded_savegame::storage::Storage;

pub const SLOT_SIZE: usize = 64;
pub const SLOT_COUNT: usize = 256;

pub fn setup<I2C: embedded_hal::i2c::I2c>(
    i2c: I2C,
) -> Storage<Eeprom24x<I2C, B64, TwoBytes, unique_serial::No>, SLOT_SIZE, SLOT_COUNT> {
    let addr = SlaveAddr::Default;
    let eeprom = Eeprom24x::new_24x256(i2c, addr);
    Storage::<_, SLOT_SIZE, SLOT_COUNT>::new(eeprom)
}

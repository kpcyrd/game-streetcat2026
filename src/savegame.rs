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

// Exact size of our savegame data
const SAVE_SIZE: usize = 4;

pub struct Save {
    pub buf: [u8; SAVE_SIZE],
}

impl Save {
    pub const fn new() -> Self {
        Self {
            buf: [0; SAVE_SIZE],
        }
    }

    pub fn set_money(&mut self, money: u16) {
        let buf = arrayref::array_mut_ref![self.buf, 0, 2];
        buf.copy_from_slice(&money.to_be_bytes());
    }

    pub fn set_unlocks(&mut self, unlocks: u16) {
        let buf = arrayref::array_mut_ref![self.buf, 2, 2];
        buf.copy_from_slice(&unlocks.to_be_bytes());
    }

    pub fn get_money(&self) -> u16 {
        let buf = arrayref::array_ref![self.buf, 0, 2];
        u16::from_be_bytes(*buf)
    }

    pub fn get_unlocks(&self) -> u16 {
        let buf = arrayref::array_ref![self.buf, 2, 2];
        u16::from_be_bytes(*buf)
    }
}

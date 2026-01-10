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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_pull() {
        let mut save = Save::new();
        save.buf[..4].copy_from_slice(&[0x12, 0x34, 0x56, 0x78]);
        save.reset(4);

        assert_eq!(save.pull_u16(0xFF), 0x1234);
        assert_eq!(save.pull_u8(0xFF), 0x56);
        assert_eq!(save.pull_u8(0xFF), 0x78);
    }

    #[test]
    fn test_save_push() {
        let mut save = Save::new();
        assert_eq!(save.slice(), &[]);

        save.push_u16(0x1234);
        save.push_u8(0x56);
        save.push_u8(0x78);

        assert_eq!(save.slice(), &[0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_save_pull_from_empty() {
        let mut save = Save::new();
        assert_eq!(save.pull_u16(0x1337), 0x1337);
    }
}

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

// Maximum size we are willing to save/load
const SAVE_SIZE: usize = 256;

#[derive(Clone, Copy)]
pub struct Save {
    pub buf: [u8; SAVE_SIZE],
    pub cursor: usize,
    pub capacity: usize,
}

impl Save {
    pub const fn new() -> Self {
        Self {
            buf: [0; SAVE_SIZE],
            cursor: 0,
            capacity: 0,
        }
    }

    pub fn reset(&mut self, capacity: usize) {
        self.capacity = capacity;
        self.cursor = 0;
    }

    // This is needed because embedded-savegame currently expects this because of w25q
    pub fn slice(&mut self) -> &mut [u8] {
        &mut self.buf[..self.capacity]
    }

    fn take(&mut self, amount: usize) -> Option<&[u8]> {
        let slice = self
            .buf
            .get(..self.capacity)
            .unwrap_or_default()
            .get(self.cursor..)
            .unwrap_or_default()
            .get(..amount);
        self.cursor = self.cursor.saturating_add(amount);
        slice
    }

    pub fn pull_u32(&mut self, default: u32) -> u32 {
        if let Some(value) = self.take(4) {
            u32::from_be_bytes(value.try_into().unwrap())
        } else {
            default
        }
    }

    fn push(&mut self, buf: &[u8]) {
        let range = self.cursor..self.cursor + buf.len();
        if let Some(slice) = self.buf.get_mut(range) {
            slice.copy_from_slice(buf);
            self.cursor = self.cursor.saturating_add(buf.len());
            self.capacity = self.cursor;
        }
    }

    pub fn push_u32(&mut self, value: u32) {
        self.push(&value.to_be_bytes());
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

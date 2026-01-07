use crate::savegame::{SLOT_COUNT, SLOT_SIZE};
use embedded_savegame::storage::{Flash, Storage};

pub struct Campaign<F: Flash> {
    pub flash: Storage<F, SLOT_SIZE, SLOT_COUNT>,
    pub save_slot: Option<embedded_savegame::Slot>,
}

impl<F: Flash> Campaign<F> {
    pub const fn new(flash: Storage<F, SLOT_SIZE, SLOT_COUNT>) -> Self {
        Campaign {
            flash,
            save_slot: None,
        }
    }

    pub fn scan_savegames(&mut self) {
        self.save_slot = self.flash.scan().unwrap();
    }

    pub fn init(&mut self) {
        /*
        if let Some(slice) = campaign.flash.read(slot.idx, &mut save.buf).unwrap() {
            let len = slice.len();
            save.reset(len);
        }
        */
        // self.flash.append(&mut [0]).unwrap();
    }
}

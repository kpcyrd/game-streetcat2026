use crate::{
    game::{Game, Unlocks, plot},
    savegame::{SLOT_COUNT, SLOT_SIZE, Save},
};
use embedded_savegame::storage::{Flash, Storage};

const CORPORATE_ESCAPE_THRESHOLD: u16 = 50;

// Mask is applied to RNG, then MIN is added
const NEXT_KEY_MIN: u8 = 4;
const NEXT_KEY_MASK: u8 = 0b111;

pub struct Campaign<F: Flash> {
    pub flash: Storage<F, SLOT_SIZE, SLOT_COUNT>,
    pub save_slot: Option<embedded_savegame::Slot>,
    pub money: u16,
    pub unlocks: Unlocks,
    pub acknowledged_scenes: Unlocks,
    pub rng: u32,
    pub next_unlock_key: u8,
    pub next_scene: Option<Game>,
}

impl<F: Flash> Campaign<F> {
    pub const fn new(flash: Storage<F, SLOT_SIZE, SLOT_COUNT>) -> Self {
        Campaign {
            flash,
            save_slot: None,
            money: 0,
            unlocks: Unlocks::empty(),
            acknowledged_scenes: Unlocks::empty(),
            rng: djb2::hash(&[]),
            next_unlock_key: 0,
            next_scene: None,
        }
    }

    pub const fn feed_rng(&mut self) {
        self.rng = djb2::hash_with_initial(self.rng, &[0xFF]);
    }

    pub fn scan_savegames(&mut self) {
        self.save_slot = self.flash.scan().ok().flatten();
    }

    pub fn write_savegame(&mut self) {
        let mut save = Save::new();
        save.set_money(self.money);
        save.set_unlocks(self.unlocks.bits());
        self.flash.append_static(&mut save.buf).ok();
    }

    pub fn init(&mut self) {
        let mut save = Save::new();

        if let Some(slot) = &self.save_slot {
            self.flash.read_static(slot.idx, &mut save.buf).ok();
        }

        self.money = save.get_money();
        self.unlocks = Unlocks::from_bits_truncate(save.get_unlocks());
        self.acknowledged_scenes = self.unlocks;

        // Start game
        self.init_next();
    }

    pub fn init_next(&mut self) {
        self.write_savegame();
        self.setup_next_unlock_key();
        self.next_scene = Some(plot::get(self));
    }

    pub const fn escaped_corporate(&self) -> bool {
        self.unlocks.contains(Unlocks::STORY_ESCAPED_CORPORATE)
    }

    pub const fn can_escape_corporate(&self) -> bool {
        self.money >= CORPORATE_ESCAPE_THRESHOLD
    }

    pub fn setup_next_unlock_key(&mut self) {
        let num = self.rng as u8;
        self.next_unlock_key = (num & NEXT_KEY_MASK) + NEXT_KEY_MIN;
    }
}

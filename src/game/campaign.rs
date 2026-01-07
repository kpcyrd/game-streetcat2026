use crate::{
    game::{Game, Unlocks, story::Story},
    savegame::{SLOT_COUNT, SLOT_SIZE, Save},
};
use embedded_savegame::storage::{Flash, Storage};

pub struct Campaign<F: Flash> {
    pub flash: Storage<F, SLOT_SIZE, SLOT_COUNT>,
    pub save_slot: Option<embedded_savegame::Slot>,
    pub money: u32,
    pub unlocks: Unlocks,
    pub rng: u32,
    pub next_scene: Option<Game>,
}

impl<F: Flash> Campaign<F> {
    pub const fn new(flash: Storage<F, SLOT_SIZE, SLOT_COUNT>) -> Self {
        Campaign {
            flash,
            save_slot: None,
            money: 0,
            unlocks: Unlocks::empty(),
            rng: djb2::hash(&[]),
            next_scene: None,
        }
    }

    pub fn feed_rng(&mut self) {
        self.rng = djb2::hash_with_initial(self.rng, &[0xFF]);
    }

    pub fn scan_savegames(&mut self) {
        self.save_slot = self.flash.scan().unwrap();
    }

    pub fn write_savegame(&mut self) {
        let mut save = Save::new();
        save.push_u32(self.money);
        save.push_u32(self.unlocks.bits());
        self.flash.append(save.slice()).unwrap();
    }

    pub fn init(&mut self) {
        let mut save = Save::new();

        if let Some(slot) = &self.save_slot
            && let Some(slice) = self.flash.read(slot.idx, &mut save.buf).unwrap()
        {
            let len = slice.len();
            save.reset(len);
        }

        self.money = save.pull_u32(0);
        self.unlocks = Unlocks::from_bits_truncate(save.pull_u32(0));

        // Start game
        self.init_next();
    }

    pub fn init_next(&mut self) {
        self.write_savegame();
        self.next_scene = Some(self.scene());
    }

    fn scene(&self) -> Game {
        if !self.unlocks.contains(Unlocks::STORY_INTRO) {
            Game::Story(Story::new(&["abc", "def"], Unlocks::STORY_INTRO))
        } else {
            Game::fishing()
        }
    }
}

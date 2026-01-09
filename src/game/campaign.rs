use crate::{
    game::{Game, Unlocks, fishing, story::Story},
    savegame::{SAVE_SIZE, SLOT_COUNT, SLOT_SIZE, Save},
};
use embedded_savegame::storage::{Flash, Storage};

const CORPORATE_ESCAPE_THRESHOLD: u16 = 50;

pub struct Campaign<F: Flash> {
    pub flash: Storage<F, SLOT_SIZE, SLOT_COUNT>,
    pub save_slot: Option<embedded_savegame::Slot>,
    pub money: u16,
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
        self.flash.append(&mut save.buf).ok();
    }

    pub fn init(&mut self) {
        let mut save = Save::new();

        if let Some(slot) = &self.save_slot
            && let Ok(Some(slice)) = self.flash.read(slot.idx, &mut save.buf)
            && slice.len() != SAVE_SIZE
        {
            save.set_money(0);
            save.set_unlocks(0);
        }

        self.money = save.get_money();
        self.unlocks = Unlocks::from_bits_truncate(save.get_unlocks());

        // Start game
        self.init_next();
    }

    pub fn init_next(&mut self) {
        self.write_savegame();
        self.next_scene = Some(self.scene());
    }

    const fn scene(&self) -> Game {
        if !self.unlocks.contains(Unlocks::STORY_INTRO) {
            Game::Story(Story::new(
                "You wake up in your\ncorporate job.\n\nYour job is to\ndelete emails.",
                Unlocks::STORY_INTRO,
            ))
        } else if self.escaped_corporate()
            && !self.unlocks.contains(Unlocks::STORY_ACKNOWLEDGED_ESCAPE)
        {
            Game::Story(Story::new(
                "You are free!\n\nNow what?",
                Unlocks::STORY_ACKNOWLEDGED_ESCAPE,
            ))
        } else {
            Game::fishing(fishing::Timer::Onboarding)
        }
    }

    pub const fn escaped_corporate(&self) -> bool {
        self.unlocks.contains(Unlocks::STORY_ESCAPED_CORPORATE)
    }

    pub const fn can_escape_corporate(&self) -> bool {
        self.money >= CORPORATE_ESCAPE_THRESHOLD
    }
}

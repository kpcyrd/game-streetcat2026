use crate::game::{Game, Unlocks, campaign::Campaign, fishing, story::Story};
use embedded_savegame::storage::Flash;

pub const STORY_UNLOCKS: Unlocks = Unlocks::SHOP_BAMBOO_ROD.union(Unlocks::SHOP_CATCHY_HOOK);

const STORY_INTRO: Story = Story::new(
    &[
        "You wake up in your",
        "corporate job.",
        "",
        "Your job is to",
        "delete emails.",
    ],
    Unlocks::STORY_INTRO,
);

const STORY_ACKNOWLEDGED_ESCAPE: Story = Story::new(
    &[
        "You are free!",
        "",
        "Unfortunately, life",
        "is now somewhat more",
        "complicated.",
    ],
    Unlocks::STORY_ESCAPED_CORPORATE,
);

const STORY_SHOP_UNLOCKED: Story = Story::new(
    &[
        "We found an upgrade!",
        "",
        "Press B to open shop.",
        "We only need money.",
        "",
        "Oh...",
    ],
    Unlocks::first_shop_unlock(),
);

const STORY_KING_GOAL: Story = Story::new(
    &[
        "I know what I want to",
        "do next.",
        "",
        "I want to be King of",
        "the streets!",
    ],
    Unlocks::first_shop_unlock(),
);

// Sanity checks
const _: () = const {
    const MAX_LINE_LENGTH: usize = 21;
    const MAX_LINES: usize = 6;
    assert!(STORY_INTRO.longest_line() <= MAX_LINE_LENGTH);
    assert!(STORY_INTRO.num_lines() <= MAX_LINES);
    assert!(STORY_ACKNOWLEDGED_ESCAPE.longest_line() <= MAX_LINE_LENGTH);
    assert!(STORY_ACKNOWLEDGED_ESCAPE.num_lines() <= MAX_LINES);
    assert!(STORY_SHOP_UNLOCKED.longest_line() <= MAX_LINE_LENGTH);
    assert!(STORY_SHOP_UNLOCKED.num_lines() <= MAX_LINES);
    assert!(STORY_KING_GOAL.longest_line() <= MAX_LINE_LENGTH);
    assert!(STORY_KING_GOAL.num_lines() <= MAX_LINES);
};

pub const fn get<F: Flash>(campaign: &Campaign<F>) -> Game {
    if !campaign.unlocks.contains(Unlocks::STORY_INTRO) {
        Game::Story(&STORY_INTRO)
    } else if campaign
        .unacknowledged()
        .contains(Unlocks::STORY_ESCAPED_CORPORATE)
    {
        Game::Story(&STORY_ACKNOWLEDGED_ESCAPE)
    } else if campaign
        .unacknowledged()
        .contains(Unlocks::first_shop_unlock())
    {
        Game::Story(&STORY_SHOP_UNLOCKED)
    } else {
        Game::fishing(fishing::Timer::Onboarding)
    }
}

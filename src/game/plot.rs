use crate::game::{Game, Unlocks, campaign::Campaign, fishing, story::Story};
use embedded_savegame::storage::Flash;

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
    &["You are free!", "", "Now what?"],
    Unlocks::STORY_ACKNOWLEDGED_ESCAPE,
);

pub const fn get<F: Flash>(campaign: &Campaign<F>) -> Game {
    if !campaign.unlocks.contains(Unlocks::STORY_INTRO) {
        Game::Story(&STORY_INTRO)
    } else if campaign.escaped_corporate()
        && !campaign
            .unlocks
            .contains(Unlocks::STORY_ACKNOWLEDGED_ESCAPE)
    {
        Game::Story(&STORY_ACKNOWLEDGED_ESCAPE)
    } else {
        Game::fishing(fishing::Timer::Onboarding)
    }
}

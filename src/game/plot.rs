use crate::game::{Game, Unlocks, campaign::Campaign, fishing, story::Story};
use embedded_savegame::storage::Flash;

pub const STORY_UNLOCKS: Unlocks = Unlocks::SHOP_UPGRADED_ROD;

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
    Unlocks::STORY_ESCAPED_CORPORATE,
);

const STORY_SHOP_UNLOCKED: Story =
    Story::new(&["Press B to enter shop"], Unlocks::first_shop_unlock());

pub const fn get<F: Flash>(campaign: &Campaign<F>) -> Game {
    if !campaign.unlocks.contains(Unlocks::STORY_INTRO) {
        Game::Story(&STORY_INTRO)
    } else if campaign.escaped_corporate()
        && !campaign
            .acknowledged_scenes
            .contains(Unlocks::STORY_ESCAPED_CORPORATE)
    {
        Game::Story(&STORY_ACKNOWLEDGED_ESCAPE)
    } else if campaign.unlocks.contains(Unlocks::SHOP_UPGRADED_ROD)
        && !campaign
            .acknowledged_scenes
            .contains(Unlocks::SHOP_UPGRADED_ROD)
    {
        Game::Story(&STORY_SHOP_UNLOCKED)
    } else {
        Game::fishing(fishing::Timer::Onboarding)
    }
}

use crate::{
    game::{Game, Unlocks, campaign::Campaign, fishing},
    gfx,
    input::Event,
    text::Text,
};
use embedded_graphics::{
    Drawable,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
};
use embedded_savegame::storage::Flash;

const MENU_LIMIT: usize = 3;
const CURSOR_LEFT_PAD: i32 = gfx::FONT_WIDTH * 2;
const ITEM_LEFT_PAD: i32 = CURSOR_LEFT_PAD + gfx::FONT_WIDTH * 6;

const MAX_ITEM_NAME: usize = 13;

const SHOP_MENU: &[&[ShopItem]] = &[
    &[ShopItem::UpgradedRod, ShopItem::CarbonRod],
    &[ShopItem::BasicBait, ShopItem::PremiumBait],
    &[
        ShopItem::BetterRates,
        ShopItem::BestRates,
        ShopItem::KingStatus,
    ],
];

// Sanity checks
const _: () = const {
    // Ensure this number matches
    assert!(SHOP_MENU.len() == MENU_LIMIT);

    // Ensure all item names fit (for loops are not const yet)
    let mut a = 0;
    while a < SHOP_MENU.len() {
        let items = SHOP_MENU[a];
        let mut b = 0;
        while b < items.len() {
            let item = items[b];
            assert!(item.text().len() <= MAX_ITEM_NAME);
            b += 1;
        }
        a += 1;
    }
};

#[derive(Debug, Clone, Copy)]
pub enum ShopItem {
    // Slot 1
    UpgradedRod,
    CarbonRod,
    // Slot 2
    BasicBait,
    PremiumBait,
    // Slot 3
    BetterRates,
    BestRates,
    KingStatus,
}

impl ShopItem {
    pub const fn price(&self) -> u16 {
        match self {
            // Slot 1
            ShopItem::UpgradedRod => 100,
            ShopItem::CarbonRod => 300,
            // Slot 2
            ShopItem::BasicBait => 50,
            ShopItem::PremiumBait => 150,
            // Slot 3
            ShopItem::BetterRates => 200,
            ShopItem::BestRates => 500,
            ShopItem::KingStatus => 9_999,
        }
    }

    pub const fn text(&self) -> &'static str {
        match self {
            // Slot 1
            ShopItem::UpgradedRod => "Upgraded Rod",
            ShopItem::CarbonRod => "Carbon Rod",
            // Slot 2
            ShopItem::BasicBait => "Basic Bait",
            ShopItem::PremiumBait => "Premium Bait",
            // Slot 3
            ShopItem::BetterRates => "Better Rates",
            ShopItem::BestRates => "Best Rates",
            ShopItem::KingStatus => "King Status",
        }
    }

    pub const fn depends(&self) -> Unlocks {
        match self {
            // Slot 1
            ShopItem::UpgradedRod => Unlocks::SHOP_UPGRADED_ROD,
            ShopItem::CarbonRod => Unlocks::SHOP_CARBON_RID,
            // Slot 2
            ShopItem::BasicBait => Unlocks::SHOP_BASIC_BAIT,
            ShopItem::PremiumBait => Unlocks::SHOP_PREMIUM_BAIT,
            // Slot 3
            ShopItem::BetterRates => Unlocks::SHOP_BETTER_RATES,
            ShopItem::BestRates => Unlocks::SHOP_BEST_RATES,
            ShopItem::KingStatus => Unlocks::SHOP_KING_STATUS,
        }
    }

    pub const fn purchased(&self) -> Unlocks {
        match self {
            // Slot 1
            ShopItem::UpgradedRod => Unlocks::BOUGHT_UPGRADED_ROD,
            ShopItem::CarbonRod => Unlocks::BOUGHT_CARBON_RID,
            // Slot 2
            ShopItem::BasicBait => Unlocks::BOUGHT_BASIC_BAIT,
            ShopItem::PremiumBait => Unlocks::BOUGHT_PREMIUM_BAIT,
            // Slot 3
            ShopItem::BetterRates => Unlocks::BOUGHT_BETTER_RATES,
            ShopItem::BestRates => Unlocks::BOUGHT_BEST_RATES,
            ShopItem::KingStatus => Unlocks::BOUGHT_KING_STATUS,
        }
    }

    fn item(idx: usize, unlocks: Unlocks) -> Option<(Self, u16)> {
        let items = SHOP_MENU.get(idx)?;

        let mut current = None;
        for item in *items {
            if unlocks.contains(item.depends()) {
                let price = if unlocks.contains(item.purchased()) {
                    0
                } else {
                    item.price()
                };
                current = Some((*item, price));
            }
        }

        current
    }
}

pub struct Shop {
    idx: usize,
}

impl Shop {
    pub const fn new() -> Self {
        Shop { idx: 0 }
    }

    pub fn event<F: Flash>(&mut self, event: Event, campaign: &mut Campaign<F>) {
        match event {
            Event::Up => self.idx = self.idx.checked_sub(1).unwrap_or(MENU_LIMIT - 1),
            Event::Down => self.idx = (self.idx + 1) % MENU_LIMIT,
            Event::A => {
                if let Some((item, price)) = ShopItem::item(self.idx, campaign.unlocks)
                    && let Some(new_balance) = campaign.money.checked_sub(price)
                {
                    campaign.money = new_balance;
                    campaign.unlocks.insert(item.purchased());
                    campaign.write_savegame();
                }
            }
            Event::B => {
                campaign.next_scene = Some(Game::fishing(fishing::Timer::Random));
            }
        }
    }

    pub fn render_price<D: DrawTarget<Color = BinaryColor>>(
        &self,
        display: &mut D,
        mut point: Point,
        price: u16,
    ) {
        if price == 0 {
            return;
        }

        point.x += CURSOR_LEFT_PAD;

        // Padding for price alignment
        if price < 10 {
            point.x += gfx::FONT_WIDTH * 3;
        } else if price < 100 {
            point.x += gfx::FONT_WIDTH * 2;
        } else if price < 1000 {
            point.x += gfx::FONT_WIDTH;
        }

        // Render price
        gfx::render_currency(display, point);
        point.x += gfx::FONT_WIDTH;

        let mut buf = itoa::Buffer::new();
        let price = buf.format(price);
        Text::new(price, point).draw(display).ok();
    }

    pub fn render<D: DrawTarget<Color = BinaryColor>, F: Flash>(
        &self,
        display: &mut D,
        campaign: &Campaign<F>,
    ) {
        Text::new("Shop!", Point::new(0, 0)).draw(display).ok();
        gfx::render_balance(display, campaign.money);

        let mut point = Point::new(0, gfx::FONT_HEIGHT * 2);
        for n in 0..MENU_LIMIT {
            if n == self.idx {
                Text::new(">", point).draw(display).ok();
            }

            let item = ShopItem::item(n, campaign.unlocks);
            if let Some((item, price)) = item {
                self.render_price(display, point, price);

                Text::new(item.text(), point + Point::new(ITEM_LEFT_PAD, 0))
                    .draw(display)
                    .ok();
            } else {
                Text::new("LOCKED", point + Point::new(CURSOR_LEFT_PAD, 0))
                    .draw(display)
                    .ok();
            }

            point.y += gfx::FONT_HEIGHT;
        }

        // Help
        Text::new("| A: Buy", gfx::LAST_LINE_A).draw(display).ok();
    }
}

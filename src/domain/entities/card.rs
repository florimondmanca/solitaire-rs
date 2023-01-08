#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Heart,
    Club,
    Diamond,
}

impl Suit {
    pub fn all() -> [Self; 4] {
        [Self::Heart, Self::Diamond, Self::Spades, Self::Club]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rank(pub u8);

impl Rank {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn all() -> Vec<Self> {
        (1..=13).map(|n| Self::new(n)).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    shown: bool,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self {
            rank,
            suit,
            shown: false,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.shown
    }

    pub fn reveal(&mut self) {
        self.shown = true;
    }

    pub fn hide(&mut self) {
        self.shown = false;
    }
}

pub fn get_standard_pack() -> Vec<Card> {
    let mut pack = Vec::new();

    for suit in Suit::all() {
        for rank in Rank::all() {
            pack.push(Card::new(rank, suit));
        }
    }

    pack
}

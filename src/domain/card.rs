// Rules: https://www.officialgamerules.org/solitaire

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Heart,
    Club,
    Diamond,
}

impl Suit {
    pub fn all() -> [Self; 4] {
        [Self::Spades, Self::Heart, Self::Club, Self::Diamond]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    face: Face,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit, face: Face) -> Self {
        Self { rank, suit, face }
    }

    pub fn is_visible(&self) -> bool {
        self.face == Face::Up
    }

    pub fn reveal(&mut self) {
        self.face = Face::Up;
    }
}

pub fn get_standard_pack() -> Vec<Card> {
    let mut pack = Vec::new();

    for suit in Suit::all() {
        for rank in Rank::all() {
            pack.push(Card::new(rank, suit, Face::Down));
        }
    }

    pack
}

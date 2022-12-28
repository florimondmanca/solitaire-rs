use rand::prelude::*;

// Rules: https://www.officialgamerules.org/solitaire

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

    pub fn show(&mut self) {
        self.face = Face::Up;
    }

    pub fn flip(&mut self) {
        self.face = match self.face {
            Face::Up => Face::Down,
            Face::Down => Face::Up,
        };
    }

    pub fn is_visible(&self) -> bool {
        self.face == Face::Up
    }
}

pub type Pile = Vec<Card>;

pub struct Board {
    pub tableau: Vec<Pile>,
    pub foundations: Vec<Pile>,
    pub stock: Pile,
    pub waste: Pile,
}

impl Board {
    pub fn new() -> Self {
        let mut tableau = (0..7).map(|_| Pile::new()).collect::<Vec<_>>();

        let mut pack = get_standard_pack();

        pack.shuffle(&mut rand::thread_rng());

        for index in 0..7 {
            let pile = &mut tableau[index];
            let mut card = pack.pop().unwrap();
            card.show();
            pile.push(card);

            for j in index + 1..7 {
                let other_pile = &mut tableau[j];
                other_pile.push(pack.pop().unwrap());
            }
        }

        let mut stock = Pile::new();

        while let Some(card) = pack.pop() {
            stock.push(card);
        }

        let foundations = (0..4).map(|_| Pile::new()).collect();

        Self {
            tableau,
            foundations,
            stock,
            waste: Pile::new(),
        }
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

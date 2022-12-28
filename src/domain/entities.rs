use std::ops::{Deref, DerefMut};

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

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct PileCard {
    pub card: Card,
    pub face: Face,
}

impl PileCard {
    pub fn new(card: Card, face: Face) -> Self {
        Self { card, face }
    }
}

#[derive(Debug, Clone)]
pub struct Pile(Vec<PileCard>);

impl Deref for Pile {
    type Target = Vec<PileCard>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Pile {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<PileCard>> for Pile {
    fn from(value: Vec<PileCard>) -> Self {
        let mut pile = Self::new();
        pile.extend(value.clone());
        pile
    }
}

impl FromIterator<PileCard> for Pile {
    fn from_iter<T: IntoIterator<Item = PileCard>>(iter: T) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

pub struct Board {
    pub tableau: Vec<Pile>,
    pub foundations: Vec<Pile>,
    pub stock: Pile,
    pub waste: Pile,
}

impl Board {
    pub fn new() -> Self {
        let mut tableau = (0..7).map(|_| Pile::new()).collect::<Vec<_>>();

        let mut cards = shuffle_cards();

        for index in 0..7 {
            let pile = &mut tableau[index];
            pile.push(PileCard::new(cards.pop().unwrap(), Face::Up));

            for j in index + 1..7 {
                let other_pile = &mut tableau[j];
                other_pile.push(PileCard::new(cards.pop().unwrap(), Face::Down));
            }
        }

        let mut stock = Pile::new();

        while let Some(card) = cards.pop() {
            stock.push(PileCard::new(card, Face::Down));
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

pub fn shuffle_cards() -> Vec<Card> {
    let mut cards = Vec::new();

    for suit in Suit::all() {
        for rank in Rank::all() {
            cards.push(Card::new(rank, suit));
        }
    }

    cards.shuffle(&mut rand::thread_rng());

    cards
}

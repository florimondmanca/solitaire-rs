use rand::prelude::*;

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

pub type Pile = Vec<Card>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Stock,
    Pile(usize),
}

pub struct Board {
    tableau: Vec<Pile>,
    foundations: Vec<Pile>,
    stock: Pile,
    waste: Pile,
}

impl Board {
    pub fn new() -> Self {
        let mut tableau = (0..7).map(|_| Pile::new()).collect::<Vec<_>>();

        let mut pack = get_standard_pack();

        pack.shuffle(&mut rand::thread_rng());

        for index in 0..7 {
            let pile = &mut tableau[index];
            let mut card = pack.pop().unwrap();
            card.reveal();
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

    pub fn get_stock(&self) -> &Pile {
        &self.stock
    }

    pub fn get_waste(&self) -> &Pile {
        &self.waste
    }

    pub fn get_foundations(&self) -> &Vec<Pile> {
        &self.foundations
    }

    pub fn get_tableau(&self) -> &Vec<Pile> {
        &self.tableau
    }

    pub fn get(&self, target: Target) -> Option<&Pile> {
        match target {
            Target::Stock => Some(&self.stock),
            Target::Pile(index) => self.tableau.get(index),
        }
    }

    pub fn get_mut(&mut self, target: Target) -> Option<&mut Pile> {
        match target {
            Target::Stock => Some(&mut self.stock),
            Target::Pile(index) => self.tableau.get_mut(index),
        }
    }

    pub fn rotate_left(&self, target: Target) -> Target {
        match target {
            Target::Stock => Target::Pile(self.tableau.len() - 1),
            Target::Pile(0) => Target::Stock,
            Target::Pile(n) => Target::Pile(n - 1),
        }
    }

    pub fn rotate_right(&self, target: Target) -> Target {
        match target {
            Target::Stock => Target::Pile(0),
            Target::Pile(n) if n == self.tableau.len() - 1 => Target::Stock,
            Target::Pile(n) => Target::Pile(n + 1),
        }
    }

    pub fn maybe_transfer(&mut self, source: Target, dest: Target) {
        if dest == Target::Stock {
            // Can't transfer to the stock pile.
            return;
        }

        let source_pile = self.get(source).unwrap();
        let dest_pile = self.get(dest).unwrap();

        // Card of rank N can be transferred to an empty pile,
        // or a pile whose top card is hidden...
        if dest_pile.last().map_or(true, |c| !c.is_visible()) {
            self.transfer(source, dest);
            return;
        }

        // ... or a pile whose top card has rank N + 1.
        let source_rank = source_pile.last().unwrap().rank.0;
        let dest_rank = dest_pile.last().unwrap().rank.0;
        if dest_rank == source_rank + 1 {
            self.transfer(source, dest);
        }
    }

    fn transfer(&mut self, source: Target, dest: Target) {
        let s = self.get_mut(source).unwrap();
        let card = s.pop().unwrap();
        let t = self.get_mut(dest).unwrap();
        t.push(card);
    }

    pub fn maybe_move_to_foundation(&mut self, target: Target) -> bool {
        let pile = self.get(target).unwrap();

        if pile.is_empty() {
            return false;
        }

        let card = *pile.last().unwrap();

        if !card.is_visible() {
            return false;
        }

        let mut was_transferred = false;

        for foundation in self.foundations.iter_mut() {
            // Find a foundation where the card be transferred, if any.
            // We do this automatically for better UX.

            // Empty foundations can only be transferred an ace.
            if foundation.is_empty() {
                if card.rank.0 == 1 {
                    foundation.push(card);
                    was_transferred = true;
                    break;
                }
                continue;
            }

            // For established foundations, the suit must match and
            // cards must be stacked with ranks ascending.
            let last = foundation.last().unwrap();

            if last.suit == card.suit && card.rank.0 == last.rank.0 + 1 {
                foundation.push(card);
                was_transferred = true;
                break;
            }
        }

        if was_transferred {
            // Need to do this outside the for-loop to please the borrow checker.
            self.get_mut(target).unwrap().pop().unwrap();
        }

        was_transferred
    }

    pub fn maybe_move_to_waste(&mut self) -> bool {
        if let Some(&card) = self.stock.last().filter(|c| c.is_visible()) {
            self.stock.pop();
            self.waste.push(card);
            true
        } else {
            false
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

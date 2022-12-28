use std::fmt;

use crate::domain::entities::{Board, Card, Face, Pile, PileCard, Rank, Suit};

static PILE_WIDTH: usize = 12;

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Spades => write!(f, "♠"),
            Suit::Heart => write!(f, "♥"),
            Suit::Club => write!(f, "♣"),
            Suit::Diamond => write!(f, "♦"),
        }?;

        Ok(())
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank(13) => write!(f, "K"),
            Rank(12) => write!(f, "Q"),
            Rank(11) => write!(f, "J"),
            Rank(1) => write!(f, "A"),
            Rank(n) => write!(f, "{n}"),
        }?;

        Ok(())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:PILE_WIDTH$}", format!("{}{}", self.rank, self.suit))?;
        Ok(())
    }
}

impl fmt::Display for PileCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.face {
            Face::Up => write!(f, "{:PILE_WIDTH$}", self.card),
            Face::Down => write!(f, "{:PILE_WIDTH$}", "🂠"),
        }?;

        Ok(())
    }
}

impl fmt::Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(card) = self.top() {
            write!(f, "{card}")?;
        }
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_len = self.tableau.iter().map(|pile| pile.len()).max().unwrap();

        for (index, _) in self.tableau.iter().enumerate() {
            let column_number = index + 1;
            write!(f, "{:PILE_WIDTH$}", format!("Pile {column_number}"))?;
        }
        writeln!(f)?;

        for pile in &self.tableau {
            write!(f, "{:PILE_WIDTH$}", format!("({} cards)", pile.len()))?;
        }
        writeln!(f)?;

        for i in 0..max_len {
            for pile in &self.tableau {
                if let Some(card) = pile.get(i) {
                    write!(f, "{card}")?;
                } else {
                    write!(f, "{:PILE_WIDTH$}", " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn render(board: Board) {
    println!("{board}");
}

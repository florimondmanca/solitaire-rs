use std::{
    error::Error,
    fmt,
    io::{stdin, stdout, Read, Write},
};
use termion::{clear, color, cursor, raw::IntoRawMode, screen::IntoAlternateScreen};

use crate::domain::entities::{Board, Face, Pile, PileCard, Rank, Suit};

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Heart => write!(f, "♥"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Spades => write!(f, "♠"),
            Suit::Club => write!(f, "♣"),
        }?;

        Ok(())
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank(13) => write!(f, "K"),
            Rank(12) => write!(f, "Q"),
            Rank(11) => write!(f, "J"),
            Rank(1) => write!(f, "A"),
            Rank(n) => write!(f, "{}", n),
        }?;

        Ok(())
    }
}

struct PileCardSprite<'a> {
    pub x: u16,
    pub y: u16,
    pub pile_card: &'a PileCard,
}

impl<'a> PileCardSprite<'a> {
    pub fn new(x: u16, y: u16, pile_card: &'a PileCard) -> Self {
        Self { x, y, pile_card }
    }

    pub fn height(&self) -> u16 {
        4
    }
}

impl<'a> fmt::Display for PileCardSprite<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", cursor::Goto(self.x, self.y))?;
        write!(f, "┌───┐")?;

        if self.pile_card.face == Face::Up {
            let suit_color = match self.pile_card.card.suit {
                Suit::Heart | Suit::Diamond => format!("{}", color::Fg(color::Red)),
                _ => format!("{}", color::Fg(color::Black)),
            };

            let rank = match format!("{}", self.pile_card.card.rank).as_str() {
                "10" => "10".into(),
                s => format!(" {s}"),
            };

            write!(f, "{}", cursor::Goto(self.x, self.y + 1))?;
            write!(
                f,
                "│{}{}{}{}│",
                suit_color,
                self.pile_card.card.suit,
                color::Fg(color::Reset),
                rank,
            )?;

            write!(f, "{}", cursor::Goto(self.x, self.y + 2))?;
            write!(
                f,
                "│ {}{}{} │",
                suit_color,
                self.pile_card.card.suit,
                color::Fg(color::Reset)
            )?;
        } else {
            for dy in &[1, 2] {
                write!(f, "{}", cursor::Goto(self.x, self.y + dy))?;
                write!(
                    f,
                    "│{}▚▚▚{}│",
                    color::Fg(color::LightBlue),
                    color::Fg(color::Reset)
                )?;
            }
        }

        write!(f, "{}", cursor::Goto(self.x, self.y + 3))?;
        write!(f, "└───┘")?;

        Ok(())
    }
}

struct PileSprite<'a> {
    pub number: u16,
    pub x: u16,
    pub y: u16,
    pub pile_cards: Vec<PileCardSprite<'a>>,
}

impl<'a> PileSprite<'a> {
    pub fn new(number: u16, x: u16, y: u16, pile: &'a Pile) -> Self {
        let mut dy = 2;

        let pile_cards = pile
            .iter()
            .enumerate()
            .map(|(index, pile_card)| {
                let sprite = PileCardSprite::new(x, y + dy, pile_card);

                dy += match index {
                    i if i == pile.len() - 1 => sprite.height(),
                    _ => 2, // Covered by next card
                };

                sprite
            })
            .collect();

        Self {
            number,
            x,
            y,
            pile_cards,
        }
    }
}

impl<'a> fmt::Display for PileSprite<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", cursor::Goto(self.x, self.y))?;
        write!(f, "{}", format!("Pile {}", self.number))?;
        write!(f, "{}", cursor::Goto(self.x, self.y + 1))?;
        write!(f, "{}", format!("({} cards)", self.pile_cards.len()))?;

        for pile_card in &self.pile_cards {
            write!(f, "{}", pile_card)?;
        }

        Ok(())
    }
}

struct BoardSprite<'a> {
    tableau: Vec<PileSprite<'a>>,
}

impl<'a> BoardSprite<'a> {
    pub fn new(board: &'a Board) -> Self {
        let dx = 12;
        let x0 = 1;
        let y0 = 1;

        let tableau = board
            .tableau
            .iter()
            .enumerate()
            .map(|(index, pile)| (index as u16, pile))
            .map(|(index, pile)| PileSprite::new(index + 1, x0 + index * dx, y0, pile))
            .collect();

        Self { tableau }
    }
}

impl<'a> fmt::Display for BoardSprite<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", clear::All)?;

        for pile in &self.tableau {
            write!(f, "{}", pile)?;
        }

        write!(f, "\n\r")?;

        Ok(())
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let board = Board::new();
    let board_sprite = BoardSprite::new(&board);

    let mut screen = cursor::HideCursor::from(stdout().into_alternate_screen()?.into_raw_mode()?);
    write!(screen, "{}", board_sprite)?;
    write!(screen, "Hint: 'q' will exit\r\n")?;

    while let Some(Ok(b)) = stdin().lock().bytes().next() {
        match b {
            b'q' => break,
            _ => {}
        }
    }

    Ok(())
}

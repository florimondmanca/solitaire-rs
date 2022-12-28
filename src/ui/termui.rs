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

struct SlotSprite {
    pub x: u16,
    pub y: u16,
}

impl SlotSprite {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for SlotSprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", cursor::Goto(self.x, self.y))?;
        write!(f, "┌╌╌╌┐")?;

        write!(f, "{}", cursor::Goto(self.x, self.y + 1))?;
        write!(f, "╎   ╎",)?;

        write!(f, "{}", cursor::Goto(self.x, self.y + 2))?;
        write!(f, "╎   ╎")?;

        write!(f, "{}", cursor::Goto(self.x, self.y + 3))?;
        write!(f, "└╌╌╌┘")?;

        Ok(())
    }
}

struct StackedPileSprite<'a> {
    pub x: u16,
    pub y: u16,
    pub pile: &'a Pile,
}

impl<'a> StackedPileSprite<'a> {
    pub fn new(x: u16, y: u16, pile: &'a Pile) -> Self {
        Self { x, y, pile }
    }

    pub fn width(&self) -> u16 {
        5
    }

    pub fn height(&self) -> u16 {
        4
    }
}

impl<'a> fmt::Display for StackedPileSprite<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(card) = self.pile.last() {
            write!(f, "{}", PileCardSprite::new(self.x, self.y, card))?;
        } else {
            write!(f, "{}", SlotSprite::new(self.x, self.y))?;
        }

        Ok(())
    }
}

struct FannedPileSprite<'a> {
    pub pile_cards: Vec<PileCardSprite<'a>>,
}

impl<'a> FannedPileSprite<'a> {
    pub fn new(x: u16, y: u16, pile: &'a Pile) -> Self {
        let last_index = pile.len() - 1;
        let mut pile_cards = Vec::new();
        let mut dy = 0;

        for (index, pile_card) in pile.iter().enumerate() {
            let sprite = PileCardSprite::new(x, y + dy, pile_card);
            dy += match index {
                i if i == last_index => sprite.height(), // Last card is visible in full.
                _ => 2, // Other cards are covered by the card after them.
            };
            pile_cards.push(sprite);
        }

        Self { pile_cards }
    }

    pub fn width(&self) -> u16 {
        5
    }
}

impl<'a> fmt::Display for FannedPileSprite<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for pile_card in &self.pile_cards {
            write!(f, "{}", pile_card)?;
        }

        Ok(())
    }
}

struct BoardSprite<'a> {
    tableau: Vec<FannedPileSprite<'a>>,
    foundations: Vec<StackedPileSprite<'a>>,
    stock: StackedPileSprite<'a>,
    waste: StackedPileSprite<'a>,
}

impl<'a> BoardSprite<'a> {
    pub fn new(board: &'a Board) -> Self {
        let stock = StackedPileSprite::new(1, 1, &board.stock);
        let waste = StackedPileSprite::new(stock.x + stock.width() + 1, 1, &board.waste);

        let mut tableau = Vec::new();
        let x0 = 1;
        let mut dx = 0;
        let y0 = stock.y + stock.height();

        for pile in board.tableau.iter() {
            let sprite = FannedPileSprite::new(x0 + dx, y0, pile);
            dx += sprite.width() + 1;
            tableau.push(sprite);
        }

        let mut foundations = Vec::new();
        let x0 = waste.y + 3 * (waste.width() + 1);
        let mut dx = 0;

        for pile in board.foundations.iter() {
            let sprite = StackedPileSprite::new(x0 + dx, 1, pile);
            dx += sprite.width() + 1;
            foundations.push(sprite);
        }

        Self {
            tableau,
            foundations,
            stock,
            waste,
        }
    }
}

impl<'a> fmt::Display for BoardSprite<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", clear::All)?;

        write!(f, "{}", self.stock)?;
        write!(f, "{}", self.waste)?;

        for pile in &self.foundations {
            write!(f, "{}", pile)?;
        }

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

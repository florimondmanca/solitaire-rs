use std::fmt;
use std::io;
use termion::{color, cursor};

use crate::domain::entities::{Card, Rank, Suit};
use crate::ui::termui::lib::HasSize;
use crate::ui::termui::lib::Loc;
use crate::ui::termui::lib::RenderResult;
use crate::ui::termui::lib::Size;
use crate::ui::termui::lib::Widget;

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

#[derive(Clone)]
pub struct CardWidget {
    size: Size,
    card: Card,
}

impl CardWidget {
    pub fn new(card: Card) -> Self {
        Self {
            size: Size::new(5, 4),
            card,
        }
    }
}

impl HasSize for CardWidget {
    fn get_size(&self) -> &Size {
        &self.size
    }
}

impl<W: io::Write> Widget<W> for CardWidget {
    fn render(&self, stdout: &mut W, loc: Loc) -> RenderResult {
        let Loc { x, y } = loc;

        write!(stdout, "{}", cursor::Goto(x, y))?;
        write!(stdout, "┌───┐")?;

        if self.card.is_visible() {
            let suit_color = match self.card.suit {
                Suit::Heart | Suit::Diamond => format!("{}", color::Fg(color::Red)),
                _ => format!("{}", color::Fg(color::Black)),
            };

            let rank = match format!("{}", self.card.rank).as_str() {
                "10" => "10".into(),
                s => format!(" {s}"),
            };

            write!(stdout, "{}", cursor::Goto(x, y + 1))?;
            write!(
                stdout,
                "│{}{}{}{}│",
                suit_color,
                self.card.suit,
                color::Fg(color::Reset),
                rank,
            )?;

            write!(stdout, "{}", cursor::Goto(x, y + 2))?;
            write!(
                stdout,
                "│ {}{}{} │",
                suit_color,
                self.card.suit,
                color::Fg(color::Reset)
            )?;
        } else {
            for dy in &[1, 2] {
                write!(stdout, "{}", cursor::Goto(x, y + dy))?;
                write!(
                    stdout,
                    "│{}▚▚▚{}│",
                    color::Fg(color::LightBlue),
                    color::Fg(color::Reset)
                )?;
            }
        }

        write!(stdout, "{}", cursor::Goto(x, y + 3))?;
        write!(stdout, "└───┘")?;

        Ok(())
    }
}

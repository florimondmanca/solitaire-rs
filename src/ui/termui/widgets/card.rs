use std::fmt;
use std::io;
use termion::{color, cursor};

use crate::domain::entities::{Card, Rank, Suit};
use crate::ui::termui::lib::HasSize;
use crate::ui::termui::lib::Loc;
use crate::ui::termui::lib::RenderResult;
use crate::ui::termui::lib::Size;
use crate::ui::termui::lib::Widget;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    Hovered,
    Picked,
}

pub static HOVER_COLOR: color::LightCyan = color::LightCyan;
pub static PICKED_COLOR: color::Yellow = color::Yellow;

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Heart => write!(f, "{}♥{}", color::Fg(color::Red), color::Fg(color::Reset)),
            Suit::Diamond => write!(f, "{}♦{}", color::Fg(color::Red), color::Fg(color::Reset)),
            Suit::Spades => write!(f, "{}♠{}", color::Fg(color::Black), color::Fg(color::Reset)),
            Suit::Club => write!(f, "{}♣{}", color::Fg(color::Black), color::Fg(color::Reset)),
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
    state: Option<CardState>,
}

impl CardWidget {
    pub fn new(card: Card, state: Option<CardState>) -> Self {
        Self {
            size: Size::new(5, 4),
            card,
            state,
        }
    }

    fn colorize(&self, s: &str) -> String {
        match self.state {
            Some(CardState::Picked) => {
                format!("{}{s}{}", color::Fg(PICKED_COLOR), color::Fg(color::Reset))
            }
            Some(CardState::Hovered) => {
                format!("{}{s}{}", color::Fg(HOVER_COLOR), color::Fg(color::Reset))
            }
            _ => s.into(),
        }
    }
}

impl HasSize for CardWidget {
    fn get_size(&self) -> &Size {
        &self.size
    }
}

impl<W: io::Write> Widget<W> for CardWidget {
    fn render(&self, f: &mut W, loc: Loc) -> RenderResult {
        let Loc { x, y } = loc;

        // Levitation effect
        let x = match self.state {
            Some(CardState::Hovered) => x + 1,
            Some(CardState::Picked) => x + 1,
            _ => x,
        };

        write!(f, "{}", cursor::Goto(x, y))?;
        write!(f, "{}", self.colorize("┌───┐"))?;

        if self.card.is_visible() {
            let rank = match format!("{}", self.card.rank).as_str() {
                "10" => "10".into(),
                s => format!(" {s}"),
            };

            write!(f, "{}", cursor::Goto(x, y + 1))?;
            write!(
                f,
                "{b}{}{}{b}",
                self.card.suit,
                rank,
                b = self.colorize("│"),
            )?;

            write!(f, "{}", cursor::Goto(x, y + 2))?;
            write!(f, "{b} {} {b}", self.card.suit, b = self.colorize("│"),)?;
        } else {
            for dy in &[1, 2] {
                write!(f, "{}", cursor::Goto(x, y + dy))?;
                write!(
                    f,
                    "{b}{}▚▚▚{}{b}",
                    color::Fg(color::LightBlue),
                    color::Fg(color::Reset),
                    b = self.colorize("│"),
                )?;
            }
        }

        write!(f, "{}", cursor::Goto(x, y + 3))?;
        write!(f, "{}", self.colorize("└───┘"))?;

        if self.state == Some(CardState::Hovered) {
            write!(f, "{}", cursor::Goto(x + 2, y + 4))?;
            write!(f, "{}", self.colorize("^"))?;
        }

        Ok(())
    }
}

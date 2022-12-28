use std::fmt;

use termion::{color, cursor};

use crate::domain::entities::{Card, Rank, Suit};

pub static CARD_HEIGHT: u16 = 4;

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

pub fn render_card<W: std::io::Write>(
    screen: &mut W,
    card: &Card,
    x: u16,
    y: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    write!(screen, "{}", cursor::Goto(x, y))?;
    write!(screen, "┌───┐")?;

    if card.is_visible() {
        let suit_color = match card.suit {
            Suit::Heart | Suit::Diamond => format!("{}", color::Fg(color::Red)),
            _ => format!("{}", color::Fg(color::Black)),
        };

        let rank = match format!("{}", card.rank).as_str() {
            "10" => "10".into(),
            s => format!(" {s}"),
        };

        write!(screen, "{}", cursor::Goto(x, y + 1))?;
        write!(
            screen,
            "│{}{}{}{}│",
            suit_color,
            card.suit,
            color::Fg(color::Reset),
            rank,
        )?;

        write!(screen, "{}", cursor::Goto(x, y + 2))?;
        write!(
            screen,
            "│ {}{}{} │",
            suit_color,
            card.suit,
            color::Fg(color::Reset)
        )?;
    } else {
        for dy in &[1, 2] {
            write!(screen, "{}", cursor::Goto(x, y + dy))?;
            write!(
                screen,
                "│{}▚▚▚{}│",
                color::Fg(color::LightBlue),
                color::Fg(color::Reset)
            )?;
        }
    }

    write!(screen, "{}", cursor::Goto(x, y + 3))?;
    write!(screen, "└───┘")?;

    Ok(())
}

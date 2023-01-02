use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::Widget;

use crate::domain::entities::{Card, Rank, Suit};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    Hovered,
    Picked,
}

pub static HOVER_COLOR: Color = Color::LightCyan;
pub static PICKED_COLOR: Color = Color::Yellow;

impl Widget for Suit {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let x = area.x;
        let y = area.y;

        let (symbol, fg) = match self {
            Suit::Heart => ("♥", Color::Red),
            Suit::Diamond => ("♦", Color::Red),
            Suit::Spades => ("♠", Color::Black),
            Suit::Club => ("♣", Color::Black),
        };

        buf.set_string(x, y, symbol, Style::default().fg(fg));
    }
}

impl Widget for Rank {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let x = area.x;
        let y = area.y;
        let symbol = match self.0 {
            13 => "K".into(),
            12 => "Q".into(),
            11 => "J".into(),
            1 => "A".into(),
            s => s.to_string(),
        };
        buf.set_string(x, y, &format!("{symbol:>2}"), Style::default());
    }
}

#[derive(Clone)]
pub struct CardWidget {
    card: Card,
    state: Option<CardState>,
}

impl CardWidget {
    pub fn new(card: Card, state: Option<CardState>) -> Self {
        Self { card, state }
    }
}

impl Widget for CardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let x = area.x;
        let y = area.y;

        // Levitation effect
        let (x, fg) = match self.state {
            Some(CardState::Hovered) => (x + 1, HOVER_COLOR),
            Some(CardState::Picked) => (x + 1, PICKED_COLOR),
            None => (x, Color::Reset),
        };

        buf.set_string(x, y, "┌───┐", Style::default().fg(fg));

        if self.card.is_visible() {
            buf.set_string(x, y + 1, "│", Style::default().fg(fg));
            let suit_area = Rect::new(x + 1, y + 1, 1, 1);
            self.card.suit.render(suit_area, buf);
            let rank_area = Rect::new(x + 2, y + 1, 2, 1);
            self.card.rank.render(rank_area, buf);
            buf.set_string(x + 4, y + 1, "│", Style::default().fg(fg));

            buf.set_string(x, y + 2, "│", Style::default().fg(fg));
            let suit_area = Rect::new(x + 3, y + 2, 1, 1);
            self.card.suit.render(suit_area, buf);
            buf.set_string(x + 4, y + 2, "│", Style::default().fg(fg));
        } else {
            for dy in &[1, 2] {
                buf.set_string(x, y + dy, "│", Style::default().fg(fg));
                buf.set_string(x + 1, y + dy, "▚▚▚", Style::default().fg(Color::LightBlue));
                buf.set_string(x + 4, y + dy, "│", Style::default().fg(fg));
            }
        }

        buf.set_string(x, y + 3, "└───┘", Style::default().fg(fg));

        if self.state == Some(CardState::Hovered) {
            buf.set_string(x + 2, y + 4, "^", Style::default().fg(fg));
        }
    }
}

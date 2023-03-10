use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::Widget;

use crate::domain::{Card, Rank, Suit};

static FOCUSED_COLOR: Color = Color::Cyan;
static PICKED_COLOR: Color = Color::Yellow;
static COVER_COLOR: Color = Color::LightBlue;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardAppearance {
    Focused,
    Picked,
}

impl Widget for Suit {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Rect { x, y, .. } = area;

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
        let Rect { x, y, .. } = area;

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
    appearance: Option<CardAppearance>,
}

impl CardWidget {
    pub fn new(card: Card, appearance: Option<CardAppearance>) -> Self {
        Self { card, appearance }
    }

    pub fn width() -> u16 {
        5
    }

    pub fn height() -> u16 {
        4
    }

    pub fn hint_height() -> u16 {
        2
    }
}

impl Widget for CardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Rect { x, y, .. } = area;

        // Levitation effect
        let x = match self.appearance {
            Some(CardAppearance::Focused) => x + 1,
            Some(CardAppearance::Picked) => x + 1,
            None => x,
        };

        let fg = match self.appearance {
            Some(CardAppearance::Focused) => FOCUSED_COLOR,
            Some(CardAppearance::Picked) => PICKED_COLOR,
            None => Color::Reset,
        };

        buf.set_string(x, y, "┌───┐", Style::default().fg(fg));

        if self.card.is_visible() {
            // | ♥ K |
            buf.set_string(x, y + 1, "│", Style::default().fg(fg));
            self.card.suit.render(Rect::new(x + 1, y + 1, 1, 1), buf);
            self.card.rank.render(Rect::new(x + 2, y + 1, 2, 1), buf);
            buf.set_string(x + 4, y + 1, "│", Style::default().fg(fg));

            // |   ♥ |
            buf.set_string(x, y + 2, "│", Style::default().fg(fg));
            self.card.suit.render(Rect::new(x + 3, y + 2, 1, 1), buf);
            buf.set_string(x + 4, y + 2, "│", Style::default().fg(fg));
        } else {
            for dy in &[1, 2] {
                // | ▚▚▚ |
                buf.set_string(x, y + dy, "│", Style::default().fg(fg));
                buf.set_string(x + 1, y + dy, "▚▚▚", Style::default().fg(COVER_COLOR));
                buf.set_string(x + 4, y + dy, "│", Style::default().fg(fg));
            }
        }

        buf.set_string(x, y + 3, "└───┘", Style::default().fg(fg));

        if self.appearance == Some(CardAppearance::Focused) {
            buf.set_string(x + 2, y + 4, "^", Style::default().fg(fg));
        }
    }
}

pub struct EmptySlotWidget {
    appearance: Option<CardAppearance>,
    content: [char; 2],
}

impl EmptySlotWidget {
    pub fn new(appearance: Option<CardAppearance>) -> Self {
        Self {
            appearance,
            content: [' '; 2],
        }
    }

    pub fn content(mut self, content: [char; 2]) -> Self {
        self.content = content;
        self
    }
}

impl Widget for EmptySlotWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let x = area.x;
        let y = area.y;

        let style = Style::default().fg(match self.appearance {
            Some(CardAppearance::Focused) => FOCUSED_COLOR,
            _ => Color::Reset,
        });

        buf.set_string(x, y, "┌╌╌╌┐", style);
        buf.set_string(x, y + 1, format!("╎ {} ╎", self.content[0]), style);
        buf.set_string(x, y + 2, format!("╎ {} ╎", self.content[1]), style);
        buf.set_string(x, y + 3, "└╌╌╌┘", style);
    }
}

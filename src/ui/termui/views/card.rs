use cursive::{
    theme::{BaseColor, Color},
    Printer, Vec2, View,
};

use crate::domain::{Card, Rank, Suit};

static RED: Color = Color::Dark(BaseColor::Red);
static BLACK: Color = Color::Dark(BaseColor::Black);
static CARD_BACK: Color = Color::Light(BaseColor::Blue);

impl From<Suit> for String {
    fn from(suit: Suit) -> String {
        let s = match suit {
            Suit::Heart => "♥",
            Suit::Diamond => "♦",
            Suit::Spades => "♠",
            Suit::Club => "♣",
        };

        s.into()
    }
}

impl From<Suit> for Color {
    fn from(suit: Suit) -> Self {
        match suit {
            Suit::Heart | Suit::Diamond => RED,
            Suit::Spades | Suit::Club => BLACK,
        }
    }
}

impl From<Rank> for String {
    fn from(rank: Rank) -> Self {
        match rank.0 {
            13 => "K".into(),
            12 => "Q".into(),
            11 => "J".into(),
            1 => "A".into(),
            n => n.to_string(),
        }
    }
}

pub enum CardView {
    Card(Card),
    Hidden,
    Empty,
}

impl From<Option<Card>> for CardView {
    fn from(value: Option<Card>) -> Self {
        match value {
            Some(card) => CardView::Card(card),
            None => CardView::Empty,
        }
    }
}

impl View for CardView {
    fn draw(&self, printer: &Printer) {
        let start: Vec2 = (0, 0).into();
        let size: Vec2 = (5, 4).into();

        match self {
            CardView::Card(card) => {
                printer.print_box(start, size, false);

                printer.with_color(Color::from(card.suit).into(), |printer| {
                    printer.print_hline((1, 1), 1, &String::from(card.suit));
                    printer.print_hline((3, 2), 1, &String::from(card.suit));
                });

                let rank = String::from(card.rank);
                printer.print_hline((4 - rank.len(), 1), rank.len(), &rank);
            }
            CardView::Empty => {
                let size = size - (1, 1);

                printer.print(start, "┌");
                printer.print(start + size.keep_y(), "└");
                printer.print_hline(start + (1, 0), size.x - 1, "─");
                printer.print_vline(start + (0, 1), size.y - 1, "╎");

                printer.print(start + size.keep_x(), "┐");
                printer.print(start + size, "┘");
                printer.print_hline(start + (1, 0) + size.keep_y(), size.x - 1, "─");
                printer.print_vline(start + (0, 1) + size.keep_x(), size.y - 1, "╎");
            }
            CardView::Hidden => {
                let size = size - (1, 1);

                printer.print(start, "┌");
                printer.print_hline(start + (1, 0), size.x - 1, "─");
                printer.print(start + size.keep_x(), "┐");
                printer.print_vline(start + (0, 1), 1, "╎");
                printer.print_vline(start + (0, 1) + size.keep_x(), 1, "╎");
                printer.with_color(CARD_BACK.into(), |printer| {
                    printer.print_hline(start + (1, 1), 3, "▚▚▚");
                });
            }
        };
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        match self {
            CardView::Hidden => (5, 2).into(),
            _ => (5, 4).into(),
        }
    }
}

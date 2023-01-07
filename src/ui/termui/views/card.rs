use cursive::{
    theme::{BaseColor, Color},
    Printer, Vec2, View,
};

use crate::domain::{Card, Pile, Rank, Suit};

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

trait PrintCard {
    fn print_card<T: Into<Vec2>>(&self, start: T, card: Card);
}

impl<'a, 'b> PrintCard for Printer<'a, 'b> {
    fn print_card<T: Into<Vec2>>(&self, start: T, card: Card) {
        let start: Vec2 = start.into();
        let size: Vec2 = (5, 4).into();

        self.print_box(start, size, false);

        if card.is_visible() {
            self.with_color(Color::from(card.suit).into(), |printer| {
                printer.print_hline(start + (1, 1), 1, &String::from(card.suit));
                printer.print_hline(start + (3, 2), 1, &String::from(card.suit));
            });
            let rank = match String::from(card.rank).as_str() {
                "10" => "10".into(),
                s => format!(" {s}"),
            };
            self.print_hline(start + (4 - rank.len(), 1), rank.len(), &rank);
        } else {
            self.with_color(CARD_BACK.into(), |printer| {
                printer.print_hline(start + (1, 1), 3, "▚▚▚");
                printer.print_hline(start + (1, 2), 3, "▚▚▚");
            });
        }
    }
}

pub struct FannedPileView(Pile);

impl FannedPileView {
    pub fn new(pile: Pile) -> Self {
        Self(pile)
    }
}

impl View for FannedPileView {
    fn draw(&self, printer: &Printer) {
        let mut y = 0;

        for card in self.0.iter().take(self.0.len() - 1) {
            printer.print_card((0, y), *card);
            y += 2;
        }

        match self.0.last() {
            Some(&card) => printer.print_card((0, y), card),
            None => printer.print_box((0, y), (5, 4), false),
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let y = 4 + (self.0.len() - 1).max(0) * 2;
        (5, y).into()
    }
}

pub struct StackedPileView(Pile);

impl StackedPileView {
    pub fn new(pile: Pile) -> Self {
        Self(pile)
    }

    pub fn push(&mut self, card: Card) {
        self.0.push(card);
    }
}

impl View for StackedPileView {
    fn draw(&self, printer: &Printer) {
        let start: Vec2 = (0, 0).into();
        let size: Vec2 = (5, 4).into();

        match self.0.last() {
            Some(card) => printer.print_card(start, *card),
            None => {
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
        };
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        (5, 4).into()
    }
}

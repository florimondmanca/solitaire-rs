use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::domain::{Card, CardAppearance, Pile};

use super::{card::CardWidget, FOCUSED_COLOR};

/**
 * Display a pile of cards fanned out as a column.
 */
#[derive(Clone)]
pub struct FannedPileWidget {
    pile: Pile,
    appearance: Option<CardAppearance>,
}

impl FannedPileWidget {
    pub fn new(pile: Pile, appearance: Option<CardAppearance>) -> Self {
        Self { pile, appearance }
    }
}

impl Widget for FannedPileWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.pile.is_empty() {
            let widget = EmptySlotWidget::new(self.appearance);
            widget.render(area, buf);
            return;
        }

        let mut region = area.clone();
        let last_index = self.pile.len() - 1;

        for (index, card) in self.pile.iter().enumerate() {
            let card_appearance = match (index == last_index, self.appearance) {
                (true, Some(v)) => Some(v),
                _ => None,
            };
            let widget = CardWidget::new(card.clone(), card_appearance);
            widget.render(region, buf);

            if index == last_index {
                // Last card is visible in full.
                region.y += 5;
            } else {
                // Other cards are partially covered.
                region.y += 2;
            }
        }
    }
}

/**
 * Display a pile of cards by only showing the topmost card, or an empty slot.
 */
pub struct StackedPileWidget {
    pile: Vec<Card>,
    appearance: Option<CardAppearance>,
}

impl StackedPileWidget {
    pub fn new(pile: Vec<Card>, appearance: Option<CardAppearance>) -> Self {
        Self { pile, appearance }
    }
}

impl Widget for StackedPileWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(top_card) = self.pile.last() {
            let card_widget = CardWidget::new(top_card.clone(), self.appearance);
            card_widget.render(area, buf);
        } else {
            let widget = EmptySlotWidget::new(self.appearance);
            widget.render(area, buf);
        }
    }
}

struct EmptySlotWidget {
    appearance: Option<CardAppearance>,
}

impl EmptySlotWidget {
    pub fn new(appearance: Option<CardAppearance>) -> Self {
        Self { appearance }
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
        buf.set_string(x, y + 1, "╎   ╎", style);
        buf.set_string(x, y + 2, "╎   ╎", style);
        buf.set_string(x, y + 3, "└╌╌╌┘", style);
    }
}

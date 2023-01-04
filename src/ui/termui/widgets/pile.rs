use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::domain::Pile;

use super::{card::CardWidget, CardAppearance, FOCUSED_COLOR};

pub type RangeAppearance = (CardAppearance, usize);

/**
 * Display a pile of cards fanned out as a column.
 */
pub struct FannedPileWidget {
    pile: Pile,
    appearance: Option<RangeAppearance>,
}

impl FannedPileWidget {
    pub fn new(pile: Pile, appearance: Option<RangeAppearance>) -> Self {
        Self { pile, appearance }
    }
}

impl Widget for FannedPileWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.pile.is_empty() {
            let widget =
                EmptySlotWidget::new(self.appearance.map(|(card_appearance, _)| card_appearance));
            widget.render(area, buf);
            return;
        }

        let mut region = area.clone();

        for (index, card) in self.pile.iter().enumerate() {
            let card_appearance = match &self.appearance {
                Some((card_appearance, size)) => {
                    let is_in_range = index >= self.pile.len() - size;
                    is_in_range.then(|| *card_appearance)
                }
                _ => None,
            };

            let widget = CardWidget::new(card.clone(), card_appearance);
            widget.render(region, buf);

            let is_last = index == self.pile.len() - 1;

            if is_last {
                // Last card is visible in full.
                region.y += CardWidget::height();
            } else {
                // Other cards are partially covered.
                region.y += CardWidget::hint_height();
            }
        }
    }
}

/**
 * Display a pile of cards by only showing the topmost card, or an empty slot.
 */
pub struct StackedPileWidget {
    pile: Pile,
    appearance: Option<CardAppearance>,
}

impl StackedPileWidget {
    pub fn new(pile: Pile, appearance: Option<CardAppearance>) -> Self {
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

use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::domain::Pile;

use super::card::{CardAppearance, CardWidget, EmptySlotWidget};

/**
 * Display a pile of cards fanned out as a column.
 */
pub struct FannedPileWidget<'a> {
    pile: &'a Pile,
    appearance: Option<(CardAppearance, usize)>,
}

impl<'a> FannedPileWidget<'a> {
    pub fn new(pile: &'a Pile, appearance: Option<(CardAppearance, usize)>) -> Self {
        Self { pile, appearance }
    }

    pub fn get_width(&self) -> u16 {
        CardWidget::width()
    }

    pub fn get_height(&self) -> u16 {
        let mut height = CardWidget::height();

        for _ in self.pile.iter().skip(1) {
            height += CardWidget::hint_height()
        }

        height
    }
}

impl<'a> Widget for FannedPileWidget<'a> {
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
pub struct StackedPileWidget<'a> {
    pile: &'a Pile,
    appearance: Option<CardAppearance>,
    empty_content: [char; 2],
}

impl<'a> StackedPileWidget<'a> {
    pub fn new(pile: &'a Pile, appearance: Option<CardAppearance>) -> Self {
        Self {
            pile,
            appearance,
            empty_content: [' '; 2],
        }
    }

    pub fn empty_content(mut self, content: [char; 2]) -> Self {
        self.empty_content = content;
        self
    }

    pub fn get_width(&self) -> u16 {
        CardWidget::width()
    }

    pub fn get_height(&self) -> u16 {
        CardWidget::height()
    }
}

impl<'a> Widget for StackedPileWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(top_card) = self.pile.last() {
            let card_widget = CardWidget::new(top_card.clone(), self.appearance);
            card_widget.render(area, buf);
        } else {
            let widget = EmptySlotWidget::new(self.appearance).content(self.empty_content);
            widget.render(area, buf);
        }
    }
}

use std::io;
use termion::cursor;

use crate::{
    domain::entities::Card,
    ui::termui::lib::{HasSize, Loc, RenderResult, Size, Widget},
};

use super::card::CardWidget;

/**
 * Display a pile of cards fanned out as a column.
 */
#[derive(Clone)]
pub struct FannedPileWidget {
    card_widgets: Vec<CardWidget>,
    size: Size,
}

impl FannedPileWidget {
    pub fn new(pile: Vec<Card>) -> Self {
        let height = match pile.len() {
            0 => 4,
            n => 4 + (n - 1) as u16 * 2,
        };

        Self {
            card_widgets: pile.into_iter().map(CardWidget::new).collect(),
            size: Size::new(5, height),
        }
    }
}

impl HasSize for FannedPileWidget {
    fn get_size(&self) -> &Size {
        &self.size
    }
}

impl<W: io::Write> Widget<W> for FannedPileWidget {
    fn render(&self, f: &mut W, loc: Loc) -> RenderResult {
        let mut loc = loc;
        let last_index = self.card_widgets.len() - 1;

        for (index, card) in self.card_widgets.iter().enumerate() {
            card.render(f, loc)?;

            if index == last_index {
                // Last card is visible in full.
                loc.y += card.get_height();
            } else {
                // Other cards are partially covered.
                loc.y += 2;
            }
        }

        Ok(())
    }
}

/**
 * Display a pile of cards by only showing the topmost card, or an empty slot.
 */
pub struct StackedPileWidget {
    pile: Vec<Card>,
    size: Size,
}

impl StackedPileWidget {
    pub fn new(pile: Vec<Card>) -> Self {
        Self {
            pile,
            size: Size::new(5, 4),
        }
    }
}

impl HasSize for StackedPileWidget {
    fn get_size(&self) -> &Size {
        &self.size
    }
}

impl<W: io::Write> Widget<W> for StackedPileWidget {
    fn render(&self, f: &mut W, loc: Loc) -> RenderResult {
        if let Some(top_card) = self.pile.last() {
            let card_widget = CardWidget::new(top_card.clone());
            card_widget.render(f, loc)
        } else {
            let widget = EmptySlotWidget::new();
            widget.render(f, loc)
        }
    }
}

struct EmptySlotWidget {
    size: Size,
}

impl EmptySlotWidget {
    pub fn new() -> Self {
        Self {
            size: Size::new(5, 4),
        }
    }
}

impl HasSize for EmptySlotWidget {
    fn get_size(&self) -> &Size {
        &self.size
    }
}

impl<W: io::Write> Widget<W> for EmptySlotWidget {
    fn render(&self, f: &mut W, loc: Loc) -> RenderResult {
        let Loc { x, y } = loc;

        write!(f, "{}", cursor::Goto(x, y))?;
        write!(f, "┌╌╌╌┐")?;
        write!(f, "{}", cursor::Goto(x, y + 1))?;
        write!(f, "╎   ╎",)?;
        write!(f, "{}", cursor::Goto(x, y + 2))?;
        write!(f, "╎   ╎")?;
        write!(f, "{}", cursor::Goto(x, y + 3))?;
        write!(f, "└╌╌╌┘")?;

        Ok(())
    }
}

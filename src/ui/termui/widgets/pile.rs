use std::io;
use termion::{color, cursor};

use crate::{
    domain::entities::{Card, Pile},
    ui::termui::lib::{HasSize, Loc, RenderResult, Size, Widget},
};

use super::{card::CardWidget, CardState, HOVER_COLOR};

/**
 * Display a pile of cards fanned out as a column.
 */
#[derive(Clone)]
pub struct FannedPileWidget {
    pile: Pile,
    size: Size,
    state: Option<CardState>,
}

impl FannedPileWidget {
    pub fn new(pile: Pile, state: Option<CardState>) -> Self {
        let height = match pile.len() {
            0 => 4,
            n => 4 + (n - 1) as u16 * 2,
        };

        Self {
            pile,
            size: Size::new(5, height),
            state,
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
        if self.pile.is_empty() {
            let widget = EmptySlotWidget::new(self.state);
            widget.render(f, loc)?;
            return Ok(());
        }

        let mut loc = loc;
        let last_index = self.pile.len() - 1;

        for (index, card) in self.pile.iter().enumerate() {
            let state = match (index == last_index, self.state) {
                (true, Some(s)) => Some(s),
                _ => None,
            };
            let widget = CardWidget::new(card.clone(), state);
            widget.render(f, loc)?;

            if index == last_index {
                // Last card is visible in full.
                loc.y += widget.get_height();
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
    state: Option<CardState>,
}

impl StackedPileWidget {
    pub fn new(pile: Vec<Card>, state: Option<CardState>) -> Self {
        Self {
            pile,
            size: Size::new(5, 4),
            state,
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
            let card_widget = CardWidget::new(top_card.clone(), self.state);
            card_widget.render(f, loc)
        } else {
            let widget = EmptySlotWidget::new(self.state);
            widget.render(f, loc)
        }
    }
}

struct EmptySlotWidget {
    size: Size,
    state: Option<CardState>,
}

impl EmptySlotWidget {
    pub fn new(state: Option<CardState>) -> Self {
        Self {
            size: Size::new(5, 4),
            state,
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

        if self.state == Some(CardState::Hovered) {
            write!(f, "{}", color::Fg(HOVER_COLOR))?;
        }
        write!(f, "{}", cursor::Goto(x, y))?;
        write!(f, "┌╌╌╌┐")?;
        write!(f, "{}", cursor::Goto(x, y + 1))?;
        write!(f, "╎   ╎",)?;
        write!(f, "{}", cursor::Goto(x, y + 2))?;
        write!(f, "╎   ╎")?;
        write!(f, "{}", cursor::Goto(x, y + 3))?;
        write!(f, "└╌╌╌┘")?;
        if self.state == Some(CardState::Hovered) {
            write!(f, "{}", color::Fg(color::Reset))?;
        }

        Ok(())
    }
}

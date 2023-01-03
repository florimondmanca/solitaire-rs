use super::{Board, Target};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardAppearance {
    Focused,
    Picked,
}

pub struct Selection {
    current_target: Target,
    picked_target: Option<Target>,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            current_target: Target::Stock,
            picked_target: None,
        }
    }
}

impl Selection {
    pub fn get_card_appearance(&self, target: Target) -> Option<CardAppearance> {
        match (self.current_target, self.picked_target) {
            (t, _) if t == target => Some(CardAppearance::Focused),
            (_, Some(t)) if t == target => Some(CardAppearance::Picked),
            _ => None,
        }
    }

    pub fn maybe_act_on_current_target(&mut self, board: &mut Board) -> bool {
        if let Some(picked_target) = self.picked_target {
            // User had previously picked a card.
            // They just chosed where it should be transferred.
            board.maybe_transfer(picked_target, self.current_target);
            self.picked_target = None;
            return true;
        }

        // User has selected a card.
        // If it's visible, pick it, otherwise, reveal it.
        if let Some(card) = board.get_mut(self.current_target).unwrap().last_mut() {
            if card.is_visible() {
                self.picked_target = Some(self.current_target);
            } else {
                card.reveal();
            }
            return true;
        };

        false
    }

    pub fn maybe_move_current_target_to_a_foundation(&mut self, board: &mut Board) -> bool {
        if self.picked_target.is_some() {
            // Can't move cards to foundations while a card is picked.
            return false;
        }

        board.maybe_move_to_a_foundation(self.current_target)
    }

    pub fn maybe_move_top_stock_card_to_waste(&mut self, board: &mut Board) -> bool {
        if self.current_target != Target::Stock {
            // Can only trash a card from the stock.
            return false;
        }

        board.maybe_move_top_stock_card_to_waste()
    }

    pub fn focus_previous_target(&mut self, board: &mut Board) {
        self.current_target = board.get_previous_target(self.current_target);
    }

    pub fn focus_next_target(&mut self, board: &mut Board) {
        self.current_target = board.get_next_target(self.current_target);
    }
}

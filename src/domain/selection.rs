use super::{Board, Target};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardAppearance {
    Focused,
    Picked,
}

pub struct RangeAppearance {
    pub card_appearance: CardAppearance,
    pub size: usize,
}

pub struct Selection {
    current_target: Target,
    picked_target: Option<Target>,
    num_selected_cards: usize,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            current_target: Target::Stock,
            picked_target: None,
            num_selected_cards: 1,
        }
    }
}

impl Selection {
    fn is_current(&self, target: Target) -> bool {
        self.current_target == target
    }

    fn is_picked(&self, target: Target) -> bool {
        self.picked_target.map_or(false, |t| t == target)
    }

    pub fn get_card_appearance(&self, target: Target) -> Option<CardAppearance> {
        if self.is_current(target) {
            return Some(CardAppearance::Focused);
        }

        if self.is_picked(target) {
            return Some(CardAppearance::Picked);
        }

        None
    }

    pub fn get_range_appearance(&self, index: usize) -> Option<RangeAppearance> {
        let target = Target::Pile(index);

        if self.is_current(target) {
            let size = match self.picked_target {
                None => self.num_selected_cards,
                Some(t) if t == target => self.num_selected_cards,
                _ => 1,
            };

            return Some(RangeAppearance {
                card_appearance: CardAppearance::Focused,
                size,
            });
        }

        if self.is_picked(target) {
            return Some(RangeAppearance {
                card_appearance: CardAppearance::Picked,
                size: self.num_selected_cards,
            });
        }

        None
    }

    pub fn maybe_act_on_current_target(&mut self, board: &mut Board) -> bool {
        if let Some(picked_target) = self.picked_target {
            // User had previously picked a card.
            // They just chosed where it should be transferred.
            board.maybe_transfer(picked_target, self.current_target, self.num_selected_cards);
            self.picked_target = None;
            self.num_selected_cards = 1;
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

        if self.picked_target.is_none() {
            self.num_selected_cards = 1;
        }
    }

    pub fn focus_next_target(&mut self, board: &mut Board) {
        self.current_target = board.get_next_target(self.current_target);

        if self.picked_target.is_none() {
            self.num_selected_cards = 1;
        }
    }

    pub fn maybe_increment_card_range(&mut self, board: &mut Board) -> bool {
        match self.current_target {
            Target::Pile(index) => {
                let pile = board.get(Target::Pile(index)).unwrap();

                if pile.is_empty() {
                    return false;
                }

                if self.num_selected_cards == pile.len() {
                    return false;
                }

                let card_above = pile[pile.len() - (self.num_selected_cards + 1)];

                if !card_above.is_visible() {
                    return false;
                }

                self.num_selected_cards += 1;

                true
            }
            _ => false,
        }
    }

    pub fn maybe_decrement_card_range(&mut self) -> bool {
        match self.current_target {
            Target::Pile(_) => {
                if self.num_selected_cards > 1 {
                    self.num_selected_cards -= 1;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

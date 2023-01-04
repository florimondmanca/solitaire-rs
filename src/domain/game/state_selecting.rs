use crate::domain::{Board, Target};

use super::{state_base::GameState, state_transferring::TransferringState, Action, TargetStatus};

pub struct SelectingState {
    current_target: Target,
    num_selected_cards: usize,
}

impl Default for SelectingState {
    fn default() -> Self {
        Self::new(Target::Stock)
    }
}

impl SelectingState {
    pub fn new(current_target: Target) -> Self {
        Self {
            current_target,
            num_selected_cards: 1,
        }
    }

    fn maybe_reveal_or_pick(&mut self, board: &mut Board) -> (bool, Option<Box<dyn GameState>>) {
        if let Some(top_card) = board.get_mut(self.current_target).unwrap().last_mut() {
            if !top_card.is_visible() {
                top_card.reveal();
                return (true, None);
            }

            let new_state = TransferringState::new(self.current_target, self.num_selected_cards);
            return (true, Some(Box::new(new_state)));
        };

        (false, None)
    }

    fn maybe_increment_card_range(&mut self, board: &Board) -> bool {
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

impl GameState for SelectingState {
    fn get_status_of(&self, target: Target) -> Option<TargetStatus> {
        (target == self.current_target).then(|| TargetStatus::Current {
            num_cards: self.num_selected_cards,
        })
    }

    fn handle(&mut self, action: Action) -> (bool, Option<Box<dyn GameState>>) {
        match action {
            Action::TargetPrevious(board) => {
                let target = board.get_previous_target(self.current_target);
                (true, Some(Box::new(SelectingState::new(target))))
            }
            Action::TargetNext(board) => {
                let target = board.get_next_target(self.current_target);
                (true, Some(Box::new(SelectingState::new(target))))
            }
            Action::IncreaseRange(board) => (self.maybe_increment_card_range(board), None),
            Action::DecreaseRange => (self.maybe_decrement_card_range(), None),
            Action::Act(board) => self.maybe_reveal_or_pick(board),
            Action::Build(board) => (board.maybe_move_to_a_foundation(self.current_target), None),
            Action::Discard(board) => (board.maybe_move_top_stock_card_to_waste(), None),
        }
    }
}

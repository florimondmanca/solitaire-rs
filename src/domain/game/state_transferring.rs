use crate::domain::Target;

use super::{state_base::GameState, state_selecting::SelectingState, Action, TargetStatus};

pub struct TransferringState {
    current_target: Target,
    num_picked_cards: usize,
    picked_target: Target,
}

impl TransferringState {
    pub fn new(current_target: Target, num_picked_cards: usize) -> Self {
        Self {
            current_target,
            num_picked_cards,
            picked_target: current_target,
        }
    }
}

impl GameState for TransferringState {
    fn get_status_of(&self, target: Target) -> Option<TargetStatus> {
        if target == self.picked_target {
            return Some(TargetStatus::Picked {
                num_cards: self.num_picked_cards,
            });
        }

        if target == self.current_target {
            return Some(TargetStatus::Current { num_cards: 1 });
        }

        None
    }

    fn handle(&mut self, action: Action) -> (bool, Option<Box<dyn GameState>>) {
        match action {
            Action::TargetPrevious(board) => {
                self.current_target = board.get_previous_target(self.current_target);
                (true, None)
            }
            Action::TargetNext(board) => {
                self.current_target = board.get_next_target(self.current_target);
                (true, None)
            }
            Action::Act(board) => {
                board.maybe_transfer(
                    self.picked_target,
                    self.current_target,
                    self.num_picked_cards,
                );

                let new_state = SelectingState::new(self.current_target);
                (true, Some(Box::new(new_state)))
            }
            _ => (false, None),
        }
    }
}

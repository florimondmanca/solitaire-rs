use crate::domain::Target;

use super::Action;

pub enum TargetStatus {
    Current { num_cards: usize },
    Picked { num_cards: usize },
}

pub trait GameState {
    fn get_status_of(&self, target: Target) -> Option<TargetStatus>;
    fn handle(&mut self, action: Action) -> (bool, Option<Box<dyn GameState>>);
}

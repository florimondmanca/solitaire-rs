use super::{
    super::Target, state_base::GameState, state_selecting::SelectingState, Action, TargetStatus,
};

pub struct StateMachine {
    current_state: Box<dyn GameState>,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self {
            current_state: Box::new(SelectingState::default()),
        }
    }
}

impl StateMachine {
    pub fn get_status_of(&self, target: Target) -> Option<TargetStatus> {
        self.current_state.get_status_of(target)
    }

    pub fn handle(&mut self, action: Action) -> bool {
        let (changed, new_state) = self.current_state.handle(action);

        if let Some(new_state) = new_state {
            self.move_to(new_state);
        }

        changed
    }

    pub fn move_to(&mut self, new_state: Box<dyn GameState>) {
        self.current_state = new_state;
    }
}

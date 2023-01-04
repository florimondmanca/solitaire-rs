use std::{cell::RefCell, rc::Rc};

use crate::domain::{Board, StateMachine};

pub struct Container {
    board: Rc<RefCell<Board>>,
    state_machine: Rc<RefCell<StateMachine>>,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            board: Rc::new(RefCell::new(Board::default())),
            state_machine: Rc::new(RefCell::new(StateMachine::default())),
        }
    }
}

impl Container {
    pub fn get_board(&self) -> Rc<RefCell<Board>> {
        Rc::clone(&self.board)
    }

    pub fn get_state_machine(&self) -> Rc<RefCell<StateMachine>> {
        Rc::clone(&self.state_machine)
    }
}

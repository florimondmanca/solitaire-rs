use std::{cell::RefCell, rc::Rc};

use crate::domain::{Board, Selection};

pub struct Container {
    board: Rc<RefCell<Board>>,
    selection: Rc<RefCell<Selection>>,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            board: Rc::new(RefCell::new(Board::default())),
            selection: Rc::new(RefCell::new(Selection::default())),
        }
    }
}

impl Container {
    pub fn get_board(&self) -> Rc<RefCell<Board>> {
        Rc::clone(&self.board)
    }

    pub fn get_selection(&self) -> Rc<RefCell<Selection>> {
        Rc::clone(&self.selection)
    }
}

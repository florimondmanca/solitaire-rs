use crate::domain::entities::{Board, Target};

use super::widgets::CardState;

pub struct App {
    board: Board,
    running: bool,
    dirty: bool,
    target: Target,
    picked: Option<Target>,
}

impl App {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            running: true,
            dirty: true,
            target: Target::Stock,
            picked: None,
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_state(&self, target: Target) -> Option<CardState> {
        match (self.target, self.picked) {
            (t, _) if t == target => Some(CardState::Hovered),
            (_, Some(t)) if t == target => Some(CardState::Picked),
            _ => None,
        }
    }

    pub fn on_keypress(&mut self, key: char) {
        match key {
            'q' => self.running = false,
            ' ' => self.on_press_space(),
            '\n' => self.on_press_enter(),
            'w' => self.on_press_w(),
            _ => {}
        }
    }

    fn on_press_space(&mut self) {
        if let Some(source) = self.picked {
            // User had previously picked a card.
            // They just chosed where it should be transferred.
            self.board.maybe_transfer(source, self.target);
            self.picked = None;
            self.dirty = true;
            return;
        }

        // User has selected a card.
        // If it's visible, pick it, otherwise, reveal it.
        if let Some(card) = self.board.get_mut(self.target).unwrap().last_mut() {
            if card.is_visible() {
                self.picked = Some(self.target);
            } else {
                card.reveal();
            }
            self.dirty = true;
        };
    }

    fn on_press_enter(&mut self) {
        if let Some(_) = self.picked {
            // Can't move cards to foundations while a card is picked.
            return;
        }

        if self.board.maybe_move_to_foundation(self.target) {
            self.dirty = true;
        }
    }

    fn on_press_w(&mut self) {
        if self.target != Target::Stock {
            // Can only trash a card from the stock.
            return;
        }

        if self.board.maybe_move_to_waste() {
            self.dirty = true;
        }
    }

    pub fn on_left(&mut self) {
        self.target = self.board.rotate_left(self.target);
        self.dirty = true;
    }

    pub fn on_right(&mut self) {
        self.target = self.board.rotate_right(self.target);
        self.dirty = true;
    }
}

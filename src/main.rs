use solitaire_rs::{domain::entities::Board, ui::termui};

fn main() {
    let board = Board::new();
    termui::render(board);
}

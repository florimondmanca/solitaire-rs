use cursive::{
    view::{Margins, Resizable},
    views::{LinearLayout, PaddedView},
    View,
};

use crate::domain::Board;

use super::card::CardView;

pub fn make_app_layout(board: &Board) -> impl View {
    PaddedView::new(
        Margins::trbl(2, 2, 2, 2),
        LinearLayout::horizontal()
            .child(PaddedView::new(Margins::lr(0, 2), make_hands(board)))
            .child(make_tableau(board))
            .child(PaddedView::new(Margins::lr(2, 0), make_foundations(board)).full_width()),
    )
}

fn make_hands(board: &Board) -> impl View {
    let mut hands = LinearLayout::vertical();

    hands.add_child(CardView::from(board.get_stock().last().copied()));
    hands.add_child(CardView::from(board.get_waste().last().copied()));

    hands
}

fn make_tableau(board: &Board) -> impl View {
    let mut tableau = LinearLayout::horizontal();

    for pile in board.get_tableau() {
        let mut column = LinearLayout::vertical();
        for _ in pile.iter().take(pile.len() - 1) {
            column.add_child(CardView::Hidden);
        }
        column.add_child(CardView::from(pile.last().copied()));
        tableau.add_child(column);
    }

    tableau
}

fn make_foundations(board: &Board) -> impl View {
    let mut foundations = LinearLayout::vertical();

    for pile in board.get_foundations() {
        foundations.add_child(CardView::from(pile.last().copied()));
    }

    foundations
}

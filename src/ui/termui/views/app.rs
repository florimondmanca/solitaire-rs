use std::{cell::RefCell, rc::Rc};

use cursive::{
    event::{Event, EventResult},
    view::{Margins, Nameable, Resizable},
    views::{Canvas, LinearLayout, PaddedView},
    View,
};

use crate::{
    domain::{Action, Board},
    infrastructure::Container,
};

use super::card::{FannedPileView, StackedPileView};

fn ui(container: &Container) -> impl View {
    let state_machine = container.get_state_machine();
    let board = container.get_board();

    let view = PaddedView::new(
        Margins::trbl(2, 2, 2, 2),
        LinearLayout::horizontal()
            .child(PaddedView::new(
                Margins::lr(0, 2),
                make_hands(Rc::clone(&board)),
            ))
            .child(make_tableau(Rc::clone(&board)))
            .child(
                PaddedView::new(Margins::lr(2, 0), make_foundations(Rc::clone(&board)))
                    .full_width(),
            ),
    );

    Canvas::new(view)
        .with_draw(|view, printer| view.draw(printer))
        .with_required_size(|view, constraint| view.required_size(constraint))
        .with_layout(|view, constraint| view.layout(constraint))
        .with_on_event(move |view, event| match event {
            Event::Char('q') => EventResult::with_cb(|s| s.quit()),
            Event::Char(' ') => {
                state_machine
                    .borrow_mut()
                    .handle(Action::Act(&mut board.borrow_mut()));

                EventResult::Consumed(None)
            }
            Event::Char('w') => {
                state_machine
                    .borrow_mut()
                    .handle(Action::Discard(&mut board.borrow_mut()));

                let c = *board.borrow().get_waste().last().unwrap();

                EventResult::with_cb(move |siv| {
                    let mut waste = siv.find_name::<StackedPileView>("waste").unwrap();
                    waste.push(c);
                })
            }
            _ => view.on_event(event),
        })
}

pub fn make_app_view(container: Container) -> impl View {
    ui(&container)
}

fn make_hands(board: Rc<RefCell<Board>>) -> impl View {
    let mut hands = LinearLayout::vertical();

    let stock_view = StackedPileView::new(board.borrow().get_stock().clone());
    hands.add_child(stock_view);

    let waste_view = StackedPileView::new(board.borrow().get_waste().clone());
    hands.add_child(waste_view.with_name("waste"));

    hands
}

fn make_tableau(board: Rc<RefCell<Board>>) -> impl View {
    let mut tableau = LinearLayout::horizontal();

    for pile in board.borrow().get_tableau() {
        tableau.add_child(FannedPileView::new(pile.clone()));
    }

    tableau
}

fn make_foundations(board: Rc<RefCell<Board>>) -> impl View {
    let mut foundations = LinearLayout::vertical();

    for pile in board.borrow().get_foundations() {
        foundations.add_child(StackedPileView::new(pile.clone()));
    }

    foundations
}

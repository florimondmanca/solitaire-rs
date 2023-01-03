use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

use crate::{domain::Target, infrastructure::Container};

use super::widgets::{FannedPileWidget, StackedPileWidget};

pub fn draw<B: Backend>(f: &mut Frame<B>, container: &Container) {
    let board = container.get_board();
    let selection = container.get_selection();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Length(68), Constraint::Min(0)].as_ref())
        .split(f.size());

    let screen = chunks[0];

    f.render_widget(
        Block::default().title("Solitaire").borders(Borders::ALL),
        screen,
    );

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Length(5 + 5),
                Constraint::Length((6 * board.borrow().get_tableau().len()) as u16 + 5),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(screen);

    // Draw hands: stock pile and waste
    let hands = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Length(5)].as_ref())
        .split(body[0]);

    let widget = StackedPileWidget::new(
        board.borrow().get_stock().clone(),
        selection.borrow().get_card_appearance(Target::Stock),
    );
    f.render_widget(widget, hands[0]);

    let widget = StackedPileWidget::new(board.borrow().get_waste().clone(), None);
    f.render_widget(widget, hands[1]);

    // Draw tableau piles.
    let tableau_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            board
                .borrow()
                .get_tableau()
                .iter()
                .map(|_| Constraint::Length(6))
                .collect::<Vec<_>>()
                .as_ref(),
        )
        .split(body[1]);

    for (index, (pile, area)) in board
        .borrow()
        .get_tableau()
        .iter()
        .zip(tableau_areas)
        .enumerate()
    {
        let widget =
            FannedPileWidget::new(pile.clone(), selection.borrow().get_range_appearance(index));
        f.render_widget(widget, area);
    }

    // Draw foundations.
    let foundation_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            board
                .borrow()
                .get_foundations()
                .iter()
                .map(|_| Constraint::Length(4))
                .collect::<Vec<_>>()
                .as_ref(),
        )
        .split(body[2]);

    for (pile, area) in board
        .borrow()
        .get_foundations()
        .iter()
        .zip(foundation_areas)
    {
        let widget = StackedPileWidget::new(pile.clone(), None);
        f.render_widget(widget, area);
    }
}

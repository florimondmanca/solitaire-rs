use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

use crate::infrastructure::Container;

use super::widgets::make_app_widget;

pub fn draw<B: Backend>(f: &mut Frame<B>, container: &Container) {
    let board_ref = container.get_board();
    let state_machine_ref = container.get_state_machine();

    let board = board_ref.borrow();
    let state_machine = state_machine_ref.borrow();

    let app = make_app_widget(&board, &state_machine);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(app.get_width()), Constraint::Min(0)].as_ref())
        .split(f.size());

    f.render_widget(
        Block::default().title("Solitaire").borders(Borders::ALL),
        chunks[0],
    );

    f.render_widget(app, chunks[0]);
}

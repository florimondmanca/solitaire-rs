use std::{error::Error, io};

use termion::{
    cursor::HideCursor, event::Key, input::TermRead, raw::IntoRawMode, screen::IntoAlternateScreen,
};
use tui::{
    backend::{Backend, TermionBackend},
    Terminal,
};

use crate::infrastructure::Container;

use super::ui;

pub fn run(container: &Container) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout()
        .lock()
        .into_raw_mode()?
        .into_alternate_screen()?;
    let stdout = HideCursor::from(stdout);
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();

    run_app(terminal, container)
}

fn run_app<B: Backend>(
    mut terminal: Terminal<B>,
    container: &Container,
) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();

    let mut dirty = true;
    let mut keys = stdin.keys();
    let board = container.get_board();
    let selection = container.get_selection();

    loop {
        if dirty {
            terminal.draw(|f| ui::draw(f, container))?;
        }

        if let Some(key) = keys.next() {
            match key? {
                Key::Char('q') => break,
                Key::Char(' ') => {
                    if selection
                        .borrow_mut()
                        .maybe_act_on_current_target(&mut board.borrow_mut())
                    {
                        dirty = true;
                    }
                }
                Key::Char('\n') => {
                    if selection
                        .borrow_mut()
                        .maybe_move_current_target_to_a_foundation(&mut board.borrow_mut())
                    {
                        dirty = true;
                    }
                }
                Key::Char('w') => {
                    if selection
                        .borrow_mut()
                        .maybe_move_top_stock_card_to_waste(&mut board.borrow_mut())
                    {
                        dirty = true;
                    }
                }
                Key::Left => {
                    selection
                        .borrow_mut()
                        .focus_previous_target(&mut board.borrow_mut());
                    dirty = true;
                }
                Key::Right => {
                    selection
                        .borrow_mut()
                        .focus_next_target(&mut board.borrow_mut());
                    dirty = true;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

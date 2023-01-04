use std::{error::Error, io};

use termion::{
    cursor::HideCursor, event::Key, input::TermRead, raw::IntoRawMode, screen::IntoAlternateScreen,
};
use tui::{
    backend::{Backend, TermionBackend},
    Terminal,
};

use crate::{domain::Action, infrastructure::Container};

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
    let state_machine = container.get_state_machine();

    loop {
        if dirty {
            terminal.draw(|f| ui::draw(f, container))?;
            dirty = false;
        }

        if let Some(key) = keys.next() {
            match key? {
                Key::Char('q') => break,
                Key::Char(' ') => {
                    if state_machine
                        .borrow_mut()
                        .handle(Action::Act(&mut board.borrow_mut()))
                    {
                        dirty = true;
                    }
                }
                Key::Char('\n') => {
                    if state_machine
                        .borrow_mut()
                        .handle(Action::Build(&mut board.borrow_mut()))
                    {
                        dirty = true;
                    }
                }
                Key::Char('w') => {
                    if state_machine
                        .borrow_mut()
                        .handle(Action::Discard(&mut board.borrow_mut()))
                    {
                        dirty = true;
                    }
                }
                Key::Left => {
                    state_machine
                        .borrow_mut()
                        .handle(Action::TargetPrevious(&mut board.borrow_mut()));
                    dirty = true;
                }
                Key::Right => {
                    state_machine
                        .borrow_mut()
                        .handle(Action::TargetNext(&mut board.borrow_mut()));
                    dirty = true;
                }
                Key::Up => {
                    if state_machine
                        .borrow_mut()
                        .handle(Action::IncreaseRange(&mut board.borrow_mut()))
                    {
                        dirty = true;
                    }
                }
                Key::Down => {
                    if state_machine.borrow_mut().handle(Action::DecreaseRange) {
                        dirty = true;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

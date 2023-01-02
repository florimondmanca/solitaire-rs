use std::{error::Error, io};

use termion::{
    cursor::HideCursor, event::Key, input::TermRead, raw::IntoRawMode, screen::IntoAlternateScreen,
};
use tui::{
    backend::{Backend, TermionBackend},
    Terminal,
};

use super::{app::App, ui};

pub fn run() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout()
        .lock()
        .into_raw_mode()?
        .into_alternate_screen()?;
    let stdout = HideCursor::from(stdout);
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();

    let app = App::new();
    run_app(terminal, app)
}

fn run_app<B: Backend>(mut terminal: Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();

    let mut keys = stdin.keys();

    loop {
        if app.is_dirty() {
            terminal.draw(|f| ui::draw(f, &mut app))?;
        }

        if let Some(key) = keys.next() {
            match key? {
                Key::Char(c) => app.on_keypress(c),
                Key::Left => app.on_left(),
                Key::Right => app.on_right(),
                _ => {}
            }
        }

        if !app.is_running() {
            return Ok(());
        }
    }
}

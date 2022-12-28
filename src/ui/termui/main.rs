use std::{
    error::Error,
    io::{stdin, stdout},
};
use termion::{
    clear, cursor,
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
    screen::IntoAlternateScreen,
};

use crate::domain::entities::Board;

use super::views::render_board;

fn render<W: std::io::Write>(
    screen: &mut W,
    board: &Board,
) -> Result<(), Box<dyn std::error::Error>> {
    write!(screen, "{}", clear::All)?;
    render_board(screen, &board)?;
    write!(screen, "\n\r")?;
    write!(screen, "Hint: 'q' will exit")?;
    write!(screen, "\n\r")?;
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut board = Board::new();

    let screen = stdout().into_alternate_screen()?.into_raw_mode()?;
    let mut screen = cursor::HideCursor::from(screen);

    render(&mut screen, &board)?;

    for event in stdin().events().filter(|r| r.is_ok()).map(|r| r.unwrap()) {
        match event {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('f')) => {
                board.tableau[0][0].flip();
                render(&mut screen, &board)?;
            }
            _ => {}
        }
    }

    Ok(())
}

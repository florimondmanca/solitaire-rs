use std::{error::Error, io};
use termion::{
    clear, cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    style,
};

use crate::domain::entities::Board;

use super::{
    lib::{HasSize, Loc, Widget},
    widgets::{FannedPileWidget, StackedPileWidget},
};

/**
 * The game state.
 */
struct Game<W: io::Write> {
    stdout: cursor::HideCursor<RawTerminal<W>>,
    board: Board,
    dirty: bool,
}

impl<W: io::Write> Game<W> {
    fn new(stdout: W) -> Self {
        let stdout = cursor::HideCursor::from(stdout.into_raw_mode().unwrap());

        Self {
            stdout,
            board: Board::new(),
            dirty: false,
        }
    }

    /**
     * Run the game loop.
     *
     * This will listen and react to events.
     */
    fn run<R: io::Read>(&mut self, stdin: R) -> Result<(), Box<dyn Error>> {
        self.init()?;

        let mut keys = stdin.keys();

        loop {
            match keys.next() {
                Some(Ok(Key::Char('q'))) => break,
                _ => {}
            }

            if self.dirty {
                self.draw()?;
            }
        }

        Ok(())
    }

    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        write!(self.stdout, "{}", clear::All)?;
        write!(self.stdout, "{}", cursor::Goto(1, 1))?;
        self.draw()?;
        self.stdout.flush()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        let gap = Loc::new(2, 2);
        let mut loc = Loc::new(1, 1);

        // Draw stock pile
        let widget = StackedPileWidget::new(self.board.stock.clone());
        widget.render(&mut self.stdout, loc)?;

        // Draw waste pile
        loc.x += widget.get_width() + gap.x;

        let widget = StackedPileWidget::new(self.board.waste.clone());
        widget.render(&mut self.stdout, loc)?;

        // Draw foundation piles
        loc.x += 2 * (widget.get_width() + gap.x);

        for pile in &self.board.foundations {
            let widget = StackedPileWidget::new(pile.clone());
            widget.render(&mut self.stdout, loc)?;
            loc.x += widget.get_width() + gap.x;
        }

        // Draw tableau piles
        loc.x = 1;
        loc.y = widget.get_height() + gap.y;

        for pile in &self.board.tableau {
            let widget = FannedPileWidget::new(pile.clone());
            widget.render(&mut self.stdout, loc)?;
            loc.x += widget.get_width() + 2;
        }

        write!(self.stdout, "\n\r")?;
        write!(self.stdout, "Hint: 'q' will exit")?;
        write!(self.stdout, "\n\r")?;

        self.stdout.flush()?;

        Ok(())
    }
}

impl<W: io::Write> Drop for Game<W> {
    fn drop(&mut self) {
        // When done, restore the defaults to avoid messing with the terminal.
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(1, 1)
        )
        .unwrap();
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut game = Game::new(stdout.lock());
    game.run(stdin.lock())?;
    Ok(())
}

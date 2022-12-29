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
    widgets::{CardState, FannedPileWidget, StackedPileWidget},
};

/**
 * The game state.
 */
struct Game<W: io::Write> {
    stdout: cursor::HideCursor<RawTerminal<W>>,
    board: Board,
    dirty: bool,
    pos: usize,
    picked: Option<usize>,
}

impl<W: io::Write> Game<W> {
    fn new(stdout: W) -> Self {
        let stdout = cursor::HideCursor::from(stdout.into_raw_mode().unwrap());

        Self {
            stdout,
            board: Board::new(),
            dirty: false,
            pos: 0,
            picked: None,
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
                Some(Ok(Key::Char(' '))) => self.on_press_space(),
                Some(Ok(Key::Char('\n'))) => self.on_press_enter(),
                Some(Ok(Key::Left)) => self.on_press_left(),
                Some(Ok(Key::Right)) => self.on_press_right(),
                _ => {}
            }

            if self.dirty {
                self.draw()?;
                self.dirty = false;
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

    fn on_press_space(&mut self) {
        if let Some(source) = self.picked {
            // We previously picked a card, and now
            // we've selected the spot where it should be moved.
            self.transfer_to(self.pos, source);
            self.picked = None;
            self.dirty = true;
            return;
        }

        // We selected one of the cards in the tableau.
        // If it's visible, pick it, otherwise, flip it over.
        if let Some(card) = self.board.tableau[self.pos].last_mut() {
            if card.is_visible() {
                self.picked = Some(self.pos);
            } else {
                card.show();
            }
            self.dirty = true;
        };
    }

    fn on_press_enter(&mut self) {
        if let Some(_) = self.picked {
            return;
        }

        // Move card to a foundation, if possible.
        let card = self.board.tableau[self.pos]
            .last()
            .filter(|c| c.is_visible());

        if card.is_none() {
            return;
        }

        let card = card.unwrap();
        let suit = card.suit;
        let rank = card.rank.0;

        for foundation in self.board.foundations.iter_mut() {
            if foundation.is_empty() {
                if rank == 1 {
                    let card = self.board.tableau[self.pos].pop().unwrap();
                    foundation.push(card);
                    self.dirty = true;
                    return;
                }
                continue;
            }

            let top = foundation.last().unwrap();

            if top.suit == suit && top.rank.0 == rank + 1 {
                let card = self.board.tableau[self.pos].pop().unwrap();
                foundation.push(card);
                self.dirty = true;
                return;
            }
        }
    }

    fn on_press_left(&mut self) {
        let num_piles = self.board.tableau.len();
        self.pos = (self.pos + num_piles - 1) % num_piles;
        self.dirty = true;
    }

    fn on_press_right(&mut self) {
        let num_piles = self.board.tableau.len();
        self.pos = (self.pos + 1) % num_piles;
        self.dirty = true;
    }

    fn transfer_to(&mut self, dest: usize, source: usize) {
        let t = &mut self.board.tableau;

        // Card of rank N can be transferred to either an empty pile...
        if t[dest].is_empty() {
            let source_card = t[source].pop().unwrap();
            t[dest].push(source_card);
            return;
        }

        // ... or a pile whose top card is hidden...
        if !t[dest].last().unwrap().is_visible() {
            let source_card = t[source].pop().unwrap();
            t[dest].push(source_card);
            return;
        }

        // ... or a pile whose top card has rank N + 1.
        let source_rank = t[source].last().unwrap().rank.0;
        let dest_rank = t[dest].last().unwrap().rank.0;
        if source_rank + 1 == dest_rank {
            let source_card = t[source].pop().unwrap();
            t[dest].push(source_card);
        }
    }

    fn get_state(&self, index: usize) -> Option<CardState> {
        match (self.pos, self.picked) {
            (pos, _) if pos == index => Some(CardState::Hovered),
            (_, Some(picked)) if picked == index => Some(CardState::Picked),
            _ => None,
        }
    }

    fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        write!(self.stdout, "{}", clear::All)?;

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

        for pile in self.board.foundations.iter() {
            let widget = StackedPileWidget::new(pile.clone());
            widget.render(&mut self.stdout, loc)?;
            loc.x += widget.get_width() + gap.x;
        }

        // Draw tableau piles
        loc.x = 1;
        loc.y = widget.get_height() + gap.y;

        for (index, pile) in self.board.tableau.iter().enumerate() {
            let widget = FannedPileWidget::new(pile.clone(), self.get_state(index));
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

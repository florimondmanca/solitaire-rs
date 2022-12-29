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
            // Stock pile is position 0, 1st tableau pile is position 1, and so on.
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
                Some(Ok(Key::Char('w'))) => self.on_press_w(),
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
        self.draw()?;
        Ok(())
    }

    fn on_press_space(&mut self) {
        if let Some(source) = self.picked {
            // User had previously picked a card.
            // They just chosed where it should be transferred.
            self.transfer_if_allowed(source, self.pos);
            self.picked = None;
            self.dirty = true;
            return;
        }

        // User has selected a card.
        // If it's visible, pick it, otherwise, reveal it.
        if let Some(card) = self.board.get_mut_pile_at(self.pos).unwrap().last_mut() {
            if card.is_visible() {
                self.picked = Some(self.pos);
            } else {
                card.reveal();
            }
            self.dirty = true;
        };
    }

    fn on_press_enter(&mut self) {
        if let Some(_) = self.picked {
            // Can't move cards to foundations while a card is picked.
            return;
        }

        // Try moving the currently hovered card to a foundation.

        let card = self
            .board
            .get_pile_at(self.pos)
            .unwrap()
            .last()
            .map(|c| *c)
            .filter(|c| c.is_visible());

        if card.is_none() {
            return;
        }

        let card = card.unwrap();
        let suit = card.suit;
        let rank = card.rank.0;
        let mut was_transferred = false;

        for foundation in self.board.foundations.iter_mut() {
            // Find a foundation where the card be transferred, if any.
            // We do this automatically for better UX.

            // Empty foundations can only be transferred an ace.
            if foundation.is_empty() {
                if rank == 1 {
                    foundation.push(card);
                    was_transferred = true;
                    self.dirty = true;
                    break;
                }
                continue;
            }

            // For established foundations, the suit must match and
            // cards must be stacked with ranks ascending.
            let last = foundation.last().unwrap();

            if last.suit == suit && rank == last.rank.0 + 1 {
                foundation.push(card);
                was_transferred = true;
                self.dirty = true;
                break;
            }
        }

        if was_transferred {
            // Need to do this outside the for-loop to please the borrow checker.
            self.board.get_mut_pile_at(self.pos).unwrap().pop().unwrap();
        }
    }

    fn on_press_w(&mut self) {
        if self.pos > 0 {
            // Can only trash a card from the stock.
            return;
        }

        if self.board.maybe_move_to_waste() {
            self.dirty = true;
        }
    }

    fn on_press_left(&mut self) {
        let max = self.board.get_num_piles();
        self.pos = (self.pos + max - 1) % max;
        self.dirty = true;
    }

    fn on_press_right(&mut self) {
        let max = self.board.get_num_piles();
        self.pos = (self.pos + 1) % max;
        self.dirty = true;
    }

    fn transfer_if_allowed(&mut self, source: usize, dest: usize) {
        if dest == 0 {
            // Can't transfer to the stock pile.
            return;
        }

        let source_pile = self.board.get_pile_at(source).unwrap();
        let dest_pile = self.board.get_pile_at(dest).unwrap();

        // Card of rank N can be transferred to an empty pile,
        // or a pile whose top card is hidden...
        if dest_pile.last().map_or(true, |c| !c.is_visible()) {
            self.board.transfer(source, dest);
            return;
        }

        // ... or a pile whose top card has rank N + 1.
        let source_rank = source_pile.last().unwrap().rank.0;
        let dest_rank = dest_pile.last().unwrap().rank.0;
        if dest_rank == source_rank + 1 {
            self.board.transfer(source, dest);
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
        let widget = StackedPileWidget::new(self.board.stock.clone(), self.get_state(0));
        widget.render(&mut self.stdout, loc)?;

        // Draw waste pile
        loc.x += widget.get_width() + gap.x;

        let widget = StackedPileWidget::new(self.board.waste.clone(), None);
        widget.render(&mut self.stdout, loc)?;

        // Draw foundation piles
        loc.x += 2 * (widget.get_width() + gap.x);

        for pile in self.board.foundations.iter() {
            let widget = StackedPileWidget::new(pile.clone(), None);
            widget.render(&mut self.stdout, loc)?;
            loc.x += widget.get_width() + gap.x;
        }

        // Draw tableau piles
        loc.x = 1;
        loc.y = widget.get_height() + gap.y;

        for (index, pile) in self.board.tableau.iter().enumerate() {
            let widget = FannedPileWidget::new(pile.clone(), self.get_state(index + 1));
            widget.render(&mut self.stdout, loc)?;
            loc.x += widget.get_width() + 2;
        }

        write!(self.stdout, "\n\r")?;
        write!(self.stdout, "Hint: 'q' will exit\n\r")?;

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

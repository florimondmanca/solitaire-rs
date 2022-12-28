use termion::cursor;

use crate::domain::entities::Pile;

use super::card::render_card;

pub static PILE_WIDTH: u16 = 5;

/**
 * Display a pile of cards fanned out as a column.
 */
pub fn render_fanned_pile<W: std::io::Write>(
    screen: &mut W,
    pile: &Pile,
    x: u16,
    y: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut dy = 0;
    let last_index = pile.len() - 1;

    for (index, card) in pile.iter().enumerate() {
        render_card(screen, card, x, y + dy)?;

        dy += match index {
            // Last card is visible in full.
            i if i == last_index => 5,
            // Other cards are covered by the card after them.
            _ => 2,
        };
    }

    Ok(())
}

/**
 * Display a pile of cards by only showing the topmost card, or an empty slot.
 */
pub fn render_stacked_pile<W: std::io::Write>(
    screen: &mut W,
    pile: &Pile,
    x: u16,
    y: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(top_card) = pile.last() {
        render_card(screen, top_card, x, y)
    } else {
        render_empty_slot(x, y, screen)
    }
}

fn render_empty_slot<W: std::io::Write>(
    x: u16,
    y: u16,
    screen: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    write!(screen, "{}", cursor::Goto(x, y))?;
    write!(screen, "┌╌╌╌┐")?;
    write!(screen, "{}", cursor::Goto(x, y + 1))?;
    write!(screen, "╎   ╎",)?;
    write!(screen, "{}", cursor::Goto(x, y + 2))?;
    write!(screen, "╎   ╎")?;
    write!(screen, "{}", cursor::Goto(x, y + 3))?;
    write!(screen, "└╌╌╌┘")?;

    Ok(())
}

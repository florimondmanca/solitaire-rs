use crate::domain::entities::Board;

use super::{
    card::CARD_HEIGHT,
    pile::{render_fanned_pile, render_stacked_pile, PILE_WIDTH},
};

static PILE_GAP: u16 = 2;

pub fn render_board<W: std::io::Write>(
    screen: &mut W,
    board: &Board,
) -> Result<(), Box<dyn std::error::Error>> {
    let stock_x = 1;
    let stock_y = 1;
    render_stacked_pile(screen, &board.stock, stock_x, stock_y)?;

    let waste_x = stock_x + PILE_WIDTH + PILE_GAP;
    let waste_y = stock_y;
    render_stacked_pile(screen, &board.waste, waste_x, waste_y)?;

    let foundation_x0 = waste_x + 2 * (PILE_WIDTH + PILE_GAP);
    let foundation_y = 1;
    let mut dx = 0;

    for pile in board.foundations.iter() {
        render_stacked_pile(screen, pile, foundation_x0 + dx, foundation_y)?;
        dx += PILE_WIDTH + PILE_GAP;
    }

    let pile_x0 = 1;
    let pile_y = stock_y + CARD_HEIGHT + 1;
    let mut dx = 0;

    for pile in board.tableau.iter() {
        render_fanned_pile(screen, pile, pile_x0 + dx, pile_y)?;
        dx += PILE_WIDTH + PILE_GAP;
    }

    Ok(())
}

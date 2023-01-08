use crate::domain::{Board, Pile, StateMachine, Target, TargetStatus};

use super::{
    card::CardAppearance,
    layout::{HorizontalWidget, VerticalWidget},
    pile::{FannedPileWidget, StackedPileWidget},
};

pub fn make_app_widget<'a>(
    board: &'a Board,
    state_machine: &'a StateMachine,
) -> HorizontalWidget<'a> {
    HorizontalWidget::new(vec![
        make_hands_widget(board.get_stock(), board.get_waste(), &state_machine).into(),
        make_tableau_widget(board.get_tableau(), &state_machine).into(),
        make_foundations_widget(board.get_foundations()).into(),
    ])
    .gap(5)
    .horizontal_margin(3)
    .vertical_margin(1)
}

fn make_hands_widget<'a>(
    stock: &'a Pile,
    waste: &'a Pile,
    state_machine: &'a StateMachine,
) -> VerticalWidget<'a> {
    let stock_appearance = state_machine
        .get_status_of(Target::Stock)
        .map(|status| match status {
            TargetStatus::Current { .. } => CardAppearance::Focused,
            TargetStatus::Picked { .. } => CardAppearance::Picked,
        });

    VerticalWidget::new(vec![
        StackedPileWidget::new(stock, stock_appearance)
            .empty_content(['↱', '↲'])
            .into(),
        StackedPileWidget::new(waste, None).into(),
    ])
    .gap(1)
}

fn make_tableau_widget<'a>(
    piles: &'a Vec<Pile>,
    state_machine: &'a StateMachine,
) -> HorizontalWidget<'a> {
    HorizontalWidget::new(
        piles
            .into_iter()
            .enumerate()
            .map(|(index, pile)| {
                let pile_appearance =
                    state_machine
                        .get_status_of(Target::Pile(index))
                        .map(|status| match status {
                            TargetStatus::Current { num_cards } => {
                                (CardAppearance::Focused, num_cards)
                            }
                            TargetStatus::Picked { num_cards } => {
                                (CardAppearance::Picked, num_cards)
                            }
                        });

                FannedPileWidget::new(pile, pile_appearance).into()
            })
            .collect::<Vec<_>>(),
    )
    .gap(2)
}

fn make_foundations_widget(foundations: &Vec<Pile>) -> VerticalWidget {
    VerticalWidget::new(
        foundations
            .iter()
            .map(|pile| StackedPileWidget::new(pile, None).into())
            .collect::<Vec<_>>(),
    )
}

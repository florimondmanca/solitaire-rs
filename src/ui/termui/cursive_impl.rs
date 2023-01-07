use std::error::Error;

use cursive::{
    theme::{BorderStyle, Palette},
    Cursive, CursiveExt, With,
};

use crate::infrastructure::Container;

use super::views::make_app_view;

pub fn run() -> Result<(), Box<dyn Error>> {
    let container = Container::default();

    let mut siv = Cursive::new();

    siv.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::default().with(|palette| {
            use cursive::theme::BaseColor::*;
            use cursive::theme::Color::TerminalDefault;
            use cursive::theme::PaletteColor::*;

            palette[Background] = TerminalDefault;
            palette[View] = TerminalDefault;
            palette[Primary] = White.dark();
            palette[TitlePrimary] = Blue.light();
            palette[Secondary] = Blue.light();
            palette[Highlight] = Blue.dark();
        }),
    });

    let app = make_app_view(container);
    siv.add_layer(app);
    siv.run();
    Ok(())

    // let mut dirty = true;
    // let mut keys = stdin.keys();
    // let board = container.get_board();
    // let state_machine = container.get_state_machine();

    // loop {
    //     if dirty {
    //         terminal.draw(|f| ui::draw(f, container))?;
    //         dirty = false;
    //     }

    //     if let Some(key) = keys.next() {
    //         match key? {
    //             Key::Char('q') => break,
    //             Key::Char(' ') => {
    //                 if state_machine
    //                     .borrow_mut()
    //                     .handle(Action::Act(&mut board.borrow_mut()))
    //                 {
    //                     dirty = true;
    //                 }
    //             }
    //             Key::Char('\n') => {
    //                 if state_machine
    //                     .borrow_mut()
    //                     .handle(Action::Build(&mut board.borrow_mut()))
    //                 {
    //                     dirty = true;
    //                 }
    //             }
    //             Key::Char('w') => {
    //                 if state_machine
    //                     .borrow_mut()
    //                     .handle(Action::Discard(&mut board.borrow_mut()))
    //                 {
    //                     dirty = true;
    //                 }
    //             }
    //             Key::Left => {
    //                 state_machine
    //                     .borrow_mut()
    //                     .handle(Action::TargetPrevious(&mut board.borrow_mut()));
    //                 dirty = true;
    //             }
    //             Key::Right => {
    //                 state_machine
    //                     .borrow_mut()
    //                     .handle(Action::TargetNext(&mut board.borrow_mut()));
    //                 dirty = true;
    //             }
    //             Key::Up => {
    //                 if state_machine
    //                     .borrow_mut()
    //                     .handle(Action::IncreaseRange(&mut board.borrow_mut()))
    //                 {
    //                     dirty = true;
    //                 }
    //             }
    //             Key::Down => {
    //                 if state_machine.borrow_mut().handle(Action::DecreaseRange) {
    //                     dirty = true;
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    // }

    // Ok(())
}

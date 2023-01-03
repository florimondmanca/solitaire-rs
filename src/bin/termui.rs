use solitaire_rs::{infrastructure::Container, ui::termui};

fn main() {
    let container = Container::default();
    termui::run(&container).unwrap();
}

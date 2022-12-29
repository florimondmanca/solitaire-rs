use std::error::Error;
use std::io;

use super::geometry::{Loc, Size};

pub type RenderResult = Result<(), Box<dyn Error>>;

pub trait Widget<W: io::Write> {
    fn render(&self, f: &mut W, loc: Loc) -> RenderResult;
}

pub trait HasSize {
    fn get_size(&self) -> &Size;

    fn get_width(&self) -> u16 {
        self.get_size().width
    }

    fn get_height(&self) -> u16 {
        self.get_size().height
    }
}

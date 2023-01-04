use tui::widgets::Widget;

use crate::ui::termui::widgets::pile::{FannedPileWidget, StackedPileWidget};

use super::{HorizontalWidget, VerticalWidget};

/*
You might expect a `trait LayoutWidget` instead of an enum.

But then implementing `{Horizontal,Vertical}Widget` would be impossible.

Indeed, we'd need to use `widgets: Vec<Box<dyn LayoutWidget>>`. But
`Widget::render(self, area, buf)` *moves* the widget object (it's `self`, not `&self`),
and a `dyn T` can't be moved (compile error akin to "size not known").

Instead, since the amount of widgets we expect to place in horizontal or vertical
layouts is fixed, we use an enum. This comes with some boilerplate that could be
reduced by using [enum_dispatch](https://docs.rs/enum_dispatch/0.3.3/enum_dispatch/).
*/
pub enum LayoutWidget<'a> {
    StackedPile(StackedPileWidget<'a>),
    FannedPile(FannedPileWidget<'a>),
    Horizontal(HorizontalWidget<'a>),
    Vertical(VerticalWidget<'a>),
}

impl<'a> From<StackedPileWidget<'a>> for LayoutWidget<'a> {
    fn from(w: StackedPileWidget<'a>) -> Self {
        LayoutWidget::StackedPile(w)
    }
}

impl<'a> From<FannedPileWidget<'a>> for LayoutWidget<'a> {
    fn from(w: FannedPileWidget<'a>) -> Self {
        LayoutWidget::FannedPile(w)
    }
}

impl<'a> From<HorizontalWidget<'a>> for LayoutWidget<'a> {
    fn from(w: HorizontalWidget<'a>) -> Self {
        LayoutWidget::Horizontal(w)
    }
}

impl<'a> From<VerticalWidget<'a>> for LayoutWidget<'a> {
    fn from(w: VerticalWidget<'a>) -> Self {
        LayoutWidget::Vertical(w)
    }
}

impl<'a> LayoutWidget<'a> {
    pub fn get_width(&self) -> u16 {
        match self {
            LayoutWidget::StackedPile(w) => w.get_width(),
            LayoutWidget::FannedPile(w) => w.get_width(),
            LayoutWidget::Horizontal(w) => w.get_width(),
            LayoutWidget::Vertical(w) => w.get_width(),
        }
    }

    pub fn get_height(&self) -> u16 {
        match self {
            LayoutWidget::StackedPile(w) => w.get_height(),
            LayoutWidget::FannedPile(w) => w.get_height(),
            LayoutWidget::Horizontal(w) => w.get_height(),
            LayoutWidget::Vertical(w) => w.get_height(),
        }
    }
}

impl<'a> Widget for LayoutWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        match self {
            LayoutWidget::StackedPile(w) => w.render(area, buf),
            LayoutWidget::FannedPile(w) => w.render(area, buf),
            LayoutWidget::Horizontal(w) => w.render(area, buf),
            LayoutWidget::Vertical(w) => w.render(area, buf),
        }
    }
}

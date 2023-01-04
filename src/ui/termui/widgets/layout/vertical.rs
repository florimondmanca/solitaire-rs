use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use super::widget::LayoutWidget;

pub struct VerticalWidget<'a> {
    widgets: Vec<LayoutWidget<'a>>,
    gap: u16,
}

impl<'a> VerticalWidget<'a> {
    pub fn new(widgets: Vec<LayoutWidget<'a>>) -> Self {
        Self { widgets, gap: 0 }
    }

    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    fn iter_heights(&self) -> impl Iterator<Item = u16> + '_ {
        self.widgets
            .iter()
            .enumerate()
            .map(|(index, w)| (index == self.widgets.len() - 1, w))
            .map(move |(is_last, w)| w.get_height() + is_last.then_some(0).unwrap_or(self.gap))
    }

    pub fn get_width(&self) -> u16 {
        self.widgets
            .iter()
            .map(|w| w.get_width())
            .max()
            .unwrap_or(0)
    }

    pub fn get_height(&self) -> u16 {
        self.iter_heights().sum()
    }
}

impl<'a> Widget for VerticalWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.iter_heights()
                    .map(|w| Constraint::Length(w))
                    .collect::<Vec<_>>()
                    .as_ref(),
            )
            .split(area);

        for (widget, area) in self.widgets.into_iter().zip(areas) {
            widget.render(area, buf);
        }
    }
}

use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use super::widget::LayoutWidget;

pub struct HorizontalWidget<'a> {
    widgets: Vec<LayoutWidget<'a>>,
    gap: u16,
    mh: u16,
    mv: u16,
}

impl<'a> HorizontalWidget<'a> {
    pub fn new(widgets: Vec<LayoutWidget<'a>>) -> Self {
        Self {
            widgets,
            gap: 0,
            mh: 0,
            mv: 0,
        }
    }

    pub fn gap(mut self, value: u16) -> Self {
        self.gap = value;
        self
    }

    pub fn horizontal_margin(mut self, value: u16) -> Self {
        self.mh = value;
        self
    }

    pub fn vertical_margin(mut self, value: u16) -> Self {
        self.mv = value;
        self
    }

    fn iter_widths(&self) -> impl Iterator<Item = u16> + '_ {
        self.widgets
            .iter()
            .enumerate()
            .map(|(index, w)| (index == self.widgets.len() - 1, w))
            .map(move |(is_last, w)| w.get_width() + is_last.then_some(0).unwrap_or(self.gap))
    }

    pub fn get_width(&self) -> u16 {
        2 * self.mh + self.iter_widths().sum::<u16>()
    }

    pub fn get_height(&self) -> u16 {
        2 * self.mv
            + self
                .widgets
                .iter()
                .map(|w| w.get_height())
                .max()
                .unwrap_or(0)
    }
}

impl<'a> Widget for HorizontalWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(self.mh)
            .vertical_margin(self.mv)
            .constraints(
                self.iter_widths()
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

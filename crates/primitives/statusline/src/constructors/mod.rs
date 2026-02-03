use ratatui::style::Style;
use ratatui::text::Line;

use crate::{OperationalMode, StatusLineStacked, StyledStatusLine};

impl<'a> StatusLineStacked<'a> {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
            left: Vec::new(),
            center_margin: 0,
            center: Line::default(),
            right: Vec::new(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn start(mut self, text: impl Into<Line<'a>>, gap: impl Into<Line<'a>>) -> Self {
        self.left.push((text.into(), gap.into()));
        self
    }

    pub fn start_bare(mut self, text: impl Into<Line<'a>>) -> Self {
        self.left.push((text.into(), "".into()));
        self
    }

    pub fn center_margin(mut self, margin: u16) -> Self {
        self.center_margin = margin;
        self
    }

    pub fn center(mut self, text: impl Into<Line<'a>>) -> Self {
        self.center = text.into();
        self
    }

    pub fn end(mut self, text: impl Into<Line<'a>>, gap: impl Into<Line<'a>>) -> Self {
        self.right.push((text.into(), gap.into()));
        self
    }

    pub fn end_bare(mut self, text: impl Into<Line<'a>>) -> Self {
        self.right.push((text.into(), "".into()));
        self
    }
}

impl<'a> StyledStatusLine<'a> {
    pub fn new() -> Self {
        Self {
            mode: OperationalMode::Operational,
            title: " WESTINGHOUSE[STATUS]2 ",
            center_text: String::new(),
            render_count: 0,
            event_count: 0,
            render_time_us: 0,
            event_time_us: 0,
            message_count: 0,
            use_slants: true,
        }
    }

    pub fn mode(mut self, mode: OperationalMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn center_text(mut self, text: impl Into<String>) -> Self {
        self.center_text = text.into();
        self
    }

    pub fn render_metrics(mut self, count: usize, time_us: u64) -> Self {
        self.render_count = count;
        self.render_time_us = time_us;
        self
    }

    pub fn event_metrics(mut self, count: usize, time_us: u64) -> Self {
        self.event_count = count;
        self.event_time_us = time_us;
        self
    }

    pub fn message_count(mut self, count: u32) -> Self {
        self.message_count = count;
        self
    }

    pub fn use_slants(mut self, use_slants: bool) -> Self {
        self.use_slants = use_slants;
        self
    }
}

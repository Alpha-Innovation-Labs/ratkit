use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::BorderType;

use crate::pane::Pane;

impl<'a> Pane<'a> {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            padding: (0, 0, 0, 0),
            text_footer: None,
            footer_height: 0,
            border_style: Style::default().fg(Color::White),
            border_type: BorderType::Rounded,
            title_style: Style::default().add_modifier(Modifier::BOLD),
            footer_style: Style::default().fg(Color::DarkGray),
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_padding(mut self, top: u16, right: u16, bottom: u16, left: u16) -> Self {
        self.padding = (top, right, bottom, left);
        self
    }

    pub fn with_uniform_padding(mut self, padding: u16) -> Self {
        self.padding = (padding, padding, padding, padding);
        self
    }

    pub fn with_text_footer(mut self, footer: Line<'a>) -> Self {
        self.text_footer = Some(footer);
        self
    }

    pub fn with_footer_height(mut self, height: u16) -> Self {
        self.footer_height = height;
        self
    }

    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    pub fn footer_style(mut self, style: Style) -> Self {
        self.footer_style = style;
        self
    }
}

//! Default trait implementation for MinimapConfig.

use ratatui::style::{Color, Style};

use crate::markdown_widget::extensions::minimap::enums::MinimapConfig;

impl Default for MinimapConfig {
    fn default() -> Self {
        Self {
            width: 10,
            height: 20,
            text_style: Style::default().fg(Color::Rgb(88, 88, 88)),
            viewport_style: Style::default()
                .fg(Color::Rgb(97, 175, 239))
                .bg(Color::Rgb(40, 44, 52)),
            background_style: Style::default().bg(Color::Rgb(30, 30, 30)),
            show_density: true,
        }
    }
}

//! Default trait implementation for TocConfig.

use ratatui::style::{Color, Style};

use crate::widgets::markdown_widget::extensions::toc::enums::TocConfig;

impl Default for TocConfig {
    fn default() -> Self {
        Self {
            compact_width: 12,
            expanded_width: 32,
            height: 20,
            text_style: Style::default().fg(Color::Rgb(160, 160, 160)),
            active_style: Style::default().fg(Color::Rgb(97, 175, 239)), // Blue
            hover_style: Style::default().fg(Color::White).bg(Color::Rgb(60, 60, 70)),
            background_style: Style::default().bg(Color::Rgb(30, 32, 38)),
            line_style: Style::default().fg(Color::Rgb(120, 120, 130)),
            active_line_style: Style::default().fg(Color::Rgb(230, 180, 80)), // Gold/yellow for visibility
            show_border: true,
            border_style: Style::default().fg(Color::Rgb(138, 99, 210)), // Purple
            title_style: Style::default().fg(Color::Rgb(138, 99, 210)),  // Purple
            title: "TOC".to_string(),
            line_spacing: 2, // 2 dots per entry (tight spacing)
        }
    }
}

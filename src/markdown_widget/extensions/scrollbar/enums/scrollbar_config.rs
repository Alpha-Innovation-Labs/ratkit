//! Configuration for scrollbar appearance.

use ratatui::style::{Color, Style};

/// Configuration for scrollbar appearance.
#[derive(Debug, Clone)]
pub struct ScrollbarConfig {
    /// Width of the scrollbar in characters.
    pub width: u16,
    /// Character used for the track (background).
    pub track_char: char,
    /// Character used for the thumb (scrollable indicator).
    pub thumb_char: char,
    /// Style for the track.
    pub track_style: Style,
    /// Style for the thumb.
    pub thumb_style: Style,
    /// Style for the percentage text.
    pub percentage_style: Style,
    /// Minimum height for the thumb in characters.
    pub min_thumb_height: u16,
}

impl Default for ScrollbarConfig {
    fn default() -> Self {
        Self {
            width: 1,
            track_char: '░',
            thumb_char: '█',
            track_style: Style::default().fg(Color::Rgb(50, 55, 65)),
            thumb_style: Style::default().fg(Color::Rgb(120, 130, 145)),
            percentage_style: Style::default().fg(Color::Rgb(70, 75, 85)),
            min_thumb_height: 1,
        }
    }
}

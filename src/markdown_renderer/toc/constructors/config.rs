//! Configuration builder methods for TocConfig.

use ratatui::style::Style;

use super::super::TocConfig;

impl TocConfig {
    /// Create a new TocConfig with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the compact mode width.
    pub fn compact_width(mut self, width: u16) -> Self {
        self.compact_width = width;
        self
    }

    /// Set the expanded mode width.
    pub fn expanded_width(mut self, width: u16) -> Self {
        self.expanded_width = width;
        self
    }

    /// Set the height.
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Set the text style for normal headings.
    pub fn text_style(mut self, style: Style) -> Self {
        self.text_style = style;
        self
    }

    /// Set the style for the active heading.
    pub fn active_style(mut self, style: Style) -> Self {
        self.active_style = style;
        self
    }

    /// Set the style for hovered headings.
    pub fn hover_style(mut self, style: Style) -> Self {
        self.hover_style = style;
        self
    }

    /// Set the background style.
    pub fn background_style(mut self, style: Style) -> Self {
        self.background_style = style;
        self
    }

    /// Set the line spacing in compact mode (in 1/8 cell units).
    /// 1 = tightest (up to 8 lines per row), 8 = one line per row.
    pub fn line_spacing(mut self, spacing: u8) -> Self {
        self.line_spacing = spacing;
        self
    }
}

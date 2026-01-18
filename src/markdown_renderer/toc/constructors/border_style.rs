//! Builder method for configuring TOC border style.

use ratatui::style::Style;

use super::super::TocConfig;

impl TocConfig {
    /// Set the border style.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to the border lines and corners.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Style};
    /// use ratatui_toolkit::markdown_renderer::toc::TocConfig;
    ///
    /// let config = TocConfig::default()
    ///     .border_style(Style::default().fg(Color::Cyan));
    /// ```
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }
}

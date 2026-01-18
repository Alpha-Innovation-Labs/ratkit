//! Builder method for configuring TOC title style.

use ratatui::style::Style;

use super::super::TocConfig;

impl TocConfig {
    /// Set the title style.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to the title text in the border.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Modifier, Style};
    /// use ratatui_toolkit::markdown_renderer::toc::TocConfig;
    ///
    /// let config = TocConfig::default()
    ///     .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    /// ```
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }
}

//! Set the custom scrollbar configuration.

use crate::widgets::markdown_widget::extensions::scrollbar::ScrollbarConfig;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the custom scrollbar configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The scrollbar configuration to use
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui::style::{Color, Style};
    /// use ratatui_toolkit::markdown_widget::extensions::scrollbar::ScrollbarConfig;
    ///
    /// let config = ScrollbarConfig {
    ///     thumb_style: Style::default().fg(Color::Cyan),
    ///     ..Default::default()
    /// };
    ///
    /// let widget = MarkdownWidget::from_state(&content, &mut state)
    ///     .show_custom_scrollbar(true)
    ///     .scrollbar_config(config);
    /// ```
    pub fn scrollbar_config(mut self, config: ScrollbarConfig) -> Self {
        self.scrollbar_config = config;
        self
    }
}

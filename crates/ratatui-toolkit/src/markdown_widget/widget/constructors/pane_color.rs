//! Constructor for pane color.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the border color for the Pane that wraps the widget.
    ///
    /// Only used when `has_pane` is true (default).
    ///
    /// # Arguments
    ///
    /// * `color` - The color to use for the pane's border
    ///
    /// # Returns
    ///
    /// The modified `MarkdownWidget` instance.
    pub fn with_pane_color(mut self, color: impl Into<ratatui::style::Color>) -> Self {
        self.pane_color = Some(color.into());
        self
    }
}

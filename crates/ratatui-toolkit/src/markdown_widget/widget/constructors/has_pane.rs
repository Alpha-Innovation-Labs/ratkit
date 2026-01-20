//! Constructor for has_pane option.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set whether to wrap the widget in a Pane.
    ///
    /// When `has_pane` is true (default), the widget is wrapped in a styled Pane
    /// with title, border, and padding. Set to false for raw markdown rendering.
    ///
    /// # Arguments
    ///
    /// * `has_pane` - Whether to wrap in a Pane (default: true)
    ///
    /// # Returns
    ///
    /// The modified `MarkdownWidget` instance.
    pub fn with_has_pane(mut self, has_pane: bool) -> Self {
        self.has_pane = has_pane;
        self
    }
}

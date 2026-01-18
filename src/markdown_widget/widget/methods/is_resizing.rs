//! Set resizing state method for MarkdownWidget.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set whether the widget is currently being resized (for smoother drag performance).
    ///
    /// # Arguments
    ///
    /// * `resizing` - Whether the widget is being resized
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn is_resizing(mut self, resizing: bool) -> Self {
        self.is_resizing = resizing;
        self
    }
}

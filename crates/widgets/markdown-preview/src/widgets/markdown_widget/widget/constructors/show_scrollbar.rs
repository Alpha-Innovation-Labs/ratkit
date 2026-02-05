//! Enable or disable the scrollbar.

use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Enable or disable the scrollbar.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the scrollbar
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }
}

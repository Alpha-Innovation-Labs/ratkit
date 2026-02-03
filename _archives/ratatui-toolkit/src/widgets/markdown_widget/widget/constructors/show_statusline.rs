//! Enable or disable the statusline.

use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set whether to show the statusline.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the statusline
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_statusline(mut self, show: bool) -> Self {
        self.show_statusline = show;
        self
    }
}

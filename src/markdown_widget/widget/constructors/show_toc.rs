//! Enable or disable the TOC (Table of Contents).

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Enable or disable the TOC (Table of Contents).
    ///
    /// When enabled, shows heading navigation in the top-right corner.
    /// Compact mode shows lines, expanded mode (on hover) shows text.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the TOC
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_toc(mut self, show: bool) -> Self {
        self.show_toc = show;
        self
    }
}

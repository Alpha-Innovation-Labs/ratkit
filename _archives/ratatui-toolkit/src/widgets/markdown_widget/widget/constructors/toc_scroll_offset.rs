//! Set the TOC scroll offset.

use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the TOC scroll offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The scroll offset for the TOC list
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_scroll_offset(mut self, offset: usize) -> Self {
        self.toc_scroll_offset = offset;
        self
    }
}

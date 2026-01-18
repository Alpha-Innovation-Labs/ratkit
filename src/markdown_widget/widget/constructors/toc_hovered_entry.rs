//! Set the hovered TOC entry index.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the hovered TOC entry index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the hovered entry, or None
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_hovered_entry(mut self, index: Option<usize>) -> Self {
        self.toc_hovered_entry = index;
        self
    }
}

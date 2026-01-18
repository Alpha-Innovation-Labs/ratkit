//! Set the TOC hovered state.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the TOC hovered state.
    ///
    /// When hovered, the TOC expands to show heading text.
    ///
    /// # Arguments
    ///
    /// * `hovered` - Whether the TOC is hovered
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_hovered(mut self, hovered: bool) -> Self {
        self.toc_hovered = hovered;
        self
    }
}

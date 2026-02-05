//! Hovered entry configuration for Toc widget.

use crate::widgets::markdown_widget::extensions::toc::Toc;

impl<'a> Toc<'a> {
    /// Set the hovered item index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the hovered heading, or None.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn hovered(self, _index: Option<usize>) -> Self {
        // Now managed by TocState, this is a no-op for compatibility
        self
    }
}

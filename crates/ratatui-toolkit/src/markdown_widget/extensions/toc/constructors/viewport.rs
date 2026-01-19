//! Viewport configuration for Toc widget.

use crate::markdown_widget::extensions::toc::Toc;

impl<'a> Toc<'a> {
    /// Set the current viewport information.
    ///
    /// # Arguments
    ///
    /// * `scroll_offset` - Current scroll offset.
    /// * `viewport_height` - Height of the visible viewport.
    /// * `total_lines` - Total number of lines in the document.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn viewport(
        self,
        _scroll_offset: usize,
        _viewport_height: usize,
        _total_lines: usize,
    ) -> Self {
        // These are now managed by TocState, this is a no-op for compatibility
        self
    }

    /// Set the TOC scroll offset (for scrolling within the TOC list).
    ///
    /// # Arguments
    ///
    /// * `offset` - The scroll offset for the TOC list.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_scroll(self, _offset: usize) -> Self {
        // Now managed by TocState, this is a no-op for compatibility
        self
    }
}

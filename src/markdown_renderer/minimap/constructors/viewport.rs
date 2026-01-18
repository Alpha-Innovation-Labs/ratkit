//! Viewport builder method for Minimap.

use super::super::Minimap;

impl<'a> Minimap<'a> {
    /// Set the current viewport position.
    ///
    /// # Arguments
    ///
    /// * `start` - First visible line (0-indexed)
    /// * `end` - Last visible line (0-indexed)
    /// * `total` - Total number of lines in the document
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn viewport(mut self, start: usize, end: usize, total: usize) -> Self {
        self.viewport_start = start;
        self.viewport_end = end;
        self.total_lines = total;
        self
    }
}

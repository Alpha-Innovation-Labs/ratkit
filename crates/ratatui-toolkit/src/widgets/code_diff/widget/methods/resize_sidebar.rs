//! Method to resize the sidebar.

use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Adjusts the sidebar width by a delta percentage.
    ///
    /// Positive delta increases width, negative decreases.
    /// The width is clamped to the ResizableSplit's min/max values.
    ///
    /// # Arguments
    ///
    /// * `delta` - The change in width percentage (positive = wider, negative = narrower)
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{CodeDiff, DiffConfig};
    ///
    /// let mut diff = CodeDiff::new()
    ///     .with_config(DiffConfig::new().sidebar_enabled(true));
    ///
    /// diff.resize_sidebar(5);  // Make 5% wider
    /// diff.resize_sidebar(-3); // Make 3% narrower
    /// ```
    pub fn resize_sidebar(&mut self, delta: i16) {
        let current = self.sidebar_split.split_percent as i16;
        let new_width = (current + delta)
            .max(self.sidebar_split.min_percent as i16)
            .min(self.sidebar_split.max_percent as i16) as u16;
        self.sidebar_split.split_percent = new_width;
    }
}

//! Builder method for configuring TOC title text.

use super::super::TocConfig;

impl TocConfig {
    /// Set the title text.
    ///
    /// # Arguments
    ///
    /// * `title` - The text to display in the border header.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::markdown_renderer::toc::TocConfig;
    ///
    /// let config = TocConfig::default().title("Table of Contents");
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}

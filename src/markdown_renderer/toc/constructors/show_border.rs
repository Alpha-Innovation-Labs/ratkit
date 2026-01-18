//! Builder method for configuring TOC border visibility.

use super::super::TocConfig;

impl TocConfig {
    /// Set whether to show a border around the TOC.
    ///
    /// # Arguments
    ///
    /// * `show` - If true, renders a rounded border in expanded mode.
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
    /// let config = TocConfig::default().show_border(false);
    /// ```
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }
}

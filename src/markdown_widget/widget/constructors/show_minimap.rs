//! Enable or disable the minimap.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Enable or disable the minimap.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the minimap
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_minimap(mut self, show: bool) -> Self {
        self.show_minimap = show;
        self
    }
}

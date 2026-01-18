//! Set the minimap height.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the minimap height.
    ///
    /// # Arguments
    ///
    /// * `height` - Height in terminal rows
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn minimap_height(mut self, height: u16) -> Self {
        self.minimap_config.height = height;
        self
    }
}

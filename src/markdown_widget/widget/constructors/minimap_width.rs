//! Set the minimap width.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the minimap width.
    ///
    /// # Arguments
    ///
    /// * `width` - Width in terminal columns
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn minimap_width(mut self, width: u16) -> Self {
        self.minimap_config.width = width;
        self
    }
}

//! Set the minimap hovered state.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the minimap hovered state.
    ///
    /// When hovered, the minimap scales up.
    ///
    /// # Arguments
    ///
    /// * `hovered` - Whether the minimap is hovered
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn minimap_hovered(mut self, hovered: bool) -> Self {
        self.minimap_hovered = hovered;
        self
    }
}

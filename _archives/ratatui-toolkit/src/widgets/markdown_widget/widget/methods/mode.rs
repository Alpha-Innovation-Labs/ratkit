//! Set mode method for MarkdownWidget.

use crate::widgets::markdown_widget::widget::enums::MarkdownWidgetMode;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the current mode for the statusline.
    ///
    /// # Arguments
    ///
    /// * `mode` - The mode to display (Normal or Drag)
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn mode(mut self, mode: MarkdownWidgetMode) -> Self {
        self.mode = mode;
        self
    }
}

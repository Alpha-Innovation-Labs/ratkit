//! Set whether selection mode is active.

use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set whether selection mode is active.
    ///
    /// This affects the mode displayed in the statusline (Normal vs Drag).
    ///
    /// # Arguments
    ///
    /// * `active` - Whether selection is active
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn selection_active(mut self, active: bool) -> Self {
        self.selection_active = active;
        self
    }
}

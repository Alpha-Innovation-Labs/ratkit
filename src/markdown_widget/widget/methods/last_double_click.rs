//! Methods for retrieving the last double-click info.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Get the last double-click info and clear it.
    ///
    /// Call this after processing events to check if a double-click occurred.
    ///
    /// # Returns
    ///
    /// `Some((line_number, line_kind, content))` if a double-click occurred, `None` otherwise.
    pub fn take_last_double_click(&mut self) -> Option<(usize, String, String)> {
        self.last_double_click.take()
    }
}

//! Methods for retrieving the last double-click info and copied text.

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

    /// Get the last copied text and clear it.
    ///
    /// Call this after processing events to check if text was copied to clipboard.
    /// Use this to show a toast notification when text is copied.
    ///
    /// # Returns
    ///
    /// `Some(text)` if text was copied, `None` otherwise.
    pub fn take_last_copied(&mut self) -> Option<String> {
        self.selection.last_copied_text.take()
    }
}

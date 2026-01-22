//! Method to check if sidebar divider is being dragged.

use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Returns whether the sidebar divider is currently being dragged.
    ///
    /// This can be used to adjust polling rate for smooth dragging.
    pub fn is_sidebar_dragging(&self) -> bool {
        self.sidebar_split.is_dragging()
    }
}

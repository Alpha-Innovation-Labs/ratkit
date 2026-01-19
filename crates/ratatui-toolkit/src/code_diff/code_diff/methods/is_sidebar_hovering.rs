//! Method to check if mouse is hovering over sidebar divider.

use crate::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Returns whether the mouse is hovering over the sidebar divider.
    ///
    /// This can be used to change the cursor style.
    pub fn is_sidebar_hovering(&self) -> bool {
        self.sidebar_split.is_hovering
    }
}

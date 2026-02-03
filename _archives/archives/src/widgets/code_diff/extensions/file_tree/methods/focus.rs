//! Focus-related methods for DiffFileTree.

use crate::widgets::code_diff::diff_file_tree::DiffFileTree;

impl DiffFileTree {
    /// Sets focus on this widget.
    pub fn focus(&mut self) {
        self.focused = true;
    }

    /// Removes focus from this widget.
    pub fn blur(&mut self) {
        self.focused = false;
    }
}

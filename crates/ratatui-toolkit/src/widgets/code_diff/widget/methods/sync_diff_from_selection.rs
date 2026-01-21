//! Method to sync diff display from current tree selection.

use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Syncs the diff display from the current tree selection.
    pub(crate) fn sync_diff_from_selection(&mut self) {
        if let Some(path) = self.file_tree.selected_path() {
            if let Some(hunks) = self.file_diffs.get(&path) {
                self.file_path = Some(path);
                self.hunks = hunks.clone();
                self.scroll_offset = 0;
            }
        }
    }
}

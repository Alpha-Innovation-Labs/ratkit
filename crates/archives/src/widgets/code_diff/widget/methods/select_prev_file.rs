//! Method to select the previous file in the sidebar.

use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Selects the previous file in the sidebar file tree.
    ///
    /// This also updates the current diff hunks to show the selected file.
    /// If already at the first file, stays at the first file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{CodeDiff, DiffConfig};
    /// use ratatui_toolkit::widgets::code_diff::diff_file_tree::FileStatus;
    ///
    /// let mut diff = CodeDiff::new()
    ///     .with_config(DiffConfig::new().sidebar_enabled(true))
    ///     .with_file("file1.rs", FileStatus::Modified, "")
    ///     .with_file("file2.rs", FileStatus::Added, "");
    ///
    /// diff.select_next_file();
    /// diff.select_prev_file(); // Back to first file
    /// ```
    pub fn select_prev_file(&mut self) {
        self.file_tree.select_prev();
        self.sync_diff_from_tree();
    }

    /// Syncs the current diff hunks from the selected file in the tree.
    fn sync_diff_from_tree(&mut self) {
        if let Some(path) = self.file_tree.selected_path() {
            if let Some(hunks) = self.file_diffs.get(&path) {
                self.file_path = Some(path);
                self.hunks = hunks.clone();
                self.scroll_offset = 0; // Reset scroll when changing files
            }
        }
    }
}

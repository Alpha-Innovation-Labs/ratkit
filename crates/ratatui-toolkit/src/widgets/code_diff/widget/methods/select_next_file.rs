//! Method to select the next file in the sidebar.

use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Selects the next file in the sidebar file tree.
    ///
    /// This also updates the current diff hunks to show the selected file.
    /// If already at the last file, stays at the last file.
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
    /// ```
    pub fn select_next_file(&mut self) {
        self.file_tree.select_next();
        self.update_current_diff();
    }

    /// Updates the current diff hunks based on the selected file in the tree.
    fn update_current_diff(&mut self) {
        if let Some(path) = self.file_tree.selected_path() {
            if let Some(hunks) = self.file_diffs.get(&path) {
                self.file_path = Some(path);
                self.hunks = hunks.clone();
                self.scroll_offset = 0; // Reset scroll when changing files
            }
        }
    }
}

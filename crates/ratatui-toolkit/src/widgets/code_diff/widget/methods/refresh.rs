//! Refresh diff from git.

use crate::widgets::code_diff::helpers::get_git_diff;

use super::super::CodeDiff;

impl CodeDiff {
    /// Refresh the diff from the current git repository.
    ///
    /// This re-fetches the git diff and updates the widget state while
    /// preserving the current configuration and sidebar visibility.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// let mut diff = CodeDiff::from_git();
    ///
    /// // User makes changes to files...
    ///
    /// // Refresh to pick up new changes
    /// diff.refresh();
    /// ```
    pub fn refresh(&mut self) {
        let diff = get_git_diff();
        let new_diff = if diff.trim().is_empty() {
            CodeDiff::new()
        } else {
            CodeDiff::from_multi_file_diff(&diff)
        };

        // Preserve config and some state
        let config = self.config.clone();
        let show_sidebar = self.show_sidebar;

        *self = new_diff;
        self.config = config;
        self.show_sidebar = show_sidebar;
    }
}

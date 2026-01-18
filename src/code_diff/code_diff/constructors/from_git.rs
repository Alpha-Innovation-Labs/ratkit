//! Create CodeDiff from current git repository diff.

use crate::code_diff::helpers::get_git_diff;
use crate::code_diff::DiffConfig;

use super::super::CodeDiff;

impl CodeDiff {
    /// Create a CodeDiff from the current git repository's diff.
    ///
    /// Tries in order:
    /// 1. Unstaged changes (`git diff`)
    /// 2. Staged changes (`git diff --cached`)
    /// 3. Last commit (`git diff HEAD~1`)
    ///
    /// Returns an empty CodeDiff if not in a git repo or no changes found.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// // Create a diff widget from current git changes
    /// let diff = CodeDiff::from_git();
    /// ```
    pub fn from_git() -> Self {
        let diff = get_git_diff();
        if diff.trim().is_empty() {
            Self::new()
        } else {
            Self::from_multi_file_diff(&diff)
        }
    }

    /// Create a CodeDiff from git with custom config.
    ///
    /// This is a convenience method that combines `from_git()` with `with_config()`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ratatui_toolkit::code_diff::{CodeDiff, DiffConfig};
    ///
    /// let diff = CodeDiff::from_git_with_config(
    ///     DiffConfig::new()
    ///         .sidebar_enabled(true)
    ///         .show_line_numbers(true)
    /// );
    /// ```
    pub fn from_git_with_config(config: DiffConfig) -> Self {
        let diff = get_git_diff();
        if diff.trim().is_empty() {
            Self::new().with_config(config)
        } else {
            Self::from_multi_file_diff(&diff).with_config(config)
        }
    }
}

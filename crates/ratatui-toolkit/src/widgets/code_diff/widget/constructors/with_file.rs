//! Builder method to add a file with its diff.

use crate::widgets::code_diff::code_diff::CodeDiff;
use crate::widgets::code_diff::diff_file_tree::FileStatus;
use crate::widgets::code_diff::diff_hunk::DiffHunk;
use crate::widgets::code_diff::diff_line::DiffLine;

impl CodeDiff {
    /// Adds a file with its diff to the widget.
    ///
    /// This is used for multi-file diffs. The file will appear in the sidebar
    /// file tree with the specified status, and its diff content will be stored
    /// for display when selected.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to display
    /// * `status` - The file's modification status (Modified, Added, Deleted, Renamed)
    /// * `diff_text` - The unified diff text for this file
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{CodeDiff, DiffConfig};
    /// use ratatui_toolkit::widgets::code_diff::diff_file_tree::FileStatus;
    ///
    /// let diff_text = r#"--- a/src/lib.rs
    /// +++ b/src/lib.rs
    /// @@ -1,3 +1,4 @@
    ///  fn main() {
    /// +    println!("Hello");
    ///  }
    /// "#;
    ///
    /// let widget = CodeDiff::new()
    ///     .with_config(DiffConfig::new().sidebar_enabled(true))
    ///     .with_file("src/lib.rs", FileStatus::Modified, diff_text);
    /// ```
    pub fn with_file(mut self, path: &str, status: FileStatus, diff_text: &str) -> Self {
        // Parse the diff text into hunks
        let hunks = parse_file_diff(diff_text);

        // Add to file tree
        self.file_tree.add_file(path, status);

        // Store the hunks for this file
        self.file_diffs.insert(path.to_string(), hunks.clone());

        // If this is the first file, also set it as the current hunks
        if self.file_path.is_none() {
            self.file_path = Some(path.to_string());
            self.hunks = hunks;
        }

        self
    }
}

/// Parses unified diff format text into hunks (for a single file).
fn parse_file_diff(diff_text: &str) -> Vec<DiffHunk> {
    use crate::widgets::code_diff::diff_hunk::DiffHunk;

    let mut hunks: Vec<DiffHunk> = Vec::new();
    let mut current_hunk: Option<DiffHunk> = None;
    let mut old_line_num: usize = 0;
    let mut new_line_num: usize = 0;

    for line in diff_text.lines() {
        // Skip file headers
        if line.starts_with("--- ") || line.starts_with("+++ ") {
            continue;
        }

        // Parse hunk header
        if line.starts_with("@@") {
            // Save previous hunk if exists
            if let Some(hunk) = current_hunk.take() {
                hunks.push(hunk);
            }

            // Parse new hunk header
            if let Some(hunk) = DiffHunk::from_header(line) {
                old_line_num = hunk.old_start;
                new_line_num = hunk.new_start;
                current_hunk = Some(hunk);
            }
            continue;
        }

        // Parse diff lines within a hunk
        if let Some(ref mut hunk) = current_hunk {
            let diff_line = if let Some(content) = line.strip_prefix('+') {
                let line = DiffLine::added(content, new_line_num);
                new_line_num += 1;
                Some(line)
            } else if let Some(content) = line.strip_prefix('-') {
                let line = DiffLine::removed(content, old_line_num);
                old_line_num += 1;
                Some(line)
            } else if let Some(content) = line.strip_prefix(' ') {
                let line = DiffLine::context(content, old_line_num, new_line_num);
                old_line_num += 1;
                new_line_num += 1;
                Some(line)
            } else if line.is_empty() {
                // Empty line in diff (context line with no content)
                let line = DiffLine::context("", old_line_num, new_line_num);
                old_line_num += 1;
                new_line_num += 1;
                Some(line)
            } else {
                None
            };

            if let Some(diff_line) = diff_line {
                hunk.add_line(diff_line);
            }
        }
    }

    // Don't forget the last hunk
    if let Some(hunk) = current_hunk {
        hunks.push(hunk);
    }

    hunks
}

//! Constructor for parsing multi-file git diff output.

use std::collections::HashMap;

use crate::primitives::resizable_grid::ResizableGrid;
use crate::services::theme::AppTheme;
use crate::widgets::code_diff::code_diff::CodeDiff;
use crate::widgets::code_diff::diff_config::DiffConfig;
use crate::widgets::code_diff::diff_file_tree::{DiffFileTree, FileStatus};
use crate::widgets::code_diff::diff_hunk::DiffHunk;
use crate::widgets::code_diff::diff_line::DiffLine;

impl CodeDiff {
    /// Creates a diff widget by parsing multi-file git diff output.
    ///
    /// Parses output from `git diff` that may contain multiple files:
    ///
    /// ```text
    /// diff --git a/file1.rs b/file1.rs
    /// --- a/file1.rs
    /// +++ b/file1.rs
    /// @@ -1,4 +1,5 @@
    /// ...
    /// diff --git a/file2.rs b/file2.rs
    /// --- a/file2.rs
    /// +++ b/file2.rs
    /// @@ -1,3 +1,4 @@
    /// ...
    /// ```
    ///
    /// The sidebar is automatically enabled when multiple files are detected.
    ///
    /// # Arguments
    ///
    /// * `diff_text` - The multi-file diff text to parse
    ///
    /// # Returns
    ///
    /// A new `CodeDiff` instance with parsed files and sidebar enabled
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// let diff_text = r#"diff --git a/src/lib.rs b/src/lib.rs
    /// --- a/src/lib.rs
    /// +++ b/src/lib.rs
    /// @@ -1,3 +1,4 @@
    ///  fn main() {
    /// +    println!("Hello");
    ///  }
    /// diff --git a/src/utils.rs b/src/utils.rs
    /// new file mode 100644
    /// --- /dev/null
    /// +++ b/src/utils.rs
    /// @@ -0,0 +1,3 @@
    /// +pub fn helper() {
    /// +}
    /// "#;
    ///
    /// let widget = CodeDiff::from_multi_file_diff(diff_text);
    /// ```
    pub fn from_multi_file_diff(diff_text: &str) -> Self {
        let files = parse_multi_file_diff(diff_text);
        let config = DiffConfig::new();

        // Build file tree entries
        let file_entries: Vec<(&str, FileStatus)> =
            files.iter().map(|f| (f.path.as_str(), f.status)).collect();

        let file_tree = if file_entries.is_empty() {
            DiffFileTree::new()
        } else {
            DiffFileTree::from_paths(&file_entries).with_focus(true)
        };

        // Build file diffs map
        let file_diffs: HashMap<String, Vec<DiffHunk>> = files
            .iter()
            .map(|f| (f.path.clone(), f.hunks.clone()))
            .collect();

        // Get first file's hunks for initial display
        let (file_path, hunks) = if let Some(first) = files.first() {
            (Some(first.path.clone()), first.hunks.clone())
        } else {
            (None, Vec::new())
        };

        // Enable sidebar if multiple files
        let show_sidebar = files.len() > 1 || config.sidebar_enabled;

        let mut sidebar_split = ResizableGrid::new(0);
        sidebar_split.split_pane_vertically(0);
        let split_index = 0;
        sidebar_split.resize_split(split_index, config.sidebar_default_width);

        Self {
            file_path,
            hunks,
            scroll_offset: 0,
            file_tree,
            file_diffs,
            show_sidebar,
            sidebar_split,
            sidebar_focused: true,
            config,
            theme: AppTheme::default(),
            area: None,
        }
    }
}

/// Parsed file info from multi-file diff.
struct ParsedFile {
    path: String,
    status: FileStatus,
    hunks: Vec<DiffHunk>,
}

/// Parses multi-file git diff output into individual file diffs.
fn parse_multi_file_diff(diff_text: &str) -> Vec<ParsedFile> {
    let mut files: Vec<ParsedFile> = Vec::new();
    let mut current_file: Option<ParsedFile> = None;
    let mut current_hunk: Option<DiffHunk> = None;
    let mut old_line_num: usize = 0;
    let mut new_line_num: usize = 0;
    let mut is_new_file = false;
    let mut is_deleted_file = false;

    for line in diff_text.lines() {
        // Start of a new file diff
        if line.starts_with("diff --git ") {
            // Save previous file
            if let Some(mut file) = current_file.take() {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }
                files.push(file);
            }

            // Extract path from "diff --git a/path b/path"
            let path = extract_git_diff_path(line);

            current_file = Some(ParsedFile {
                path,
                status: FileStatus::Modified, // Will be updated based on headers
                hunks: Vec::new(),
            });
            is_new_file = false;
            is_deleted_file = false;
            continue;
        }

        // Check for new/deleted file mode
        if line.starts_with("new file mode") {
            is_new_file = true;
            if let Some(ref mut file) = current_file {
                file.status = FileStatus::Added;
            }
            continue;
        }
        if line.starts_with("deleted file mode") {
            is_deleted_file = true;
            if let Some(ref mut file) = current_file {
                file.status = FileStatus::Deleted;
            }
            continue;
        }
        if line.starts_with("rename from") || line.starts_with("rename to") {
            if let Some(ref mut file) = current_file {
                file.status = FileStatus::Renamed;
            }
            continue;
        }

        // Skip file headers (we've already extracted the path)
        if line.starts_with("--- ") {
            // Detect deleted file from "--- a/path"
            if line == "--- /dev/null" {
                is_new_file = true;
                if let Some(ref mut file) = current_file {
                    file.status = FileStatus::Added;
                }
            }
            continue;
        }
        if line.starts_with("+++ ") {
            // Detect new file from "+++ /dev/null"
            if line == "+++ /dev/null" {
                is_deleted_file = true;
                if let Some(ref mut file) = current_file {
                    file.status = FileStatus::Deleted;
                }
            }
            continue;
        }

        // Parse hunk header
        if line.starts_with("@@") {
            // Save previous hunk
            if let Some(ref mut file) = current_file {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }
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
                let diff_line = DiffLine::added(content, new_line_num);
                new_line_num += 1;
                Some(diff_line)
            } else if let Some(content) = line.strip_prefix('-') {
                let diff_line = DiffLine::removed(content, old_line_num);
                old_line_num += 1;
                Some(diff_line)
            } else if let Some(content) = line.strip_prefix(' ') {
                let diff_line = DiffLine::context(content, old_line_num, new_line_num);
                old_line_num += 1;
                new_line_num += 1;
                Some(diff_line)
            } else if line.is_empty() {
                // Empty line in diff (context line with no content)
                let diff_line = DiffLine::context("", old_line_num, new_line_num);
                old_line_num += 1;
                new_line_num += 1;
                Some(diff_line)
            } else {
                None
            };

            if let Some(diff_line) = diff_line {
                hunk.add_line(diff_line);
            }
        }
    }

    // Don't forget the last file
    if let Some(mut file) = current_file {
        if let Some(hunk) = current_hunk {
            file.hunks.push(hunk);
        }
        files.push(file);
    }

    // Suppress unused variable warnings
    let _ = is_new_file;
    let _ = is_deleted_file;

    files
}

/// Extracts file path from "diff --git a/path b/path" line.
fn extract_git_diff_path(line: &str) -> String {
    // Format: "diff --git a/path b/path"
    // We want the path after "b/"
    if let Some(b_part) = line.split(" b/").nth(1) {
        b_part.to_string()
    } else if let Some(a_part) = line.strip_prefix("diff --git a/") {
        // Fallback: take everything before " b/"
        if let Some(path) = a_part.split(' ').next() {
            path.to_string()
        } else {
            a_part.to_string()
        }
    } else {
        String::new()
    }
}

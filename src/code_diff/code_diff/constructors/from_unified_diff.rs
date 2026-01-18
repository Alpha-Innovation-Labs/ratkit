use std::collections::HashMap;

use crate::code_diff::code_diff::CodeDiff;
use crate::code_diff::diff_config::DiffConfig;
use crate::code_diff::diff_hunk::DiffHunk;
use crate::code_diff::diff_line::DiffLine;
use crate::diff_file_tree::DiffFileTree;
use crate::resizable_split::ResizableSplit;
use crate::theme::AppTheme;

impl CodeDiff {
    /// Creates a diff widget by parsing unified diff format text.
    ///
    /// Parses the standard unified diff format used by `git diff`, `diff -u`, etc.
    ///
    /// # Arguments
    ///
    /// * `diff_text` - The unified diff text to parse
    ///
    /// # Returns
    ///
    /// A new `CodeDiff` instance with parsed hunks
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// let diff_text = r#"
    /// --- a/file.txt
    /// +++ b/file.txt
    /// @@ -1,4 +1,5 @@
    ///  context line
    /// -removed line
    /// +added line
    ///  more context
    /// "#;
    ///
    /// let diff = CodeDiff::from_unified_diff(diff_text);
    /// assert!(!diff.hunks.is_empty());
    /// ```
    pub fn from_unified_diff(diff_text: &str) -> Self {
        let (file_path, hunks) = parse_unified_diff(diff_text);
        let config = DiffConfig::new();

        // Create ResizableSplit with config values
        let mut sidebar_split = ResizableSplit::new(config.sidebar_default_width);
        sidebar_split.min_percent = config.sidebar_min_width;
        sidebar_split.max_percent = config.sidebar_max_width;

        Self {
            file_path,
            hunks,
            scroll_offset: 0,
            file_tree: DiffFileTree::new(),
            file_diffs: HashMap::new(),
            show_sidebar: config.sidebar_enabled,
            sidebar_split,
            sidebar_focused: true,
            config,
            theme: AppTheme::default(),
        }
    }
}

/// Parses unified diff format text into hunks.
///
/// # Arguments
///
/// * `diff_text` - The unified diff text to parse
///
/// # Returns
///
/// A tuple of (optional file path, vector of hunks)
fn parse_unified_diff(diff_text: &str) -> (Option<String>, Vec<DiffHunk>) {
    let mut file_path: Option<String> = None;
    let mut hunks: Vec<DiffHunk> = Vec::new();
    let mut current_hunk: Option<DiffHunk> = None;
    let mut old_line_num: usize = 0;
    let mut new_line_num: usize = 0;

    for line in diff_text.lines() {
        // Parse file headers
        if line.starts_with("--- ") {
            // Old file header - extract path
            let path = line
                .strip_prefix("--- ")
                .and_then(|p| p.strip_prefix("a/"))
                .unwrap_or_else(|| line.strip_prefix("--- ").unwrap_or(""));
            if file_path.is_none() && !path.is_empty() {
                file_path = Some(path.to_string());
            }
            continue;
        }

        if line.starts_with("+++ ") {
            // New file header - skip
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

    (file_path, hunks)
}

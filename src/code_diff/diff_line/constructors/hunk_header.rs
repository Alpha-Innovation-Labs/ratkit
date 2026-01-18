use crate::code_diff::diff_line::DiffLine;
use crate::code_diff::enums::DiffLineKind;

impl DiffLine {
    /// Creates a hunk header diff line.
    ///
    /// Hunk headers mark the start of a diff section and contain line number
    /// information (e.g., `@@ -1,4 +1,5 @@ function_name`).
    ///
    /// # Arguments
    ///
    /// * `content` - The full hunk header text
    ///
    /// # Returns
    ///
    /// A new `DiffLine` with `DiffLineKind::HunkHeader`
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{DiffLine, DiffLineKind};
    ///
    /// let line = DiffLine::hunk_header("@@ -1,4 +1,5 @@ fn main()");
    /// assert!(matches!(line.kind, DiffLineKind::HunkHeader));
    /// ```
    pub fn hunk_header(content: impl Into<String>) -> Self {
        Self {
            kind: DiffLineKind::HunkHeader,
            content: content.into(),
            old_line_num: None,
            new_line_num: None,
        }
    }
}

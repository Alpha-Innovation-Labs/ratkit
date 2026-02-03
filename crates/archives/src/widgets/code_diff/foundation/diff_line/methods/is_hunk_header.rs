use crate::widgets::code_diff::diff_line::DiffLine;
use crate::widgets::code_diff::enums::DiffLineKind;

impl DiffLine {
    /// Returns true if this is a hunk header line.
    ///
    /// # Returns
    ///
    /// `true` if the line kind is `HunkHeader`, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::DiffLine;
    ///
    /// let line = DiffLine::hunk_header("@@ -1,4 +1,5 @@");
    /// assert!(line.is_hunk_header());
    /// ```
    pub fn is_hunk_header(&self) -> bool {
        matches!(self.kind, DiffLineKind::HunkHeader)
    }
}

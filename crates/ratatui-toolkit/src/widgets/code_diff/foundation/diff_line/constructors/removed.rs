use crate::widgets::code_diff::diff_line::DiffLine;
use crate::widgets::code_diff::enums::DiffLineKind;

impl DiffLine {
    /// Creates a removed diff line.
    ///
    /// Removed lines exist only in the old version of the file and are typically
    /// displayed with a red background and a '-' prefix.
    ///
    /// # Arguments
    ///
    /// * `content` - The text content of the line
    /// * `old_line_num` - Line number in the old version
    ///
    /// # Returns
    ///
    /// A new `DiffLine` with `DiffLineKind::Removed`
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{DiffLine, DiffLineKind};
    ///
    /// let line = DiffLine::removed("deleted line", 5);
    /// assert!(matches!(line.kind, DiffLineKind::Removed));
    /// assert!(line.new_line_num.is_none());
    /// ```
    pub fn removed(content: impl Into<String>, old_line_num: usize) -> Self {
        Self {
            kind: DiffLineKind::Removed,
            content: content.into(),
            old_line_num: Some(old_line_num),
            new_line_num: None,
        }
    }
}

use crate::widgets::code_diff::diff_line::DiffLine;
use crate::widgets::code_diff::enums::DiffLineKind;

impl DiffLine {
    /// Creates an added diff line.
    ///
    /// Added lines exist only in the new version of the file and are typically
    /// displayed with a green background and a '+' prefix.
    ///
    /// # Arguments
    ///
    /// * `content` - The text content of the line
    /// * `new_line_num` - Line number in the new version
    ///
    /// # Returns
    ///
    /// A new `DiffLine` with `DiffLineKind::Added`
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{DiffLine, DiffLineKind};
    ///
    /// let line = DiffLine::added("new line", 10);
    /// assert!(matches!(line.kind, DiffLineKind::Added));
    /// assert!(line.old_line_num.is_none());
    /// ```
    pub fn added(content: impl Into<String>, new_line_num: usize) -> Self {
        Self {
            kind: DiffLineKind::Added,
            content: content.into(),
            old_line_num: None,
            new_line_num: Some(new_line_num),
        }
    }
}

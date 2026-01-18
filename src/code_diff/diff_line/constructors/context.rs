use crate::code_diff::diff_line::DiffLine;
use crate::code_diff::enums::DiffLineKind;

impl DiffLine {
    /// Creates a context (unchanged) diff line.
    ///
    /// Context lines appear in both the old and new versions of the file
    /// and are shown without highlighting to provide surrounding context.
    ///
    /// # Arguments
    ///
    /// * `content` - The text content of the line
    /// * `old_line_num` - Line number in the old version
    /// * `new_line_num` - Line number in the new version
    ///
    /// # Returns
    ///
    /// A new `DiffLine` with `DiffLineKind::Context`
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{DiffLine, DiffLineKind};
    ///
    /// let line = DiffLine::context("unchanged line", 5, 7);
    /// assert!(matches!(line.kind, DiffLineKind::Context));
    /// ```
    pub fn context(content: impl Into<String>, old_line_num: usize, new_line_num: usize) -> Self {
        Self {
            kind: DiffLineKind::Context,
            content: content.into(),
            old_line_num: Some(old_line_num),
            new_line_num: Some(new_line_num),
        }
    }
}

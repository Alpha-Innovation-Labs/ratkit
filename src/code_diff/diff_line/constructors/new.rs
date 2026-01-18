use crate::code_diff::diff_line::DiffLine;
use crate::code_diff::enums::DiffLineKind;

impl DiffLine {
    /// Creates a new diff line with all fields specified.
    ///
    /// # Arguments
    ///
    /// * `kind` - The type of diff line
    /// * `content` - The text content of the line
    /// * `old_line_num` - Line number in the old version (None for added lines)
    /// * `new_line_num` - Line number in the new version (None for removed lines)
    ///
    /// # Returns
    ///
    /// A new `DiffLine` instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{DiffLine, DiffLineKind};
    ///
    /// let line = DiffLine::new(
    ///     DiffLineKind::Context,
    ///     "unchanged line",
    ///     Some(10),
    ///     Some(12),
    /// );
    /// ```
    pub fn new(
        kind: DiffLineKind,
        content: impl Into<String>,
        old_line_num: Option<usize>,
        new_line_num: Option<usize>,
    ) -> Self {
        Self {
            kind,
            content: content.into(),
            old_line_num,
            new_line_num,
        }
    }
}

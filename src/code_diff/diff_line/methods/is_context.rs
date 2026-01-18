use crate::code_diff::diff_line::DiffLine;
use crate::code_diff::enums::DiffLineKind;

impl DiffLine {
    /// Returns true if this is a context (unchanged) line.
    ///
    /// # Returns
    ///
    /// `true` if the line kind is `Context`, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::DiffLine;
    ///
    /// let line = DiffLine::context("unchanged", 1, 1);
    /// assert!(line.is_context());
    /// ```
    pub fn is_context(&self) -> bool {
        matches!(self.kind, DiffLineKind::Context)
    }
}

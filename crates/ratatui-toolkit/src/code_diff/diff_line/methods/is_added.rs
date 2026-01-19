use crate::code_diff::diff_line::DiffLine;
use crate::code_diff::enums::DiffLineKind;

impl DiffLine {
    /// Returns true if this is an added line.
    ///
    /// # Returns
    ///
    /// `true` if the line kind is `Added`, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::DiffLine;
    ///
    /// let line = DiffLine::added("new line", 5);
    /// assert!(line.is_added());
    /// ```
    pub fn is_added(&self) -> bool {
        matches!(self.kind, DiffLineKind::Added)
    }
}

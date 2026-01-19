use crate::code_diff::diff_line::DiffLine;
use crate::code_diff::enums::DiffLineKind;

impl DiffLine {
    /// Returns true if this is a removed line.
    ///
    /// # Returns
    ///
    /// `true` if the line kind is `Removed`, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::DiffLine;
    ///
    /// let line = DiffLine::removed("deleted line", 5);
    /// assert!(line.is_removed());
    /// ```
    pub fn is_removed(&self) -> bool {
        matches!(self.kind, DiffLineKind::Removed)
    }
}

use crate::widgets::code_diff::diff_line::DiffLine;
use crate::widgets::code_diff::enums::DiffLineKind;

impl DiffLine {
    /// Returns the prefix character for this line kind.
    ///
    /// - Context lines: ' ' (space)
    /// - Added lines: '+'
    /// - Removed lines: '-'
    /// - Hunk headers: '@'
    ///
    /// # Returns
    ///
    /// A single character prefix for display
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::DiffLine;
    ///
    /// let added = DiffLine::added("new line", 5);
    /// assert_eq!(added.prefix(), '+');
    ///
    /// let removed = DiffLine::removed("old line", 5);
    /// assert_eq!(removed.prefix(), '-');
    /// ```
    pub fn prefix(&self) -> char {
        match self.kind {
            DiffLineKind::Context => ' ',
            DiffLineKind::Added => '+',
            DiffLineKind::Removed => '-',
            DiffLineKind::HunkHeader => '@',
        }
    }
}

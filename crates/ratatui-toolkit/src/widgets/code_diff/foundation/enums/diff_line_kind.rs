//! Diff line kind enumeration.
//!
//! Represents the type of a line in a diff output.

/// Represents the type of a line in a diff.
///
/// Each line in a diff can be one of several types:
/// - Context lines that are unchanged
/// - Added lines that exist only in the new version
/// - Removed lines that exist only in the old version
/// - Hunk headers that mark the start of a diff hunk
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::code_diff::DiffLineKind;
///
/// let kind = DiffLineKind::Added;
/// assert!(matches!(kind, DiffLineKind::Added));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffLineKind {
    /// An unchanged context line present in both old and new versions.
    #[default]
    Context,

    /// A line that was added in the new version.
    /// Typically displayed with a green background and '+' prefix.
    Added,

    /// A line that was removed from the old version.
    /// Typically displayed with a red background and '-' prefix.
    Removed,

    /// A hunk header line (e.g., `@@ -1,4 +1,5 @@ context`).
    /// Marks the start of a diff section with line number information.
    HunkHeader,
}

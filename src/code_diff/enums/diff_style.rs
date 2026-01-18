//! Diff display style enumeration.
//!
//! Represents the layout style for displaying diffs.

/// Represents the display style for a diff view.
///
/// Determines how the diff is laid out in the terminal.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::code_diff::DiffStyle;
///
/// let style = DiffStyle::SideBySide;
/// assert!(matches!(style, DiffStyle::SideBySide));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffStyle {
    /// Side-by-side display with old version on the left and new version on the right.
    /// This is the default style, similar to VS Code's diff viewer.
    #[default]
    SideBySide,

    /// Unified diff display where changes are shown inline.
    /// Removed lines appear above added lines.
    Unified,

    /// Inline diff where changes are highlighted within lines.
    /// Useful for seeing character-level changes.
    Inline,
}

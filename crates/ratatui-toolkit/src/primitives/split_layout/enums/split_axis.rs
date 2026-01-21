/// Axis for splitting panes.
///
/// Use vertical splits for left/right panes and horizontal splits for
/// top/bottom panes.
///
/// # Example
/// ```rust
/// use ratatui_toolkit::primitives::split_layout::SplitAxis;
///
/// let axis = SplitAxis::Vertical;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitAxis {
    /// Vertical split (left/right panes).
    Vertical,
    /// Horizontal split (top/bottom panes).
    Horizontal,
}

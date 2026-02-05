//! Selection state for markdown widget text selection and copy.

pub mod constructors;
pub mod methods;

pub use constructors::*;
pub use methods::*;

use crate::widgets::markdown_widget::foundation::types::SelectionPos;

/// Selection state for markdown widget.
///
/// Tracks whether selection mode is active and the selection bounds.
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    /// Whether selection mode is active.
    pub active: bool,
    /// Selection anchor (start point).
    pub anchor: Option<SelectionPos>,
    /// Current cursor/end position.
    pub cursor: Option<SelectionPos>,
    /// Cached rendered lines for stable selection.
    pub frozen_lines: Option<Vec<ratatui::text::Line<'static>>>,
    /// Width when lines were frozen.
    pub frozen_width: usize,
    /// Last copied text (for showing toast notification).
    pub last_copied_text: Option<String>,
}

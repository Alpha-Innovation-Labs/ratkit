//! Constructor for MarkdownState.

use crate::markdown_widget::state::markdown_state::MarkdownState;
use crate::markdown_widget::state::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    ScrollState, SelectionState, SourceState, VimState,
};

impl MarkdownState {
    /// Create a new MarkdownState with all default values.
    ///
    /// This is equivalent to `MarkdownState::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new MarkdownState with custom display settings.
    pub fn with_display(display: DisplaySettings) -> Self {
        Self {
            display,
            ..Default::default()
        }
    }
}

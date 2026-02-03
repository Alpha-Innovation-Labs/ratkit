//! Tree view state for tracking selection, expansion, and filtering.

pub mod constructors;
pub mod methods;
pub mod traits;

use std::collections::HashSet;

/// Tree view state for StatefulWidget pattern.
///
/// Tracks the current selection, expanded nodes, scroll offset,
/// and filter state for the tree view.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::tree_view::TreeViewState;
///
/// let mut state = TreeViewState::new();
/// state.select(vec![0, 1]);
/// state.expand(vec![0]);
/// ```
#[derive(Debug, Clone, Default)]
pub struct TreeViewState {
    /// Currently selected node path (indices from root)
    pub selected_path: Option<Vec<usize>>,
    /// Set of expanded node paths
    pub expanded: HashSet<Vec<usize>>,
    /// Vertical scroll offset
    pub offset: usize,
    /// Current filter text
    pub filter: Option<String>,
    /// Whether filter mode is active
    pub filter_mode: bool,
}

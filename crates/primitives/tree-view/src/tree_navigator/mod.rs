//! Tree navigator with configurable keybindings.

pub mod constructors;
pub mod methods;
pub mod traits;

use crate::keybindings::TreeKeyBindings;

/// Tree navigator with configurable keybindings.
///
/// Provides navigation methods for tree views with customizable key mappings.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::tree_view::{TreeNavigator, TreeNode, TreeViewState};
///
/// let navigator = TreeNavigator::new();
/// let nodes = vec![TreeNode::new("Item")];
/// let mut state = TreeViewState::new();
/// navigator.select_next(&nodes, &mut state);
/// ```
#[derive(Clone)]
pub struct TreeNavigator {
    /// The keybindings for navigation.
    pub keybindings: TreeKeyBindings,
}

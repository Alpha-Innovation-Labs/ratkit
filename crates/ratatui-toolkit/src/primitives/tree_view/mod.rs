//! Tree view widget for hierarchical data display.
//!
//! This module provides a complete tree view implementation with:
//! - `TreeNode` - Hierarchical data structure for tree nodes
//! - `NodeState` - State information for rendering nodes
//! - `TreeViewState` - Selection, expansion, and filter state
//! - `TreeView` - The main widget (takes ownership of nodes)
//! - `TreeViewRef` - Widget that borrows nodes (avoids cloning)
//! - `TreeNavigator` - Keyboard navigation with configurable keybindings
//! - `TreeKeyBindings` - Customizable keybindings for navigation
//!
//! # Example
//!
//! ```rust
//! use ratatui_toolkit::tree_view::{TreeNode, TreeView, TreeViewState};
//!
//! let nodes = vec![
//!     TreeNode::with_children("Root", vec![
//!         TreeNode::new("Child 1"),
//!         TreeNode::new("Child 2"),
//!     ]),
//! ];
//!
//! let tree = TreeView::new(nodes)
//!     .render_fn(|data, state| {
//!         ratatui::text::Line::from(*data)
//!     });
//!
//! let mut state = TreeViewState::new();
//! ```

pub mod helpers;
mod keybindings;
mod node_state;
mod tree_navigator;
mod tree_node;
mod tree_view_ref;
mod tree_view_state;
mod widget;

// Re-export keybindings
pub use keybindings::TreeKeyBindings;

// Re-export helpers
pub use helpers::get_visible_paths;
pub use helpers::get_visible_paths_filtered;
pub use helpers::matches_filter;

// Re-export node_state
pub use node_state::NodeState;

// Re-export tree_navigator
pub use tree_navigator::TreeNavigator;

// Re-export tree_node
pub use tree_node::TreeNode;

// Re-export widget
pub use widget::NodeRenderFn;
pub use widget::TreeView;

// Re-export tree_view_ref
pub use tree_view_ref::NodeFilterFn;
pub use tree_view_ref::NodeRenderRefFn;
pub use tree_view_ref::TreeViewRef;

// Re-export tree_view_state
pub use tree_view_state::TreeViewState;

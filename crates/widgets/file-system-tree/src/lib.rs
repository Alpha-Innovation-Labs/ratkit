//! File system tree widget for ratatui.
//!
//! A tree view widget for browsing file system directories with:
//! - Directory and file display with icons
//! - Expand/collapse directories
//! - Selection navigation
//! - Filter mode for searching
//! - Hidden file filtering
//!
//! # Example
//!
//! ```rust,no_run
//! use std::path::PathBuf;
//! use ratkit_file_system_tree::{FileSystemTree, FileSystemTreeState};
//!
//! let tree = FileSystemTree::new(PathBuf::from(".")).unwrap();
//! let mut state = FileSystemTreeState::new();
//! ```

mod entry;
mod state;
mod config;
mod tree_node;
mod widget;

pub use config::FileSystemTreeConfig;
pub use entry::FileSystemEntry;
pub use state::FileSystemTreeState;
pub use tree_node::FileSystemTreeNode;
pub use widget::FileSystemTree;

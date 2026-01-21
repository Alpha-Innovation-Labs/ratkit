//! File system tree browser component
//!
//! Provides a tree view for browsing file system directories.

pub mod constructors;
pub mod methods;
pub mod traits;

use std::path::PathBuf;

use crate::primitives::tree_view::TreeNode;

use ratatui::style::Style;
use ratatui::widgets::Block;

/// Represents a file system entry (file or directory)
#[derive(Debug, Clone)]
pub struct FileSystemEntry {
    /// Name of the file/directory
    pub name: String,
    /// Full path
    pub path: PathBuf,
    /// Whether this is a directory
    pub is_dir: bool,
    /// Whether this entry is hidden (starts with .)
    pub is_hidden: bool,
}

/// Configuration for the file system tree
#[derive(Debug, Clone, Copy)]
pub struct FileSystemTreeConfig {
    /// Show hidden files (starting with .)
    pub show_hidden: bool,
    /// Use dark theme for icons (true = dark, false = light)
    pub use_dark_theme: bool,
    /// Style for directories
    pub dir_style: Style,
    /// Style for files
    pub file_style: Style,
    /// Style for selected items
    pub selected_style: Style,
}

/// File system tree browser widget
#[derive(Clone)]
pub struct FileSystemTree<'a> {
    /// Root directory to browse
    pub root_path: PathBuf,
    /// Tree nodes built from file system
    pub nodes: Vec<TreeNode<FileSystemEntry>>,
    /// Configuration
    pub(crate) config: FileSystemTreeConfig,
    /// Optional block wrapper
    block: Option<Block<'a>>,
}

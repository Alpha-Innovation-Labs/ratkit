//! Diff file tree widget for displaying changed files.
//!
//! A tree view widget for displaying changed files in a diff, similar to
//! gitui's unstaged changes panel or VS Code's source control view.
//!
//! This implementation wraps the generic TreeView component for simplified code.
//!
//! # Features
//!
//! - **Hierarchical display**: Groups files by directory structure
//! - **Status markers**: Visual indicators for file status (M, +, -, R)
//! - **Collapsible directories**: Expand/collapse with keyboard
//! - **Color coding**: Green (added), yellow (modified), red (deleted), blue (renamed)
//! - **Keyboard navigation**: Uses TreeView's navigation
//!
//! # Structure
//!
//! - [`DiffFileTree`] - The main tree widget (wraps TreeView)
//! - [`DiffFileEntry`] - Data type for tree nodes (path + status)
//! - [`FileStatus`] - File modification status enum
//!
//! # Example
//!
//! ```rust
//! use ratatui_toolkit::diff_file_tree::{DiffFileTree, FileStatus};
//!
//! let files = vec![
//!     ("src/lib.rs", FileStatus::Modified),
//!     ("src/utils/helper.rs", FileStatus::Added),
//!     ("src/old_module.rs", FileStatus::Deleted),
//! ];
//!
//! let tree = DiffFileTree::from_paths(&files);
//!
//! // Render with ratatui Widget trait...
//! // frame.render_widget(&tree, area);
//! ```
//!
//! # Visual Style
//!
//! ```text
//! M  src/
//!      lib.rs
//! +    utils/
//! +      helper.rs
//! -    old_module.rs
//! ```

pub mod constructors;
pub mod helpers;
pub mod methods;
pub mod traits;

pub use helpers::file_icon;

use crate::theme::AppTheme;
use crate::tree_view::{TreeNode, TreeViewState};
use ratatui::style::Color;

/// The modification status of a file in a diff.
///
/// Each status has an associated color and prefix character
/// for display in the file tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileStatus {
    /// File was added (new file).
    Added,
    /// File was modified (existing file changed).
    #[default]
    Modified,
    /// File was deleted.
    Deleted,
    /// File was renamed.
    Renamed,
}

impl FileStatus {
    /// Returns the prefix character for this status.
    #[must_use]
    pub fn prefix(&self) -> &'static str {
        match self {
            Self::Added => "+",
            Self::Modified => "M",
            Self::Deleted => "-",
            Self::Renamed => "R",
        }
    }

    /// Returns the color associated with this status.
    #[must_use]
    pub fn color(&self) -> Color {
        match self {
            Self::Added => Color::Green,
            Self::Modified => Color::Yellow,
            Self::Deleted => Color::Red,
            Self::Renamed => Color::Blue,
        }
    }
}

/// A single file or directory entry in a diff tree.
///
/// This is the data type stored in each [`TreeNode`](crate::tree_view::TreeNode).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffFileEntry {
    /// The display name (filename or directory name).
    pub name: String,
    /// The full path to this file/directory.
    pub full_path: String,
    /// Whether this entry is a directory.
    pub is_dir: bool,
    /// The modification status (None for directories).
    pub status: Option<FileStatus>,
}

impl DiffFileEntry {
    /// Creates a new file entry.
    #[must_use]
    pub fn file(name: &str, full_path: &str, status: FileStatus) -> Self {
        Self {
            name: name.to_string(),
            full_path: full_path.to_string(),
            is_dir: false,
            status: Some(status),
        }
    }

    /// Creates a new directory entry.
    #[must_use]
    pub fn directory(name: &str, full_path: &str) -> Self {
        Self {
            name: name.to_string(),
            full_path: full_path.to_string(),
            is_dir: true,
            status: None,
        }
    }
}

/// A tree widget for displaying changed files in a diff.
///
/// Renders a hierarchical view of files with their modification status,
/// similar to gitui's file tree in the changes panel.
///
/// This is a wrapper around [`TreeView`](crate::tree_view::TreeView) that
/// provides a specialized API for diff file trees.
#[derive(Debug, Clone)]
pub struct DiffFileTree {
    /// The tree nodes containing diff file entries.
    pub nodes: Vec<TreeNode<DiffFileEntry>>,
    /// State for selection and expansion (managed by TreeView).
    pub state: TreeViewState,
    /// Currently selected visible row index.
    pub selected_index: usize,
    /// Whether this widget currently has focus.
    pub focused: bool,
    /// Application theme for styling.
    pub theme: AppTheme,
}

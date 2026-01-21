//! Code diff widget for displaying side-by-side diffs.
//!
//! The main widget that renders diff hunks in a side-by-side or unified view,
//! similar to VS Code's diff viewer. Optionally includes an integrated file
//! tree sidebar for multi-file diffs.
//!
//! # Structure
//!
//! - [`CodeDiff`] - The main diff widget struct
//! - [`constructors`] - Constructor functions (`new`, `from_unified_diff`, `from_multi_file_diff`)
//! - [`methods`] - Instance methods for configuration and data access
//! - [`traits`] - Trait implementations (`Widget`, `Default`)
//!
//! # Example
//!
//! ```rust
//! use ratatui_toolkit::code_diff::CodeDiff;
//!
//! let diff_text = r#"
//! --- a/file.txt
//! +++ b/file.txt
//! @@ -1,4 +1,5 @@
//!  context line
//! -removed line
//! +added line
//!  more context
//! "#;
//!
//! let widget = CodeDiff::from_unified_diff(diff_text);
//! ```
//!
//! # Multi-file diff with sidebar
//!
//! ```rust
//! use ratatui_toolkit::code_diff::{CodeDiff, DiffConfig};
//! use ratatui_toolkit::widgets::code_diff::diff_file_tree::FileStatus;
//!
//! let diff = CodeDiff::new()
//!     .with_config(DiffConfig::new().sidebar_enabled(true))
//!     .with_file("src/lib.rs", FileStatus::Modified, "--- a/src/lib.rs\n+++ b/src/lib.rs\n...")
//!     .with_file("src/new.rs", FileStatus::Added, "--- /dev/null\n+++ b/src/new.rs\n...");
//! ```

pub mod constructors;
pub mod methods;
pub mod traits;

use std::collections::HashMap;

use crate::primitives::resizable_split::ResizableSplit;
use crate::services::theme::AppTheme;
use crate::widgets::code_diff::diff_config::DiffConfig;
use crate::widgets::code_diff::diff_file_tree::DiffFileTree;
use crate::widgets::code_diff::diff_hunk::DiffHunk;

/// A widget for displaying code diffs in a terminal UI.
///
/// `CodeDiff` renders diff hunks in a side-by-side format (like VS Code) or
/// unified format, with support for syntax highlighting, line numbers, and
/// visual markers for added/removed lines.
///
/// # Layout
///
/// In side-by-side mode:
/// - Left panel shows the old/original version
/// - Right panel shows the new/modified version
/// - Lines are aligned horizontally for easy comparison
/// - Empty spaces fill gaps where lines were added/removed
///
/// When sidebar is enabled:
/// - Left panel shows file tree with status markers
/// - Right panel shows the diff for the selected file
/// - `[` key toggles sidebar visibility
/// - `h/l` keys switch focus between sidebar and diff
///
/// # Visual Elements
///
/// - Green background for added lines (+)
/// - Red background for removed lines (-)
/// - Gray header bars for hunk information
/// - Line numbers on each side
///
/// # Fields
///
/// * `file_path` - Optional path to the file being diffed (for single-file mode)
/// * `hunks` - Collection of diff hunks to display (for single-file mode)
/// * `config` - Display configuration (colors, style, sidebar options)
/// * `scroll_offset` - Current vertical scroll position
/// * `file_tree` - Internal file tree widget for sidebar
/// * `file_diffs` - Map of file paths to their diff hunks (for multi-file mode)
/// * `show_sidebar` - Whether sidebar is currently visible
/// * `sidebar_split` - Resizable split for sidebar/diff area division with mouse drag support
/// * `sidebar_focused` - Whether sidebar has focus (vs diff view)
#[derive(Debug, Clone)]
pub struct CodeDiff {
    /// Optional path to the file being diffed (single-file mode).
    pub file_path: Option<String>,

    /// The diff hunks to display (single-file mode).
    pub hunks: Vec<DiffHunk>,

    /// Display configuration.
    pub config: DiffConfig,

    /// Current vertical scroll offset.
    pub scroll_offset: usize,

    /// Internal file tree widget for sidebar.
    pub file_tree: DiffFileTree,

    /// Map of file paths to their diff hunks (multi-file mode).
    pub file_diffs: HashMap<String, Vec<DiffHunk>>,

    /// Whether the sidebar is currently visible.
    pub show_sidebar: bool,

    /// Resizable split for sidebar/diff area division with mouse drag support.
    pub sidebar_split: ResizableSplit,

    /// Whether the sidebar has focus (vs diff view).
    pub sidebar_focused: bool,

    /// Application theme for styling.
    pub theme: AppTheme,
}

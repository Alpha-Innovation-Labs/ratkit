//! Code diff widget for displaying side-by-side diffs.
//!
//! This module provides a VS Code-style diff viewer widget for ratatui,
//! supporting side-by-side display of code changes with syntax highlighting.

pub mod foundation;
pub mod widget;

pub use foundation::diff_config::DiffConfig;
pub use foundation::diff_hunk::DiffHunk;
pub use foundation::diff_line::DiffLine;
pub use foundation::enums::{DiffLineKind, DiffStyle};
pub use foundation::helpers::get_git_diff;
pub use widget::CodeDiff;

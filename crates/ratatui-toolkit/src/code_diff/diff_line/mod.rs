//! Diff line representation.
//!
//! Represents a single line in a diff output, including its kind, content,
//! and line numbers from both the old and new versions.
//!
//! # Structure
//!
//! - [`DiffLine`] - The diff line struct
//! - [`constructors`] - Constructor functions (`new`, `context`, `added`, `removed`)
//! - [`methods`] - Instance methods for accessing line properties
//!
//! # Example
//!
//! ```rust
//! use ratatui_toolkit::code_diff::{DiffLine, DiffLineKind};
//!
//! // Create a removed line
//! let line = DiffLine::removed("old content", 5);
//!
//! // Create an added line
//! let line = DiffLine::added("new content", 5);
//!
//! // Create a context line (unchanged)
//! let line = DiffLine::context("unchanged", 5, 5);
//! ```

pub mod constructors;
pub mod methods;

use crate::code_diff::enums::DiffLineKind;

/// Represents a single line in a diff output.
///
/// A `DiffLine` contains the content of the line, its kind (context, added, removed,
/// or hunk header), and the line numbers from both the old and new file versions.
///
/// # Fields
///
/// * `kind` - The type of diff line (context, added, removed, or hunk header)
/// * `content` - The text content of the line
/// * `old_line_num` - Line number in the old version (None for added lines)
/// * `new_line_num` - Line number in the new version (None for removed lines)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffLine {
    /// The type of this diff line.
    pub kind: DiffLineKind,

    /// The text content of the line.
    pub content: String,

    /// Line number in the old/original file.
    /// `None` for added lines since they don't exist in the old version.
    pub old_line_num: Option<usize>,

    /// Line number in the new/modified file.
    /// `None` for removed lines since they don't exist in the new version.
    pub new_line_num: Option<usize>,
}

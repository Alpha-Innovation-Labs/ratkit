//! Diff hunk representation.
//!
//! A hunk is a contiguous section of a diff, representing a group of related changes.
//! Each hunk starts with a header line like `@@ -1,4 +1,5 @@ context` that indicates
//! the line ranges in both the old and new versions.
//!
//! # Structure
//!
//! - [`DiffHunk`] - The diff hunk struct
//! - [`constructors`] - Constructor functions (`new`, `from_header`)
//! - [`methods`] - Instance methods for working with hunks
//!
//! # Example
//!
//! ```rust
//! use ratatui_toolkit::code_diff::{DiffHunk, DiffLine};
//!
//! let mut hunk = DiffHunk::new(1, 4, 1, 5);
//! hunk.add_line(DiffLine::context("unchanged", 1, 1));
//! hunk.add_line(DiffLine::removed("old line", 2));
//! hunk.add_line(DiffLine::added("new line", 2));
//! ```

pub mod constructors;
pub mod methods;

use crate::code_diff::diff_line::DiffLine;

/// Represents a single hunk (section) in a diff.
///
/// A hunk contains the line range information and a collection of lines that
/// make up this section of the diff. The header information (old_start, old_count,
/// new_start, new_count) corresponds to the unified diff format:
/// `@@ -old_start,old_count +new_start,new_count @@ context`
///
/// # Fields
///
/// * `old_start` - Starting line number in the old file
/// * `old_count` - Number of lines from the old file in this hunk
/// * `new_start` - Starting line number in the new file
/// * `new_count` - Number of lines from the new file in this hunk
/// * `context` - Optional context text (e.g., function name) shown in the header
/// * `lines` - The actual diff lines in this hunk
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffHunk {
    /// Starting line number in the old/original file.
    pub old_start: usize,

    /// Number of lines from the old file included in this hunk.
    pub old_count: usize,

    /// Starting line number in the new/modified file.
    pub new_start: usize,

    /// Number of lines from the new file included in this hunk.
    pub new_count: usize,

    /// Optional context text from the hunk header (e.g., function name).
    pub context: Option<String>,

    /// The lines in this hunk, including context, added, and removed lines.
    pub lines: Vec<DiffLine>,
}

//! Code diff widget for displaying side-by-side diffs.
//!
//! This module provides a VS Code-style diff viewer widget for ratatui,
//! supporting side-by-side display of code changes with syntax highlighting.
//!
//! # Features
//!
//! - **Side-by-side layout**: Old version on left, new version on right
//! - **Line number display**: Independent line numbers per side
//! - **Visual markers**: Green for additions, red for deletions
//! - **Hunk headers**: Gray bars showing `@@ -X,Y +A,B @@ context`
//! - **Alignment**: Matching lines align horizontally
//! - **Scrolling**: Support for vertical scrolling through large diffs
//!
//! # Structure
//!
//! - [`widget`] - The primary widget implementation (`CodeDiff`)
//! - [`foundation`] - Core diff types (config, hunks, lines, enums)
//! - [`extensions`] - Optional extensions like the file tree sidebar
//! - [`CodeDiff`] - The main diff widget
//! - [`DiffConfig`] - Display configuration (colors, styles)
//! - [`DiffHunk`] - A section of the diff with related changes
//! - [`DiffLine`] - A single line in a diff
//! - [`DiffLineKind`] - Type of diff line (context, added, removed)
//! - [`DiffStyle`] - Display style (side-by-side, unified, inline)
//!
//! # Example
//!
//! ```rust
//! use ratatui_toolkit::code_diff::{CodeDiff, DiffConfig};
//!
//! // Parse a unified diff
//! let diff_text = r#"
//! --- a/example.rs
//! +++ b/example.rs
//! @@ -1,4 +1,5 @@
//!  fn main() {
//! -    println!("Hello");
//! +    println!("Hello, World!");
//! +    println!("Goodbye!");
//!  }
//! "#;
//!
//! let widget = CodeDiff::from_unified_diff(diff_text)
//!     .with_config(DiffConfig::new().show_line_numbers(true));
//!
//! // Render with ratatui...
//! // widget.render(area, &mut frame.buffer_mut());
//! ```
//!
//! # Building Diffs Programmatically
//!
//! ```rust
//! use ratatui_toolkit::code_diff::{CodeDiff, DiffHunk, DiffLine, DiffConfig};
//!
//! let mut diff = CodeDiff::new()
//!     .with_file_path("src/lib.rs")
//!     .with_config(DiffConfig::new());
//!
//! let mut hunk = DiffHunk::new(1, 2, 1, 3);
//! hunk.add_line(DiffLine::context("fn main() {", 1, 1));
//! hunk.add_line(DiffLine::removed("    old_code();", 2));
//! hunk.add_line(DiffLine::added("    new_code();", 2));
//! hunk.add_line(DiffLine::added("    extra_code();", 3));
//! hunk.add_line(DiffLine::context("}", 3, 4));
//!
//! diff.add_hunk(hunk);
//! ```

// Core modules
pub mod extensions;
pub mod foundation;
pub mod widget;

// Backwards-compatible module re-exports
pub use extensions::file_tree as diff_file_tree;
pub use foundation::diff_config;
pub use foundation::diff_hunk;
pub use foundation::diff_line;
pub use foundation::enums;
pub use foundation::helpers;
pub use widget as code_diff;

// Re-export main types at the module level
pub use foundation::diff_config::DiffConfig;
pub use foundation::diff_hunk::DiffHunk;
pub use foundation::diff_line::DiffLine;
pub use foundation::enums::{DiffLineKind, DiffStyle};
pub use widget::CodeDiff;

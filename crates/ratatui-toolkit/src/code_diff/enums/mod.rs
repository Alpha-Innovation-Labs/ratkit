//! Enumeration types for the code diff module.
//!
//! This module contains simple enums used throughout the diff widget:
//!
//! - [`DiffLineKind`] - The type of a diff line (context, added, removed, hunk header)
//! - [`DiffStyle`] - The display style for the diff (side-by-side, unified, inline)

mod diff_line_kind;
mod diff_style;

pub use diff_line_kind::DiffLineKind;
pub use diff_style::DiffStyle;

//! Diff colors module for CodeDiff widget theming.
//!
//! This module provides [`DiffColors`] which contains all the colors needed
//! for rendering unified diff views with syntax-highlighted additions,
//! removals, and context lines.
//!
//! # Color Categories
//!
//! The diff color scheme includes:
//! - **Line colors**: Colors for added, removed, and context line text
//! - **Background colors**: Background colors for different line types
//! - **Highlight colors**: Emphasized colors for inline changes
//! - **Line number colors**: Colors for the gutter line numbers
//!
//! # Example
//!
//! ```rust
//! use ratatui::style::Color;
//! use ratatui_toolkit::services::theme::DiffColors;
//!
//! let colors = DiffColors::default();
//! // Use colors.added for added line text color
//! ```

use ratatui::style::Color;

/// Colors for rendering unified diff views.
///
/// This struct contains all the colors needed for the [`CodeDiff`](crate::CodeDiff)
/// widget to render diff output with proper syntax highlighting for additions,
/// removals, context lines, and hunk headers.
///
/// # Fields
///
/// The color scheme is organized into logical groups:
///
/// - **Line text colors**: `added`, `removed`, `context`, `hunk_header`
/// - **Highlight colors**: `highlight_added`, `highlight_removed` for inline changes
/// - **Background colors**: `added_bg`, `removed_bg`, `context_bg` for line backgrounds
/// - **Line number colors**: `line_number`, `added_line_number_bg`, `removed_line_number_bg`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffColors {
    /// Color for added line text (e.g., green).
    ///
    /// Used for lines prefixed with `+` in unified diff format.
    pub added: Color,

    /// Color for removed line text (e.g., red).
    ///
    /// Used for lines prefixed with `-` in unified diff format.
    pub removed: Color,

    /// Color for context line text (e.g., gray).
    ///
    /// Used for unchanged lines that provide context around changes.
    pub context: Color,

    /// Color for hunk header text (e.g., cyan).
    ///
    /// Used for lines like `@@ -1,3 +1,4 @@` that mark diff sections.
    pub hunk_header: Color,

    /// Highlight color for added text within a line.
    ///
    /// Used to emphasize specific added characters or words
    /// when showing inline changes.
    pub highlight_added: Color,

    /// Highlight color for removed text within a line.
    ///
    /// Used to emphasize specific removed characters or words
    /// when showing inline changes.
    pub highlight_removed: Color,

    /// Background color for added lines.
    ///
    /// A subtle green tint to distinguish added lines.
    pub added_bg: Color,

    /// Background color for removed lines.
    ///
    /// A subtle red tint to distinguish removed lines.
    pub removed_bg: Color,

    /// Background color for context lines.
    ///
    /// Usually the same as the panel background or slightly different.
    pub context_bg: Color,

    /// Color for line numbers in the gutter.
    ///
    /// Usually a muted color that doesn't distract from the diff content.
    pub line_number: Color,

    /// Background color for line numbers on added lines.
    ///
    /// Slightly tinted to match the added line background.
    pub added_line_number_bg: Color,

    /// Background color for line numbers on removed lines.
    ///
    /// Slightly tinted to match the removed line background.
    pub removed_line_number_bg: Color,
}

mod constructors;
mod traits;

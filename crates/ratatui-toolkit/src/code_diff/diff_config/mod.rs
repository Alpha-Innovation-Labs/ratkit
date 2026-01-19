//! Configuration for the diff widget.
//!
//! Contains styling and display options for the code diff widget.
//!
//! # Structure
//!
//! - [`DiffConfig`] - The configuration struct
//! - [`constructors`] - Constructor functions (`new`, `default`)
//! - [`methods`] - Builder methods for configuration
//!
//! # Example
//!
//! ```rust
//! use ratatui::style::Color;
//! use ratatui_toolkit::code_diff::DiffConfig;
//!
//! let config = DiffConfig::new()
//!     .added_bg(Color::Green)
//!     .removed_bg(Color::Red)
//!     .show_line_numbers(true);
//! ```

pub mod constructors;
pub mod methods;

use ratatui::style::Color;

use crate::code_diff::enums::DiffStyle;

/// Configuration options for the CodeDiff widget.
///
/// Controls the visual appearance and behavior of the diff display,
/// including colors for added/removed lines, line number visibility,
/// context line count, and sidebar options.
///
/// # Fields
///
/// * `added_bg` - Background color for added lines (default: dark green)
/// * `removed_bg` - Background color for removed lines (default: dark red)
/// * `added_fg` - Foreground color for added lines (default: green)
/// * `removed_fg` - Foreground color for removed lines (default: red)
/// * `hunk_header_bg` - Background color for hunk headers (default: dark gray)
/// * `hunk_header_fg` - Foreground color for hunk headers (default: cyan)
/// * `line_number_fg` - Foreground color for line numbers (default: dark gray)
/// * `show_line_numbers` - Whether to display line numbers (default: true)
/// * `context_lines` - Number of context lines to show around changes (default: 3)
/// * `style` - The diff display style (default: SideBySide)
/// * `gutter_width` - Width of the gutter for markers (default: 2)
/// * `line_number_width` - Width of line number columns (default: 4)
/// * `sidebar_enabled` - Whether the sidebar is enabled (default: false)
/// * `sidebar_default_width` - Default sidebar width as percentage (default: 25)
/// * `sidebar_min_width` - Minimum sidebar width as percentage (default: 10)
/// * `sidebar_max_width` - Maximum sidebar width as percentage (default: 50)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffConfig {
    /// Background color for added lines.
    pub added_bg: Color,

    /// Foreground color for added lines.
    pub added_fg: Color,

    /// Background color for removed lines.
    pub removed_bg: Color,

    /// Foreground color for removed lines.
    pub removed_fg: Color,

    /// Background color for hunk header lines.
    pub hunk_header_bg: Color,

    /// Foreground color for hunk header lines.
    pub hunk_header_fg: Color,

    /// Foreground color for line numbers.
    pub line_number_fg: Color,

    /// Whether to show line numbers.
    pub show_line_numbers: bool,

    /// Number of context lines to show around changes.
    pub context_lines: usize,

    /// The diff display style.
    pub style: DiffStyle,

    /// Width of the gutter column (for +/- markers).
    pub gutter_width: u16,

    /// Width of line number columns.
    pub line_number_width: u16,

    /// Whether the sidebar file tree is enabled.
    pub sidebar_enabled: bool,

    /// Default sidebar width as percentage (0-100).
    pub sidebar_default_width: u16,

    /// Minimum sidebar width as percentage (0-100).
    pub sidebar_min_width: u16,

    /// Maximum sidebar width as percentage (0-100).
    pub sidebar_max_width: u16,
}

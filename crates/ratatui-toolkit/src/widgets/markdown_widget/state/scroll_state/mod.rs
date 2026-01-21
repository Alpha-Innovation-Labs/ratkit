//! Scroll state for markdown widget.
//!
//! Manages scroll offset, viewport dimensions, and current line position.
//! This module handles ONLY scrolling - nothing else.

pub mod constructors;
pub mod methods;
pub mod traits;

pub use constructors::*;
pub use methods::*;
pub use traits::*;

/// Scroll state for markdown rendering.
///
/// Manages scroll position, viewport dimensions, and current line for navigation.
#[derive(Debug, Clone)]
pub struct ScrollState {
    /// Current scroll offset (0-indexed, first visible line index).
    pub scroll_offset: usize,
    /// Height of viewport (number of visible lines).
    pub viewport_height: usize,
    /// Total number of lines in document.
    pub total_lines: usize,
    /// Currently selected line (1-indexed, for highlighting).
    pub current_line: usize,
    /// Current filter text (when in filter mode).
    pub filter: Option<String>,
    /// Whether filter mode is currently active.
    pub filter_mode: bool,
}

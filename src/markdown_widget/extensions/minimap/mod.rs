//! Minimap widget for document overview.
//!
//! Provides a compact visual representation of document content using
//! Braille characters to show text density. Useful for navigation and
//! understanding document structure at a glance.
//!
//! # Features
//!
//! - Braille-based text density rendering
//! - Viewport position indicator
//! - Click-to-scroll navigation
//! - Configurable width and appearance

pub mod constructors;
pub mod enums;
mod helpers;
mod methods;
mod traits;

pub use constructors::*;
pub use enums::*;
pub use enums::MinimapConfig;
pub use helpers::*;
pub use methods::*;
pub use traits::*;

/// A minimap widget that shows a compact overview of document content.
///
/// The minimap renders text content using Braille Unicode characters
/// to create a visual density map. Each Braille character can represent
/// multiple lines of source content, providing a birds-eye view.
///
/// # Example
///
/// ```rust,no_run
/// use ratatui_toolkit::markdown_widget::extensions::minimap::Minimap;
///
/// let content = "# Hello\n\nSome text here.\n\n## Section\n\nMore content.";
/// let minimap = Minimap::new(content)
///     .width(10)
///     .viewport(0, 20, 100); // viewing lines 0-20 of 100 total
/// ```
#[derive(Debug)]
pub struct Minimap<'a> {
    /// The source content to render.
    pub(crate) content: &'a str,
    /// Width of the minimap in characters.
    pub(crate) width: u16,
    /// Current viewport start line.
    pub(crate) viewport_start: usize,
    /// Current viewport end line.
    pub(crate) viewport_end: usize,
    /// Total number of lines in content.
    pub(crate) total_lines: usize,
    /// Configuration for appearance.
    pub(crate) config: MinimapConfig,
}

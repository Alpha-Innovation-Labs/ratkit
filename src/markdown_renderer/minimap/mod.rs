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

mod constructors;
mod methods;
mod traits;

pub use constructors::*;
pub use methods::*;

use ratatui::style::{Color, Style};

/// Configuration for minimap appearance.
#[derive(Debug, Clone)]
pub struct MinimapConfig {
    /// Width of the minimap in characters.
    pub width: u16,
    /// Height of the minimap in characters (for corner overlay mode).
    pub height: u16,
    /// Style for the minimap text (Braille characters).
    pub text_style: Style,
    /// Style for the viewport indicator.
    pub viewport_style: Style,
    /// Style for the minimap border/background.
    pub background_style: Style,
    /// Whether to show line density or just structure.
    pub show_density: bool,
}

/// A minimap widget that shows a compact overview of document content.
///
/// The minimap renders text content using Braille Unicode characters
/// to create a visual density map. Each Braille character can represent
/// multiple lines of source content, providing a birds-eye view.
///
/// # Example
///
/// ```rust,no_run
/// use ratatui_toolkit::markdown_renderer::minimap::Minimap;
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

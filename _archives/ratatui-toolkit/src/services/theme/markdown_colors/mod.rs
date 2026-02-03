//! Markdown colors module for MarkdownWidget theming.
//!
//! This module provides [`MarkdownColors`] which contains all the colors needed
//! for rendering markdown content with proper syntax highlighting for headings,
//! links, code blocks, emphasis, and other markdown elements.
//!
//! # Color Categories
//!
//! The markdown color scheme includes:
//! - **Text colors**: Base text and heading colors
//! - **Link colors**: URL and link text colors
//! - **Code colors**: Inline code and code block colors
//! - **Emphasis colors**: Bold, italic, and quote colors
//! - **List colors**: Bullet and enumeration colors
//!
//! # Example
//!
//! ```rust
//! use ratatui::style::Color;
//! use ratatui_toolkit::services::theme::MarkdownColors;
//!
//! let colors = MarkdownColors::default();
//! // Use colors.heading for heading text color
//! ```

use ratatui::style::Color;

/// Colors for rendering markdown content.
///
/// This struct contains all the colors needed for the [`MarkdownWidget`](crate::MarkdownWidget)
/// to render markdown with proper syntax highlighting for all markdown elements.
///
/// # Fields
///
/// The color scheme covers all common markdown elements:
///
/// - **Text**: `text`, `heading`
/// - **Links**: `link`, `link_text`
/// - **Code**: `code`, `code_block`
/// - **Emphasis**: `emph` (italic), `strong` (bold)
/// - **Structure**: `block_quote`, `horizontal_rule`
/// - **Lists**: `list_item`, `list_enumeration`
/// - **Images**: `image`, `image_text`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownColors {
    /// Color for regular paragraph text.
    ///
    /// The base color for all markdown text content.
    pub text: Color,

    /// Color for heading text (h1-h6).
    ///
    /// Typically a prominent color like the theme's primary color.
    pub heading: Color,

    /// Color for link URLs.
    ///
    /// Used for the actual URL portion of links.
    pub link: Color,

    /// Color for link display text.
    ///
    /// Used for the clickable text of a link `[text](url)`.
    pub link_text: Color,

    /// Color for inline code.
    ///
    /// Used for `code` spans within paragraphs.
    pub code: Color,

    /// Color for block quote text.
    ///
    /// Used for `> quoted text` blocks.
    pub block_quote: Color,

    /// Color for emphasized (italic) text.
    ///
    /// Used for `*italic*` or `_italic_` text.
    pub emph: Color,

    /// Color for strong (bold) text.
    ///
    /// Used for `**bold**` or `__bold__` text.
    pub strong: Color,

    /// Color for horizontal rules.
    ///
    /// Used for `---` or `***` separators.
    pub horizontal_rule: Color,

    /// Color for unordered list bullets.
    ///
    /// Used for `-`, `*`, or `+` list markers.
    pub list_item: Color,

    /// Color for ordered list numbers.
    ///
    /// Used for `1.`, `2.`, etc. list markers.
    pub list_enumeration: Color,

    /// Color for image markers.
    ///
    /// Used for the `!` prefix in `![alt](url)`.
    pub image: Color,

    /// Color for image alt text.
    ///
    /// Used for the alt text in `![alt text](url)`.
    pub image_text: Color,

    /// Color for code block text.
    ///
    /// Used for fenced code blocks.
    pub code_block: Color,
}

mod constructors;
mod traits;

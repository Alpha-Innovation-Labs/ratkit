//! Theme definitions for markdown rendering.
//!
//! Supports loading themes from JSON files with named colors and light/dark mode.
//!
//! This module provides:
//! - [`ColorPalette`] - Color palette mapping named colors to RGB values
//! - [`MarkdownTheme`] - Markdown theme configuration with support for light/dark modes
//! - [`MarkdownStyle`] - Configuration for markdown rendering styles
//! - [`ColorMapping`] - Color mapping for light/dark modes
//! - [`ThemeVariant`] - Theme variant selection (Dark, Light, Auto)
//! - [`SyntaxThemeVariant`] - Syntax highlighting theme variant (Dark, Light)
//! - [`SyntaxHighlighter`] - Syntax highlighting for code blocks
//! - [`palettes`] - Predefined color palettes for common themes
//!
//! # Example
//!
//! ```rust,ignore
//! use ratatui_toolkit::markdown_widget::extensions::theme::{
//!     ColorPalette, MarkdownTheme, ThemeVariant, palettes,
//! };
//!
//! // Use a predefined dark palette
//! let palette = palettes::dark_default();
//!
//! // Get a color by name
//! let blue = palette.get_or_default("blue");
//! ```

pub mod color_mapping;
pub mod color_palette;
pub mod enums;
mod get_effective_theme_variant;
mod load_theme_from_json;
pub mod markdown_style;
pub mod markdown_theme;
pub mod palettes;
pub mod syntax_highlighter;

pub use color_mapping::ColorMapping;
pub use color_palette::ColorPalette;
pub use enums::SyntaxThemeVariant;
pub use enums::ThemeVariant;
pub use get_effective_theme_variant::get_effective_theme_variant;
pub use load_theme_from_json::load_theme_from_json;
pub use markdown_style::MarkdownStyle;
pub use markdown_theme::MarkdownTheme;
pub use syntax_highlighter::SyntaxHighlighter;

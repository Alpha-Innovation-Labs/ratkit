//! Theme definitions for markdown rendering.
//!
//! Supports loading themes from JSON files with named colors and light/dark mode.
//!
//! This module provides:
//! - [`ColorPalette`] - Color palette mapping named colors to RGB values
//! - [`MarkdownTheme`] - Markdown theme configuration with support for light/dark modes
//! - [`ColorMapping`] - Color mapping for light/dark modes
//! - [`ThemeVariant`] - Theme variant selection (Dark, Light, Auto)
//! - [`palettes`] - Predefined color palettes for common themes
//!
//! # Example
//!
//! ```rust,ignore
//! use ratatui_toolkit::markdown_renderer::theme::{
//!     ColorPalette, MarkdownTheme, ThemeVariant, palettes,
//! };
//!
//! // Use a predefined dark palette
//! let palette = palettes::dark_default();
//!
//! // Get a color by name
//! let blue = palette.get_or_default("blue");
//! ```

mod color_mapping;
mod color_palette;
mod get_effective_theme_variant;
mod load_theme_from_json;
mod markdown_theme;
pub mod palettes;
mod theme_variant;

pub use color_mapping::ColorMapping;
pub use color_palette::ColorPalette;
pub use get_effective_theme_variant::get_effective_theme_variant;
pub use load_theme_from_json::load_theme_from_json;
pub use markdown_theme::MarkdownTheme;
pub use theme_variant::ThemeVariant;

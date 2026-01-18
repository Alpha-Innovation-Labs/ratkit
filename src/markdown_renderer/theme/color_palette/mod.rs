//! Color palette for mapping named colors to RGB values.
//!
//! The [`ColorPalette`] struct provides a way to define and lookup colors by name,
//! which is useful for theming systems where colors are referenced by semantic names
//! rather than direct RGB values.

mod constructors;
mod methods;

use ratatui::style::Color;
use std::collections::HashMap;

/// Color palette mapping named colors to RGB values.
///
/// This struct wraps a `HashMap` to provide named color lookups. Colors can be
/// added with semantic names (like "primary", "error", "success") and retrieved
/// by those names, with optional fallback to default colors.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_renderer::theme::ColorPalette;
/// use ratatui::style::Color;
///
/// let mut palette = ColorPalette::new();
/// palette.add_color("primary", Color::Rgb(97, 175, 239));
///
/// let color = palette.get_or_default("primary");
/// ```
#[derive(Debug, Clone, Default)]
pub struct ColorPalette(pub(crate) HashMap<String, Color>);

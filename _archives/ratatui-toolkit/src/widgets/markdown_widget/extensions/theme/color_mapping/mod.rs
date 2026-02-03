//! Color mapping for light/dark mode support.
//!
//! The [`ColorMapping`] struct provides a way to define different colors for
//! light and dark color schemes, allowing themes to adapt to the user's
//! terminal or application theme.

mod methods;

#[cfg(feature = "markdown")]
use serde::Deserialize;

/// Color mapping for light/dark modes.
///
/// This struct holds color names (not actual colors) that map to entries
/// in a [`ColorPalette`]. When resolving the actual color, the appropriate
/// variant (dark or light) is selected based on the current color scheme.
///
/// # Fields
///
/// * `dark` - Color name to use in dark mode
/// * `light` - Color name to use in light mode
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::theme::{ColorMapping, ColorPalette, palettes};
///
/// // ColorMapping references color names, not actual RGB values
/// let mapping = ColorMapping {
///     dark: Some("blue".to_string()),
///     light: Some("oceanBlue".to_string()),
/// };
///
/// let palette = palettes::dark_default();
/// let color = mapping.get_color(&palette, true); // true = dark mode
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct ColorMapping {
    /// Color name to use in dark mode.
    #[serde(default)]
    pub dark: Option<String>,

    /// Color name to use in light mode.
    #[serde(default)]
    pub light: Option<String>,
}

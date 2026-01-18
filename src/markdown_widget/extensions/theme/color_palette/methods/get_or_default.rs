//! Method to get a color by name with a default fallback.

use crate::markdown_widget::extensions::theme::color_palette::ColorPalette;
use ratatui::style::Color;

impl ColorPalette {
    /// Get a color by name with default fallback.
    ///
    /// If the color is not found in the palette, returns `Color::White` as the default.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the color to retrieve
    ///
    /// # Returns
    ///
    /// The [`Color`] associated with the name, or `Color::White` if not found.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::markdown_widget::extensions::theme::ColorPalette;
    /// use ratatui::style::Color;
    ///
    /// let palette = ColorPalette::new();
    /// // Returns Color::White since "nonexistent" is not in the palette
    /// let color = palette.get_or_default("nonexistent");
    /// ```
    pub fn get_or_default(&self, name: &str) -> Color {
        self.get(name).unwrap_or(Color::White)
    }
}

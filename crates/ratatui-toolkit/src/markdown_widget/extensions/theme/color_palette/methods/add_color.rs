//! Method to add a color to the palette.

use crate::markdown_widget::extensions::theme::color_palette::ColorPalette;
use ratatui::style::Color;

impl ColorPalette {
    /// Add a color to the palette.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to associate with the color (e.g., "primary", "error")
    /// * `color` - The ratatui [`Color`] value
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::markdown_widget::extensions::theme::ColorPalette;
    /// use ratatui::style::Color;
    ///
    /// let mut palette = ColorPalette::new();
    /// palette.add_color("blue", Color::Rgb(97, 175, 239));
    /// ```
    pub fn add_color(&mut self, name: &str, color: Color) {
        self.0.insert(name.to_string(), color);
    }
}

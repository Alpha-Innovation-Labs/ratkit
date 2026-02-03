//! Method to get a color by name from the palette.

use crate::widgets::markdown_widget::extensions::theme::color_palette::ColorPalette;
use ratatui::style::Color;

impl ColorPalette {
    /// Get a color by name, with fallback.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the color to retrieve
    ///
    /// # Returns
    ///
    /// `Some(Color)` if the color exists in the palette, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::markdown_widget::extensions::theme::ColorPalette;
    /// use ratatui::style::Color;
    ///
    /// let mut palette = ColorPalette::new();
    /// palette.add_color("blue", Color::Rgb(97, 175, 239));
    ///
    /// assert!(palette.get("blue").is_some());
    /// assert!(palette.get("nonexistent").is_none());
    /// ```
    pub fn get(&self, name: &str) -> Option<Color> {
        self.0.get(name).copied()
    }
}

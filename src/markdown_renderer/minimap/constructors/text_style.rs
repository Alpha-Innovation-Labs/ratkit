//! Text style builder method for Minimap.

use ratatui::style::Style;

use super::super::Minimap;

impl<'a> Minimap<'a> {
    /// Set the style for minimap text (Braille characters).
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to minimap text
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn text_style(mut self, style: Style) -> Self {
        self.config.text_style = style;
        self
    }
}

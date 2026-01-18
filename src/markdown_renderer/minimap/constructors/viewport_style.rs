//! Viewport style builder method for Minimap.

use ratatui::style::Style;

use super::super::Minimap;

impl<'a> Minimap<'a> {
    /// Set the style for the viewport indicator.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to the viewport region
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn viewport_style(mut self, style: Style) -> Self {
        self.config.viewport_style = style;
        self
    }
}

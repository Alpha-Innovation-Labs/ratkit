//! Method to get the effective color for a color scheme.

use crate::widgets::markdown_widget::extensions::theme::color_mapping::ColorMapping;
use crate::widgets::markdown_widget::extensions::theme::ColorPalette;
use ratatui::style::Color;

impl ColorMapping {
    /// Get color for the specified color scheme.
    ///
    /// Resolves the appropriate color name (dark or light) based on the
    /// `is_dark` parameter, then looks up that color in the provided palette.
    ///
    /// If the preferred variant (dark/light) is not set, falls back to the other variant.
    /// If neither variant is set, returns `Color::White`.
    ///
    /// # Arguments
    ///
    /// * `palette` - The [`ColorPalette`] to look up color names in
    /// * `is_dark` - Whether to use dark mode colors (`true`) or light mode colors (`false`)
    ///
    /// # Returns
    ///
    /// The resolved [`Color`] for the current color scheme.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::markdown_widget::extensions::theme::{ColorMapping, palettes};
    ///
    /// let mapping = ColorMapping {
    ///     dark: Some("blue".to_string()),
    ///     light: Some("oceanBlue".to_string()),
    /// };
    ///
    /// let palette = palettes::dark_default();
    ///
    /// // Get dark mode color
    /// let dark_color = mapping.get_color(&palette, true);
    ///
    /// // Get light mode color
    /// let light_color = mapping.get_color(&palette, false);
    /// ```
    pub fn get_color(&self, palette: &ColorPalette, is_dark: bool) -> Color {
        let key = if is_dark {
            self.dark.as_ref().or(self.light.as_ref())
        } else {
            self.light.as_ref().or(self.dark.as_ref())
        };
        key.map(|s| palette.get_or_default(s))
            .unwrap_or(Color::White)
    }
}

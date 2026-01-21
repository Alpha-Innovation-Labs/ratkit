//! Constructor for creating a new empty [`ColorPalette`].

use crate::widgets::markdown_widget::extensions::theme::color_palette::ColorPalette;
use std::collections::HashMap;

impl ColorPalette {
    /// Create a new empty palette.
    ///
    /// # Returns
    ///
    /// A new [`ColorPalette`] with no colors defined.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::markdown_widget::extensions::theme::ColorPalette;
    ///
    /// let palette = ColorPalette::new();
    /// assert!(palette.get("any_color").is_none());
    /// ```
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

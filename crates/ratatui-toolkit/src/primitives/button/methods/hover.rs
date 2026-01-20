use ratatui::style::Style;

use crate::primitives::button::Button;

impl Button {
    /// Returns the hover style.
    ///
    /// # Returns
    ///
    /// The style applied when the button is hovered
    #[inline]
    pub fn hover(&self) -> Style {
        self.hover_style
    }
}

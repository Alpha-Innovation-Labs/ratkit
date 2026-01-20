use ratatui::style::Style;

use crate::primitives::button::Button;

impl Button {
    /// Returns the normal (non-hovered) style.
    ///
    /// # Returns
    ///
    /// The style applied when the button is not hovered
    #[inline]
    pub fn normal(&self) -> Style {
        self.normal_style
    }
}

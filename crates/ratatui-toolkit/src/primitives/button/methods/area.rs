use ratatui::layout::Rect;

use crate::primitives::button::Button;

impl Button {
    /// Returns the area where the button was rendered, if rendered.
    ///
    /// # Returns
    ///
    /// `Some(area)` if the button has been rendered, `None` otherwise
    #[inline]
    pub fn area(&self) -> Option<Rect> {
        self.area
    }
}

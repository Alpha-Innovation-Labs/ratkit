use ratatui::layout::Rect;

use crate::primitives::button::Button;

impl Button {
    /// Sets the button's area (used for click detection).
    ///
    /// # Arguments
    ///
    /// * `area` - The rectangular area where the button is rendered
    #[inline]
    pub fn set_area(&mut self, area: Rect) {
        self.area = Some(area);
    }
}

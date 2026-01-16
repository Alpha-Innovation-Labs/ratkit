use crate::button::Button;

impl Button {
    /// Checks if a click at the given coordinates is within the button's area.
    ///
    /// # Arguments
    ///
    /// * `column` - The column (x) coordinate
    /// * `row` - The row (y) coordinate
    ///
    /// # Returns
    ///
    /// `true` if the coordinates are within the button's area, `false` otherwise
    ///
    /// # Note
    ///
    /// Returns `false` if the button has not been rendered yet (no area set)
    pub fn is_clicked(&self, column: u16, row: u16) -> bool {
        if let Some(area) = self.area {
            column >= area.x
                && column < area.x + area.width
                && row >= area.y
                && row < area.y + area.height
        } else {
            false
        }
    }
}

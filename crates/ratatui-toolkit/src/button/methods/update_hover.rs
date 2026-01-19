use crate::button::Button;

impl Button {
    /// Updates the button's hover state based on cursor position.
    ///
    /// # Arguments
    ///
    /// * `column` - The column (x) coordinate of the cursor
    /// * `row` - The row (y) coordinate of the cursor
    pub fn update_hover(&mut self, column: u16, row: u16) {
        self.hovered = self.is_clicked(column, row);
    }
}

use crate::primitives::button::Button;

impl Button {
    /// Updates the button's hover state based on cursor position.
    ///
    /// # Arguments
    ///
    /// * `column` - The column (x) coordinate of the cursor
    /// * `row` - The row (y) coordinate of the cursor
    ///
    /// # Returns
    ///
    /// Nothing (updates internal hover state)
    pub fn update_hover(&mut self, column: u16, row: u16) {
        self.hovered = self.is_clicked(column, row);
    }
}

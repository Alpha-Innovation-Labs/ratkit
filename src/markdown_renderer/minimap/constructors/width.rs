//! Width builder method for Minimap.

use super::super::Minimap;

impl<'a> Minimap<'a> {
    /// Set the width of the minimap in characters.
    ///
    /// # Arguments
    ///
    /// * `width` - Width in terminal columns
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self.config.width = width;
        self
    }
}

use crate::primitives::button::Button;

impl Button {
    /// Returns the button's text content.
    ///
    /// # Returns
    ///
    /// A string slice containing the button's text
    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }
}

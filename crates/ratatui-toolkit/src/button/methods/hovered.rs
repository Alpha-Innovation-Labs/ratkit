use crate::button::Button;

impl Button {
    /// Returns whether the button is currently hovered.
    ///
    /// # Returns
    ///
    /// `true` if the button is being hovered, `false` otherwise
    #[inline]
    pub fn hovered(&self) -> bool {
        self.hovered
    }
}

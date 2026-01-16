//! Trait implementations for the Button widget.
//!
//! This module contains standard library trait implementations for the `Button` type.

use crate::button::Button;

impl Default for Button {
    /// Returns a default button with text "Button".
    ///
    /// # Returns
    ///
    /// A new `Button` instance with default styling and text "Button"
    fn default() -> Self {
        Self::new("Button")
    }
}

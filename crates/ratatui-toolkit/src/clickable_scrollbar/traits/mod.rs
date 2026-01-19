//! Trait implementations for [`ClickableScrollbar`] and [`ClickableScrollbarState`].
//!
//! This module contains standard library and custom trait implementations.

use crate::clickable_scrollbar::ClickableScrollbarState;

impl Default for ClickableScrollbarState {
    fn default() -> Self {
        Self::new()
    }
}

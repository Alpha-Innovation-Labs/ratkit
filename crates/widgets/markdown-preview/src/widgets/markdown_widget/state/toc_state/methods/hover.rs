//! Hover methods for TocState.

use crate::widgets::markdown_widget::state::toc_state::TocState;

impl TocState {
    /// Check if the TOC is currently hovered.
    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    /// Set the hover state of the TOC.
    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
        if !hovered {
            self.hovered_entry = None;
        }
    }

    /// Get the currently hovered entry index, if any.
    pub fn hovered_entry(&self) -> Option<usize> {
        self.hovered_entry
    }

    /// Set the hovered entry index.
    pub fn set_hovered_entry(&mut self, index: Option<usize>) {
        self.hovered_entry = index;
    }

    /// Check if a specific entry is hovered.
    pub fn is_entry_hovered(&self, index: usize) -> bool {
        self.hovered_entry == Some(index)
    }
}

//! Entry management methods for TocState.

use crate::markdown_widget::state::toc_state::enums::TocEntry;
use crate::markdown_widget::state::toc_state::TocState;

impl TocState {
    /// Get all TOC entries.
    pub fn entries(&self) -> &[TocEntry] {
        &self.entries
    }

    /// Set the TOC entries.
    pub fn set_entries(&mut self, entries: Vec<TocEntry>) {
        self.entries = entries;
        // Reset scroll if entries change
        if self.scroll_offset >= self.entries.len() {
            self.scroll_offset = 0;
        }
    }

    /// Clear all TOC entries.
    pub fn clear_entries(&mut self) {
        self.entries.clear();
        self.scroll_offset = 0;
        self.hovered_entry = None;
    }

    /// Get the number of entries.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Get an entry by index.
    pub fn get_entry(&self, index: usize) -> Option<&TocEntry> {
        self.entries.get(index)
    }

    /// Check if the TOC has any entries.
    pub fn has_entries(&self) -> bool {
        !self.entries.is_empty()
    }
}

//! Entry accessor methods for TOC widget.

use super::super::Toc;
use crate::widgets::markdown_widget::state::toc_state::TocEntry;

impl<'a> Toc<'a> {
    /// Get the number of entries in the TOC.
    ///
    /// Delegates to the underlying TocState.
    pub fn entry_count(&self) -> usize {
        self.toc_state.entry_count()
    }

    /// Get all entries.
    ///
    /// Delegates to the underlying TocState.
    pub fn entries(&self) -> &[TocEntry] {
        self.toc_state.entries()
    }

    /// Get the target line number for a clicked entry.
    ///
    /// # Arguments
    ///
    /// * `entry_index` - The index of the clicked entry.
    ///
    /// # Returns
    ///
    /// The line number to scroll to, or None if the index is invalid.
    pub fn click_to_line(&self, entry_index: usize) -> Option<usize> {
        self.toc_state.get_entry(entry_index).map(|e| e.line_number)
    }

    /// Get the entry at a given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The entry index.
    ///
    /// # Returns
    ///
    /// The entry, or None if the index is invalid.
    pub fn get_entry(&self, index: usize) -> Option<&TocEntry> {
        self.toc_state.get_entry(index)
    }
}

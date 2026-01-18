//! Click-to-line conversion for TOC navigation.

use super::super::Toc;

impl<'a> Toc<'a> {
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
        self.entries.get(entry_index).map(|e| e.line_number)
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
    pub fn get_entry(&self, index: usize) -> Option<&super::super::TocEntry> {
        self.entries.get(index)
    }

    /// Get the number of entries in the TOC.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Get all entries.
    pub fn entries(&self) -> &[super::super::TocEntry] {
        &self.entries
    }
}

//! Extract TOC entries from markdown content.

use crate::widgets::markdown_widget::state::toc_state::enums::TocEntry;
use crate::widgets::markdown_widget::state::toc_state::TocState;

impl TocState {
    /// Create a TocState with entries extracted from markdown content.
    ///
    /// This scans the content for headings and populates the entries.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to scan.
    ///
    /// # Returns
    ///
    /// A new TocState with extracted entries.
    pub fn from_content(content: &str) -> Self {
        let entries = Self::extract_headings(content);
        Self {
            entries,
            ..Default::default()
        }
    }

    /// Update entries from markdown content.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to scan.
    pub fn update_from_content(&mut self, content: &str) {
        self.entries = Self::extract_headings(content);
        // Reset scroll if needed
        if self.scroll_offset >= self.entries.len() {
            self.scroll_offset = 0;
        }
    }

    /// Extract headings from markdown content.
    fn extract_headings(content: &str) -> Vec<TocEntry> {
        let mut entries = Vec::new();
        let mut in_code_block = false;
        let mut section_id = 0usize;

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Track code blocks
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }

            if in_code_block {
                continue;
            }

            // Check for headings
            if trimmed.starts_with('#') {
                let level = trimmed.chars().take_while(|c| *c == '#').count() as u8;
                if level <= 6 {
                    let text = trimmed[level as usize..].trim().to_string();
                    if !text.is_empty() {
                        entries.push(TocEntry {
                            text,
                            level,
                            line_number: line_num + 1, // 1-indexed
                            section_id: section_id.to_string(),
                        });
                        section_id += 1;
                    }
                }
            }
        }

        entries
    }
}

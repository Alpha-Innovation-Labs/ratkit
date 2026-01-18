//! Constructor for Toc widget.

use super::super::{Toc, TocConfig, TocEntry};

impl<'a> Toc<'a> {
    /// Create a new TOC widget from markdown content.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to extract headings from.
    ///
    /// # Returns
    ///
    /// A new `Toc` instance with extracted headings.
    pub fn new(content: &'a str) -> Self {
        let entries = Self::extract_headings(content);
        Self {
            content,
            entries,
            active_index: None,
            hovered_index: None,
            expanded: false,
            config: TocConfig::default(),
            scroll_offset: 0,
            total_lines: content.lines().count(),
            toc_scroll_offset: 0,
        }
    }

    /// Set whether the TOC is expanded.
    ///
    /// # Arguments
    ///
    /// * `expanded` - True for expanded mode (full text), false for compact mode (lines).
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set the TOC configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The TOC configuration.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn config(mut self, config: TocConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the current viewport information.
    ///
    /// # Arguments
    ///
    /// * `scroll_offset` - Current scroll offset.
    /// * `viewport_height` - Height of the visible viewport.
    /// * `total_lines` - Total number of lines in the document.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn viewport(
        mut self,
        scroll_offset: usize,
        viewport_height: usize,
        total_lines: usize,
    ) -> Self {
        self.scroll_offset = scroll_offset;
        self.total_lines = total_lines;
        self.active_index = self.find_active_heading(scroll_offset, viewport_height);
        self
    }

    /// Set the hovered item index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the hovered heading, or None.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn hovered(mut self, index: Option<usize>) -> Self {
        self.hovered_index = index;
        self
    }

    /// Set the TOC scroll offset (for scrolling within the TOC list).
    ///
    /// # Arguments
    ///
    /// * `offset` - The scroll offset for the TOC list.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_scroll(mut self, offset: usize) -> Self {
        self.toc_scroll_offset = offset;
        self
    }

    /// Find the heading that is currently active based on scroll position.
    ///
    /// Always returns Some if there are any entries - finds the heading
    /// that the user is currently "in" based on scroll position.
    fn find_active_heading(&self, scroll_offset: usize, _viewport_height: usize) -> Option<usize> {
        if self.entries.is_empty() {
            return None;
        }

        // Find the last heading whose line_number is <= scroll_offset + a small buffer
        // This represents "which section are we currently in"
        let mut active = 0; // Default to first heading

        for (i, entry) in self.entries.iter().enumerate() {
            // If this heading is at or before current scroll position, we're in this section
            if entry.line_number <= scroll_offset + 2 {
                active = i;
            } else {
                // Past the scroll position, stop looking
                break;
            }
        }

        Some(active)
    }

    /// Extract headings from markdown content.
    fn extract_headings(content: &str) -> Vec<TocEntry> {
        let mut entries = Vec::new();
        let mut in_code_block = false;
        let mut section_id = 0;

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
                            section_id,
                        });
                        section_id += 1;
                    }
                }
            }
        }

        entries
    }
}

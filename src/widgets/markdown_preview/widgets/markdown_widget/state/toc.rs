//! Table of Contents state for markdown widget.
//!
//! Single source of truth for TOC state including scroll offset, hover state, and entries.

/// State for the Table of Contents sidebar.
///
/// Manages scroll position, hover state, and TOC entries.
#[derive(Debug, Clone, Default)]
pub struct TocState {
    /// Current scroll offset within the TOC.
    pub scroll_offset: usize,
    /// Index of the currently hovered entry, if any.
    pub hovered_entry: Option<usize>,
    /// Whether the TOC itself is hovered.
    pub hovered: bool,
    /// List of TOC entries extracted from the document.
    pub entries: Vec<TocEntry>,
}

/// Constructor for TocState.

impl TocState {
    /// Create a new empty TocState.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Table of Contents entry.

/// A single entry in the Table of Contents.
#[derive(Debug, Clone, Default)]
pub struct TocEntry {
    /// The text content of the heading.
    pub text: String,
    /// The heading level (1-6).
    pub level: u8,
    /// The line number in the source document.
    pub line_number: usize,
    /// A unique identifier for the section.
    pub section_id: String,
}

/// Entry management methods for TocState.

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

/// Extract TOC entries from markdown content.

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

/// Hover methods for TocState.

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

/// Scroll methods for TocState.

impl TocState {
    /// Get the current scroll offset.
    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    /// Set the scroll offset.
    pub fn set_scroll_offset(&mut self, offset: usize) {
        self.scroll_offset = offset;
    }

    /// Scroll up by a given amount, clamping at 0.
    pub fn scroll_up(&mut self, amount: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
    }

    /// Scroll down by a given amount, clamping at max entries.
    pub fn scroll_down(&mut self, amount: usize) {
        let max_offset = self.entries.len().saturating_sub(1);
        self.scroll_offset = (self.scroll_offset + amount).min(max_offset);
    }

    /// Scroll to the top.
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    /// Scroll to the bottom.
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.entries.len().saturating_sub(1);
    }
}

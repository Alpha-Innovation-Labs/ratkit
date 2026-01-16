//! Scroll manager for markdown rendering
//!
//! Provides utilities to manage scroll offset, handle mouse scroll events,
//! and track collapse/expand state for markdown sections.

use ratatui::layout::Rect;
use std::collections::HashMap;

/// Manages scroll state for markdown rendering
#[derive(Debug, Clone)]
pub struct MarkdownScrollManager {
    /// Current scroll offset (0-indexed, first visible line index)
    pub scroll_offset: usize,
    /// Height of viewport (number of visible lines)
    pub viewport_height: usize,
    /// Total number of lines in document
    pub total_lines: usize,
    /// Currently selected line (1-indexed, for highlighting)
    pub current_line: usize,
    /// Section collapse state: section_id -> is_collapsed
    pub collapsed_sections: HashMap<usize, bool>,
    /// Expandable content state: content_id -> { collapsed, max_lines }
    pub expandable_content: HashMap<String, ExpandableState>,
    /// Default max lines for expandable content
    pub default_max_lines: usize,
}

impl Default for MarkdownScrollManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownScrollManager {
    /// Create a new scroll manager
    pub fn new() -> Self {
        Self {
            scroll_offset: 0,
            viewport_height: 20,
            total_lines: 0,
            current_line: 1,
            collapsed_sections: HashMap::new(),
            expandable_content: HashMap::new(),
            default_max_lines: 3,
        }
    }

    /// Update viewport dimensions
    pub fn update_viewport(&mut self, area: Rect) {
        self.viewport_height = area.height as usize;
    }

    /// Update total line count
    pub fn update_total_lines(&mut self, total: usize) {
        self.total_lines = total.max(1);
    }

    /// Scroll up by given number of lines
    pub fn scroll_up(&mut self, amount: usize) {
        let max_offset = self.max_scroll_offset();
        self.scroll_offset = self.scroll_offset.saturating_sub(amount).min(max_offset);
    }

    /// Scroll down by given number of lines
    pub fn scroll_down(&mut self, amount: usize) {
        let max_offset = self.max_scroll_offset();
        self.scroll_offset = (self.scroll_offset + amount).min(max_offset);
    }

    /// Move current line up (for keyboard navigation)
    pub fn line_up(&mut self) {
        if self.current_line > 1 {
            self.current_line -= 1;
        }
        self.adjust_scroll_for_current_line();
    }

    /// Move current line down (for keyboard navigation)
    pub fn line_down(&mut self) {
        if self.current_line < self.total_lines {
            self.current_line += 1;
        }
        self.adjust_scroll_for_current_line();
    }

    /// Move to top of document
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
        self.current_line = 1;
    }

    /// Move to bottom of document
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.max_scroll_offset();
        self.current_line = self.total_lines;
    }

    /// Set current line and adjust scroll to keep it visible
    pub fn set_current_line(&mut self, line: usize) {
        self.current_line = line.clamp(1, self.total_lines);
        self.adjust_scroll_for_current_line();
    }

    /// Adjust scroll offset to ensure current_line is visible
    pub fn adjust_scroll_for_current_line(&mut self) {
        if self.current_line < self.scroll_offset + 1 {
            self.scroll_offset = self.current_line.saturating_sub(1);
        }
        if self.viewport_height > 0 && self.current_line > self.scroll_offset + self.viewport_height
        {
            self.scroll_offset = self.current_line.saturating_sub(self.viewport_height);
        }
    }

    /// Check if current line is visible in the viewport
    pub fn is_current_line_visible(&self) -> bool {
        let first_visible = self.scroll_offset + 1;
        let last_visible = self.scroll_offset + self.viewport_height;
        self.current_line >= first_visible && self.current_line <= last_visible
    }

    /// Get the maximum valid scroll offset
    pub fn max_scroll_offset(&self) -> usize {
        self.total_lines.saturating_sub(self.viewport_height)
    }

    /// Get range of currently visible lines (1-indexed, inclusive)
    pub fn visible_range(&self) -> (usize, usize) {
        let start = self.scroll_offset + 1;
        let end = (self.scroll_offset + self.viewport_height).min(self.total_lines);
        (start, end)
    }

    /// Calculate percentage scrolled (0.0 to 1.0)
    pub fn scroll_percentage(&self) -> f64 {
        let max_offset = self.max_scroll_offset();
        if max_offset == 0 {
            0.0
        } else {
            self.scroll_offset as f64 / max_offset as f64
        }
    }

    // ========== Collapse/Expand Section Methods ==========

    /// Toggle the collapse state of a section
    pub fn toggle_section_collapse(&mut self, section_id: usize) {
        let is_collapsed = self.collapsed_sections.entry(section_id).or_insert(false);
        *is_collapsed = !*is_collapsed;
    }

    /// Set the collapse state of a section
    pub fn set_section_collapsed(&mut self, section_id: usize, collapsed: bool) {
        self.collapsed_sections.insert(section_id, collapsed);
    }

    /// Check if a section is collapsed
    pub fn is_section_collapsed(&self, section_id: usize) -> bool {
        self.collapsed_sections
            .get(&section_id)
            .copied()
            .unwrap_or(false)
    }

    /// Expand a section
    pub fn expand_section(&mut self, section_id: usize) {
        self.collapsed_sections.insert(section_id, false);
    }

    /// Collapse a section
    pub fn collapse_section(&mut self, section_id: usize) {
        self.collapsed_sections.insert(section_id, true);
    }

    /// Expand all sections
    pub fn expand_all_sections(&mut self) {
        let section_ids: Vec<usize> = self.collapsed_sections.keys().copied().collect();
        for section_id in section_ids {
            self.collapsed_sections.insert(section_id, false);
        }
    }

    /// Collapse all sections
    pub fn collapse_all_sections(&mut self) {
        let section_ids: Vec<usize> = self.collapsed_sections.keys().copied().collect();
        for section_id in section_ids {
            self.collapsed_sections.insert(section_id, true);
        }
    }

    // ========== Expandable Content Methods ==========

    /// Set the default max lines for expandable content
    pub fn set_default_max_lines(&mut self, max_lines: usize) {
        self.default_max_lines = max_lines.max(1);
    }

    /// Get max lines for expandable content
    pub fn get_max_lines(&self, content_id: &str) -> usize {
        self.expandable_content
            .get(content_id)
            .map(|state| state.max_lines)
            .unwrap_or(self.default_max_lines)
    }

    /// Set max lines for expandable content
    pub fn set_max_lines(&mut self, content_id: &str, max_lines: usize) {
        let state = self
            .expandable_content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableState {
                collapsed: true,
                max_lines: self.default_max_lines,
            });
        state.max_lines = max_lines.max(1);
    }

    /// Toggle expandable content
    pub fn toggle_expandable(&mut self, content_id: &str) {
        let state = self
            .expandable_content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableState {
                collapsed: true,
                max_lines: self.default_max_lines,
            });
        state.collapsed = !state.collapsed;
    }

    /// Check if expandable content is collapsed
    pub fn is_expandable_collapsed(&self, content_id: &str) -> bool {
        self.expandable_content
            .get(content_id)
            .map(|state| state.collapsed)
            .unwrap_or(true)
    }

    /// Expand expandable content
    pub fn expand_expandable(&mut self, content_id: &str) {
        let state = self
            .expandable_content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableState {
                collapsed: true,
                max_lines: self.default_max_lines,
            });
        state.collapsed = false;
    }

    /// Collapse expandable content
    pub fn collapse_expandable(&mut self, content_id: &str) {
        let state = self
            .expandable_content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableState {
                collapsed: true,
                max_lines: self.default_max_lines,
            });
        state.collapsed = true;
    }

    /// Clear all state
    pub fn clear(&mut self) {
        self.scroll_offset = 0;
        self.total_lines = 0;
        self.current_line = 1;
        self.collapsed_sections.clear();
        self.expandable_content.clear();
    }
}

/// State for expandable content
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpandableState {
    /// Whether the content is collapsed (showing limited lines)
    pub collapsed: bool,
    /// Maximum number of visible lines when collapsed
    pub max_lines: usize,
}

impl ExpandableState {
    /// Create a new expandable state
    pub fn new(collapsed: bool, max_lines: usize) -> Self {
        Self {
            collapsed,
            max_lines: max_lines.max(1),
        }
    }
}

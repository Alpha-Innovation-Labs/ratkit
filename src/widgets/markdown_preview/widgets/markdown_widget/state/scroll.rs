//! Scroll state for markdown widget.
//!
//! Manages scroll offset, viewport dimensions, and current line position.
//! This module handles ONLY scrolling - nothing else.

/// Scroll state for markdown rendering.
///
/// Manages scroll position, viewport dimensions, and current line for navigation.
#[derive(Debug, Clone)]
pub struct ScrollState {
    /// Current scroll offset (0-indexed, first visible line index).
    pub scroll_offset: usize,
    /// Height of viewport (number of visible lines).
    pub viewport_height: usize,
    /// Total number of lines in document.
    pub total_lines: usize,
    /// Currently selected line (1-indexed, for highlighting).
    pub current_line: usize,
    /// Current filter text (when in filter mode).
    pub filter: Option<String>,
    /// Whether filter mode is currently active.
    pub filter_mode: bool,
}

/// Constructor for ScrollState.

impl ScrollState {
    /// Create a new scroll state with default settings.
    pub fn new() -> Self {
        Self {
            scroll_offset: 0,
            viewport_height: 20,
            total_lines: 0,
            current_line: 1,
            filter: None,
            filter_mode: false,
        }
    }
}

/// Adjust scroll for current line method for ScrollState.

impl ScrollState {
    /// Adjust scroll offset to ensure current_line is visible with a margin.
    /// Scrolls when the cursor gets within 3 lines of the top/bottom edge.
    pub fn adjust_scroll_for_current_line(&mut self) {
        const SCROLL_MARGIN: usize = 3;

        if self.viewport_height == 0 {
            return;
        }

        // Calculate the visible range
        let first_visible = self.scroll_offset + 1;
        let last_visible = self.scroll_offset + self.viewport_height;

        // If viewport is too small for margins, fall back to simple visibility
        if self.viewport_height <= SCROLL_MARGIN * 2 {
            if self.current_line < first_visible {
                self.scroll_offset = self.current_line.saturating_sub(1);
            } else if self.current_line > last_visible {
                self.scroll_offset = self.current_line.saturating_sub(self.viewport_height);
            }
            return;
        }

        // Scroll up if current line is within margin of the top
        let top_threshold = first_visible + SCROLL_MARGIN;
        if self.current_line < top_threshold && self.scroll_offset > 0 {
            // Scroll up to maintain margin
            let desired_offset = self.current_line.saturating_sub(SCROLL_MARGIN + 1);
            self.scroll_offset = desired_offset;
        }

        // Scroll down if current line is within margin of the bottom
        let bottom_threshold = last_visible.saturating_sub(SCROLL_MARGIN);
        if self.current_line > bottom_threshold {
            // Scroll down to maintain margin
            let desired_offset = self.current_line + SCROLL_MARGIN - self.viewport_height;
            self.scroll_offset = desired_offset.min(self.max_scroll_offset());
        }
    }
}

/// Filter line down method for ScrollState.

impl ScrollState {
    /// Move to the next line that matches the filter text.
    ///
    /// # Arguments
    ///
    /// * `filter_text` - The text to match against (case-insensitive)
    ///
    /// This method is used in filter mode to navigate through matching lines.
    /// It updates both `current_line` and `scroll_offset` to show the next match.
    pub fn filter_line_down(&mut self, _filter_text: String) {
        if self.current_line < self.total_lines {
            self.current_line += 1;
        }
        self.adjust_scroll_for_current_line();
    }

    /// Move to the previous line that matches the filter text.
    ///
    /// # Arguments
    ///
    /// * `filter_text` - The text to match against (case-insensitive)
    ///
    /// This method is used in filter mode to navigate through matching lines.
    /// It updates both `current_line` and `scroll_offset` to show the previous match.
    pub fn filter_line_up(&mut self, _filter_text: String) {
        if self.current_line > 1 {
            self.current_line -= 1;
        }
        self.adjust_scroll_for_current_line();
    }
}

#[cfg(test)]
mod filter_tests {
    use crate::widgets::markdown_preview::widgets::markdown_widget::state::ScrollState;

    #[test]
    fn test_filter_line_down() {
        let mut state = ScrollState {
            scroll_offset: 0,
            viewport_height: 10,
            total_lines: 100,
            current_line: 5,
            filter: None,
            filter_mode: false,
        };
        state.filter_line_down(String::new());
        assert_eq!(state.current_line, 6);
    }

    #[test]
    fn test_filter_line_up() {
        let mut state = ScrollState {
            scroll_offset: 0,
            viewport_height: 10,
            total_lines: 100,
            current_line: 5,
            filter: None,
            filter_mode: false,
        };
        state.filter_line_up(String::new());
        assert_eq!(state.current_line, 4);
    }

    #[test]
    fn test_filter_line_down_at_bottom() {
        let mut state = ScrollState {
            scroll_offset: 90,
            viewport_height: 10,
            total_lines: 100,
            current_line: 100,
            filter: None,
            filter_mode: false,
        };
        state.filter_line_down(String::new());
        assert_eq!(state.current_line, 100);
    }

    #[test]
    fn test_filter_line_up_at_top() {
        let mut state = ScrollState {
            scroll_offset: 0,
            viewport_height: 10,
            total_lines: 100,
            current_line: 1,
            filter: None,
            filter_mode: false,
        };
        state.filter_line_up(String::new());
        assert_eq!(state.current_line, 1);
    }
}

/// Filter line up method for ScrollState.

#[cfg(test)]
mod filter_empty_tests {
    use crate::widgets::markdown_preview::widgets::markdown_widget::state::ScrollState;

    #[test]
    fn test_filter_line_down_empty_filter() {
        let mut state = ScrollState {
            scroll_offset: 0,
            viewport_height: 10,
            total_lines: 100,
            current_line: 5,
            filter: None,
            filter_mode: false,
        };
        state.filter_line_down(String::new());
        assert_eq!(state.current_line, 6);
    }

    #[test]
    fn test_filter_line_up_empty_filter() {
        let mut state = ScrollState {
            scroll_offset: 0,
            viewport_height: 10,
            total_lines: 100,
            current_line: 5,
            filter: None,
            filter_mode: false,
        };
        state.filter_line_up(String::new());
        assert_eq!(state.current_line, 4);
    }

    #[test]
    fn test_filter_line_down_at_bottom() {
        let mut state = ScrollState {
            scroll_offset: 90,
            viewport_height: 10,
            total_lines: 100,
            current_line: 100,
            filter: None,
            filter_mode: false,
        };
        state.filter_line_down(String::new());
        assert_eq!(state.current_line, 100);
    }

    #[test]
    fn test_filter_line_up_at_top() {
        let mut state = ScrollState {
            scroll_offset: 0,
            viewport_height: 10,
            total_lines: 100,
            current_line: 1,
            filter: None,
            filter_mode: false,
        };
        state.filter_line_up(String::new());
        assert_eq!(state.current_line, 1);
    }
}

/// Is current line visible method for ScrollState.

impl ScrollState {
    /// Check if current line is visible in the viewport.
    ///
    /// # Returns
    ///
    /// `true` if the current line is within the visible viewport.
    pub fn is_current_line_visible(&self) -> bool {
        let first_visible = self.scroll_offset + 1;
        let last_visible = self.scroll_offset + self.viewport_height;
        self.current_line >= first_visible && self.current_line <= last_visible
    }
}

/// Line down method for ScrollState.

impl ScrollState {
    /// Move current line down (for keyboard navigation).
    pub fn line_down(&mut self) {
        if self.current_line < self.total_lines {
            self.current_line += 1;
        }
        self.adjust_scroll_for_current_line();
    }
}

/// Line up method for ScrollState.

impl ScrollState {
    /// Move current line up (for keyboard navigation).
    pub fn line_up(&mut self) {
        if self.current_line > 1 {
            self.current_line -= 1;
        }
        self.adjust_scroll_for_current_line();
    }
}

/// Max scroll offset method for ScrollState.

impl ScrollState {
    /// Get the maximum valid scroll offset.
    ///
    /// # Returns
    ///
    /// The maximum scroll offset that keeps content visible.
    pub fn max_scroll_offset(&self) -> usize {
        self.total_lines.saturating_sub(self.viewport_height)
    }
}

/// Scroll down method for ScrollState.

impl ScrollState {
    /// Scroll down by given number of lines.
    ///
    /// # Arguments
    ///
    /// * `amount` - Number of lines to scroll down.
    pub fn scroll_down(&mut self, amount: usize) {
        let max_offset = self.max_scroll_offset();
        self.scroll_offset = (self.scroll_offset + amount).min(max_offset);
    }
}

/// Scroll percentage method for ScrollState.

impl ScrollState {
    /// Calculate percentage scrolled (0.0 to 1.0).
    ///
    /// # Returns
    ///
    /// The scroll position as a percentage of total scrollable content.
    pub fn scroll_percentage(&self) -> f64 {
        let max_offset = self.max_scroll_offset();
        if max_offset == 0 {
            0.0
        } else {
            self.scroll_offset as f64 / max_offset as f64
        }
    }
}

/// Scroll to bottom method for ScrollState.

impl ScrollState {
    /// Move to bottom of document.
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.max_scroll_offset();
        self.current_line = self.total_lines;
    }
}

/// Scroll to top method for ScrollState.

impl ScrollState {
    /// Move to top of document.
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
        self.current_line = 1;
    }
}

/// Scroll up method for ScrollState.

impl ScrollState {
    /// Scroll up by given number of lines.
    ///
    /// # Arguments
    ///
    /// * `amount` - Number of lines to scroll up.
    pub fn scroll_up(&mut self, amount: usize) {
        let max_offset = self.max_scroll_offset();
        self.scroll_offset = self.scroll_offset.saturating_sub(amount).min(max_offset);
    }
}

/// Set current line method for ScrollState.

impl ScrollState {
    /// Set current line and adjust scroll to keep it visible.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number to set as current (1-indexed).
    pub fn set_current_line(&mut self, line: usize) {
        self.current_line = line.clamp(1, self.total_lines.max(1));
        self.adjust_scroll_for_current_line();
    }
}

/// Update total lines method for ScrollState.

impl ScrollState {
    /// Update total line count.
    ///
    /// # Arguments
    ///
    /// * `total` - The total number of lines in the document.
    pub fn update_total_lines(&mut self, total: usize) {
        self.total_lines = total.max(1);
    }
}

/// Update viewport method for ScrollState.
use ratatui::layout::Rect;

impl ScrollState {
    /// Update viewport dimensions.
    ///
    /// # Arguments
    ///
    /// * `area` - The new viewport area.
    pub fn update_viewport(&mut self, area: Rect) {
        self.viewport_height = area.height as usize;
    }
}

/// Visible range method for ScrollState.

impl ScrollState {
    /// Get range of currently visible lines (1-indexed, inclusive).
    ///
    /// # Returns
    ///
    /// A tuple of (start_line, end_line) for visible content.
    pub fn visible_range(&self) -> (usize, usize) {
        let start = self.scroll_offset + 1;
        let end = (self.scroll_offset + self.viewport_height).min(self.total_lines);
        (start, end)
    }
}

/// Default trait implementation for ScrollState.

impl Default for ScrollState {
    fn default() -> Self {
        Self::new()
    }
}

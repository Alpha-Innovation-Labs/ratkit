//! Filter line down method for ScrollState.

use crate::widgets::markdown_widget::state::scroll_state::ScrollState;

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
mod tests {
    use super::*;

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

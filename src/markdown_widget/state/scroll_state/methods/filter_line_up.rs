//! Filter line up method for ScrollState.

use crate::markdown_widget::state::scroll_state::ScrollState;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_line_down_empty_filter() {
        let mut state = ScrollState {
            scroll_offset: 0,
            viewport_height: 10,
            total_lines: 100,
            current_line: 5,
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
        };
        state.filter_line_up(String::new());
        assert_eq!(state.current_line, 1);
    }
}

//! Convert click position to scroll offset.

use ratatui::layout::Rect;

use crate::widgets::markdown_widget::state::scroll_state::ScrollState;

/// Convert a click Y position to a scroll offset.
///
/// # Arguments
///
/// * `click_y` - The Y coordinate of the click (absolute screen position).
/// * `area` - The scrollbar area rectangle.
/// * `scroll` - The current scroll state.
///
/// # Returns
///
/// The scroll offset that corresponds to clicking at the given position.
pub fn click_to_offset(click_y: u16, area: Rect, scroll: &ScrollState) -> usize {
    let track_height = area.height;
    if track_height == 0 {
        return 0;
    }

    // Calculate relative position within the track (0.0 to 1.0)
    let relative_y = click_y.saturating_sub(area.y);
    let ratio = relative_y as f64 / track_height as f64;

    // Calculate max scroll offset
    let max_scroll = scroll.total_lines.saturating_sub(scroll.viewport_height);

    // Convert ratio to scroll offset
    (ratio * max_scroll as f64).round() as usize
}

/// Check if a position is within the scrollbar area.
///
/// # Arguments
///
/// * `x` - The X coordinate to check.
/// * `y` - The Y coordinate to check.
/// * `area` - The scrollbar area rectangle.
///
/// # Returns
///
/// True if the position is within the scrollbar area.
pub fn is_in_scrollbar_area(x: u16, y: u16, area: Rect) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_click_at_top() {
        let area = Rect::new(0, 0, 1, 20);
        let scroll = ScrollState {
            scroll_offset: 0,
            viewport_height: 10,
            total_lines: 100,
            current_line: 1,
            filter: None,
            filter_mode: false,
        };
        let offset = click_to_offset(0, area, &scroll);
        assert_eq!(offset, 0);
    }

    #[test]
    fn test_click_at_bottom() {
        let area = Rect::new(0, 0, 1, 20);
        let scroll = ScrollState {
            scroll_offset: 0,
            viewport_height: 10,
            total_lines: 100,
            current_line: 1,
            filter: None,
            filter_mode: false,
        };
        let offset = click_to_offset(19, area, &scroll);
        // Should be close to max_scroll (90)
        assert!(offset >= 80);
    }

    #[test]
    fn test_click_at_middle() {
        let area = Rect::new(0, 0, 1, 20);
        let scroll = ScrollState {
            scroll_offset: 0,
            viewport_height: 10,
            total_lines: 100,
            current_line: 1,
            filter: None,
            filter_mode: false,
        };
        let offset = click_to_offset(10, area, &scroll);
        // Should be roughly half of max_scroll (45)
        assert!(offset >= 40 && offset <= 50);
    }

    #[test]
    fn test_is_in_area() {
        let area = Rect::new(10, 5, 2, 15);
        assert!(is_in_scrollbar_area(10, 5, area));
        assert!(is_in_scrollbar_area(11, 10, area));
        assert!(!is_in_scrollbar_area(9, 5, area));
        assert!(!is_in_scrollbar_area(12, 5, area));
        assert!(!is_in_scrollbar_area(10, 4, area));
        assert!(!is_in_scrollbar_area(10, 20, area));
    }
}

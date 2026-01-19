//! Calculate thumb position and size.

use crate::markdown_widget::state::scroll_state::ScrollState;

/// Calculate the thumb position (y offset) and height for the scrollbar.
///
/// # Arguments
///
/// * `scroll` - The scroll state to calculate from.
/// * `track_height` - The total height of the scrollbar track in characters.
/// * `min_thumb_height` - Minimum height for the thumb.
///
/// # Returns
///
/// A tuple of (thumb_y, thumb_height) where:
/// - `thumb_y` is the offset from the top of the track (0-indexed)
/// - `thumb_height` is the height of the thumb in characters
pub fn thumb_bounds(scroll: &ScrollState, track_height: u16, min_thumb_height: u16) -> (u16, u16) {
    let total = scroll.total_lines.max(1);
    let viewport = scroll.viewport_height.max(1);

    // If content fits in viewport, thumb fills entire track
    if total <= viewport {
        return (0, track_height);
    }

    // Thumb height = (viewport / total) * track_height
    let thumb_height = ((viewport as f64 / total as f64) * track_height as f64)
        .max(min_thumb_height as f64)
        .min(track_height as f64) as u16;

    // Calculate scrollable range
    let max_scroll = total.saturating_sub(viewport);
    let scrollable_track = track_height.saturating_sub(thumb_height);

    // Thumb position = (scroll_offset / max_scroll) * scrollable_track
    let thumb_y = if max_scroll > 0 && scrollable_track > 0 {
        ((scroll.scroll_offset as f64 / max_scroll as f64) * scrollable_track as f64) as u16
    } else {
        0
    };

    (thumb_y, thumb_height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_fits_viewport() {
        let scroll = ScrollState {
            scroll_offset: 0,
            viewport_height: 20,
            total_lines: 10,
            current_line: 1,
            filter: None,
            filter_mode: false,
        };
        let (y, height) = thumb_bounds(&scroll, 20, 1);
        assert_eq!(y, 0);
        assert_eq!(height, 20); // Thumb fills entire track
    }

    #[test]
    fn test_at_top() {
        let scroll = ScrollState {
            scroll_offset: 0,
            viewport_height: 10,
            total_lines: 100,
            current_line: 1,
            filter: None,
            filter_mode: false,
        };
        let (y, _height) = thumb_bounds(&scroll, 20, 1);
        assert_eq!(y, 0);
    }

    #[test]
    fn test_at_bottom() {
        let scroll = ScrollState {
            scroll_offset: 90, // max_scroll = 100 - 10 = 90
            viewport_height: 10,
            total_lines: 100,
            current_line: 1,
            filter: None,
            filter_mode: false,
        };
        let (y, height) = thumb_bounds(&scroll, 20, 1);
        // Thumb should be at bottom: y + height = track_height
        assert_eq!(y + height, 20);
    }

    #[test]
    fn test_min_thumb_height() {
        let scroll = ScrollState {
            scroll_offset: 0,
            viewport_height: 1,
            total_lines: 1000,
            current_line: 1,
            filter: None,
            filter_mode: false,
        };
        let (_y, height) = thumb_bounds(&scroll, 20, 3);
        assert!(height >= 3); // Should respect min_thumb_height
    }
}

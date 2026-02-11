//! Custom scrollbar widget for markdown document navigation.
//!
//! Provides a visual scrollbar that accurately tracks scroll position using block characters.
//! Supports click-to-scroll and drag interactions, with optional percentage indicator.
//!
//! # Features
//!
//! - Block character rendering (█ for thumb, ░ for track)
//! - Accurate scroll position tracking via ScrollState
//! - Click on track to jump to position
//! - Drag thumb to scroll
//! - Optional percentage indicator
//!
//! # Architecture
//!
//! The CustomScrollbar extension is a UI widget only - it receives `&ScrollState` as a parameter
//! and ONLY handles rendering. State mutations happen through ScrollState methods or via
//! the click_to_offset helper for interaction.

use crate::widgets::markdown_preview::widgets::markdown_widget::state::ScrollState;

/// Custom scrollbar widget for markdown navigation.
///
/// Shows scroll position using block characters with accurate tracking.
/// Supports click-to-scroll and drag interactions.
///
/// This is a UI-only widget that receives `&ScrollState` for state access.
/// State mutations happen through `ScrollState` methods, not here.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::state::scroll::ScrollState;
///
/// let scroll_state = ScrollState::default();
/// let scrollbar = CustomScrollbar::new(&scroll_state)
///     .config(ScrollbarConfig::default())
///     .show_percentage(true);
/// ```
#[derive(Debug)]
pub struct CustomScrollbar<'a> {
    /// Reference to the scroll state.
    pub(crate) scroll_state: &'a ScrollState,
    /// Configuration for appearance.
    pub(crate) config: ScrollbarConfig,
    /// Whether to show percentage indicator.
    pub(crate) show_percentage: bool,
}

/// Constructor methods for CustomScrollbar.

impl<'a> CustomScrollbar<'a> {
    /// Create a new CustomScrollbar with the given scroll state.
    ///
    /// # Arguments
    ///
    /// * `scroll_state` - Reference to the scroll state to track.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let scrollbar = CustomScrollbar::new(&scroll_state);
    /// ```
    pub fn new(scroll_state: &'a ScrollState) -> Self {
        Self {
            scroll_state,
            config: ScrollbarConfig::default(),
            show_percentage: false,
        }
    }

    /// Set the scrollbar configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to use.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn config(mut self, config: ScrollbarConfig) -> Self {
        self.config = config;
        self
    }

    /// Enable or disable the percentage indicator.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the percentage.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }
}

/// Configuration for scrollbar appearance.
use ratatui::style::{Color, Style};

/// Configuration for scrollbar appearance.
#[derive(Debug, Clone)]
pub struct ScrollbarConfig {
    /// Width of the scrollbar in characters.
    pub width: u16,
    /// Character used for the track (background).
    pub track_char: char,
    /// Character used for the thumb (scrollable indicator).
    pub thumb_char: char,
    /// Style for the track.
    pub track_style: Style,
    /// Style for the thumb.
    pub thumb_style: Style,
    /// Style for the percentage text.
    pub percentage_style: Style,
    /// Minimum height for the thumb in characters.
    pub min_thumb_height: u16,
}

impl Default for ScrollbarConfig {
    fn default() -> Self {
        Self {
            width: 1,
            track_char: '░',
            thumb_char: '█',
            track_style: Style::default().fg(Color::Rgb(50, 55, 65)),
            thumb_style: Style::default().fg(Color::Rgb(120, 130, 145)),
            percentage_style: Style::default().fg(Color::Rgb(70, 75, 85)),
            min_thumb_height: 1,
        }
    }
}

/// Convert click position to scroll offset.
use ratatui::layout::Rect;

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
    use crate::widgets::markdown_preview::widgets::markdown_widget::state::ScrollState;

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

/// Render the percentage indicator.
use ratatui::buffer::Buffer;

impl<'a> CustomScrollbar<'a> {
    /// Render the percentage indicator next to the scrollbar.
    ///
    /// Shows the current scroll percentage (0-100%) to the left of the scrollbar.
    ///
    /// # Arguments
    ///
    /// * `area` - The scrollbar area (percentage renders to the left).
    /// * `buf` - The buffer to render to.
    pub(crate) fn render_percentage(&self, area: Rect, buf: &mut Buffer) {
        let total = self.scroll_state.total_lines;
        let viewport = self.scroll_state.viewport_height;
        let offset = self.scroll_state.scroll_offset;

        // Calculate percentage
        let max_scroll = total.saturating_sub(viewport);
        let percentage = if max_scroll > 0 {
            ((offset as f64 / max_scroll as f64) * 100.0).round() as u8
        } else {
            0
        };

        // Format percentage string (right-aligned, 4 chars: "100%", " 50%", "  0%")
        let pct_str = format!("{:>3}%", percentage);

        // Render to the left of the scrollbar, vertically centered
        let text_x = area.x.saturating_sub(pct_str.len() as u16 + 1);
        let text_y = area.y + area.height / 2;

        if text_y < area.y + area.height {
            for (i, ch) in pct_str.chars().enumerate() {
                let x = text_x + i as u16;
                if let Some(cell) = buf.cell_mut((x, text_y)) {
                    cell.set_char(ch).set_style(self.config.percentage_style);
                }
            }
        }
    }
}

/// Render the scrollbar thumb (scrollable indicator).

impl<'a> CustomScrollbar<'a> {
    /// Render the thumb (scrollable indicator) of the scrollbar.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the thumb in (same as track area).
    /// * `buf` - The buffer to render to.
    pub(crate) fn render_thumb(&self, area: Rect, buf: &mut Buffer) {
        let (thumb_y, thumb_height) =
            thumb_bounds(self.scroll_state, area.height, self.config.min_thumb_height);

        let thumb_start = area.y + thumb_y;
        let thumb_end = thumb_start + thumb_height;

        for y in thumb_start..thumb_end.min(area.y + area.height) {
            for x in area.x..area.x + area.width {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(self.config.thumb_char)
                        .set_style(self.config.thumb_style);
                }
            }
        }
    }
}

/// Render the scrollbar track (background).

impl<'a> CustomScrollbar<'a> {
    /// Render the track (background) of the scrollbar.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the track in.
    /// * `buf` - The buffer to render to.
    pub(crate) fn render_track(&self, area: Rect, buf: &mut Buffer) {
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(self.config.track_char)
                        .set_style(self.config.track_style);
                }
            }
        }
    }
}

/// Calculate thumb position and size.

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
mod thumb_bounds_tests {
    use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::scrollbar::thumb_bounds;
    use crate::widgets::markdown_preview::widgets::markdown_widget::state::ScrollState;

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

/// Widget trait implementation for CustomScrollbar.
use ratatui::widgets::Widget;

impl<'a> Widget for CustomScrollbar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Don't render if content fits in viewport
        if self.scroll_state.total_lines <= self.scroll_state.viewport_height {
            return;
        }

        // Render track (background)
        self.render_track(area, buf);

        // Render thumb (scrollable indicator)
        self.render_thumb(area, buf);

        // Render percentage if enabled
        if self.show_percentage {
            self.render_percentage(area, buf);
        }
    }
}

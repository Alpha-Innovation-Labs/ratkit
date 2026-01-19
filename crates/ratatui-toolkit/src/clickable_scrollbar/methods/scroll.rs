//! Scrolling implementation for [`ClickableScrollbarState`].
//!
//! This module contains the implementation of scrolling methods for managing
//! scroll position, content bounds, and scroll increments.

use crate::clickable_scrollbar::methods::scroll_trait::ClickableScrollbarStateScrollExt;
use crate::clickable_scrollbar::ClickableScrollbarState;

impl ClickableScrollbarStateScrollExt for ClickableScrollbarState {
    fn set_content(mut self, content_len: usize, page_len: usize) -> Self {
        self.page_len = page_len;
        self.max_offset = content_len.saturating_sub(page_len);
        self
    }

    fn position(mut self, offset: usize) -> Self {
        self.offset = offset.min(self.max_offset);
        self
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn set_offset(&mut self, offset: usize) -> bool {
        let old = self.offset;
        self.offset = offset.min(self.max_offset);
        old != self.offset
    }

    fn scroll_up(&mut self, n: usize) -> bool {
        let old = self.offset;
        self.offset = self.offset.saturating_sub(n);
        old != self.offset
    }

    fn scroll_down(&mut self, n: usize) -> bool {
        let old = self.offset;
        self.offset = (self.offset + n).min(self.max_offset);
        old != self.offset
    }

    fn scroll_increment(&self) -> usize {
        self.scroll_by
            .unwrap_or_else(|| (self.page_len / 10).max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::layout::Rect;
    use ratatui::widgets::ScrollbarOrientation;

    fn create_test_state() -> ClickableScrollbarState {
        ClickableScrollbarState {
            area: Rect::default(),
            orientation: ScrollbarOrientation::VerticalRight,
            offset: 0,
            page_len: 10,
            max_offset: 90,
            scroll_by: None,
            drag_active: false,
        }
    }

    #[test]
    fn test_set_content() {
        let state = ClickableScrollbarState::new().set_content(100, 20);
        assert_eq!(state.page_len, 20);
        assert_eq!(state.max_offset, 80);
    }

    #[test]
    fn test_position() {
        let state = ClickableScrollbarState::new()
            .set_content(100, 10)
            .position(50);
        assert_eq!(state.offset, 50);
    }

    #[test]
    fn test_position_clamped() {
        let state = ClickableScrollbarState::new()
            .set_content(100, 10)
            .position(200);
        assert_eq!(state.offset, 90);
    }

    #[test]
    fn test_scroll_up() {
        let mut state = create_test_state();
        state.offset = 50;
        let changed = state.scroll_up(10);
        assert!(changed);
        assert_eq!(state.offset, 40);
    }

    #[test]
    fn test_scroll_up_saturating() {
        let mut state = create_test_state();
        state.offset = 5;
        let changed = state.scroll_up(10);
        assert!(changed);
        assert_eq!(state.offset, 0);
    }

    #[test]
    fn test_scroll_down() {
        let mut state = create_test_state();
        let changed = state.scroll_down(10);
        assert!(changed);
        assert_eq!(state.offset, 10);
    }

    #[test]
    fn test_scroll_down_saturating() {
        let mut state = create_test_state();
        state.offset = 85;
        let changed = state.scroll_down(10);
        assert!(changed);
        assert_eq!(state.offset, 90);
    }

    #[test]
    fn test_scroll_increment_default() {
        let state = ClickableScrollbarState::new().set_content(100, 50);
        assert_eq!(state.scroll_increment(), 5);
    }

    #[test]
    fn test_scroll_increment_minimum() {
        let state = ClickableScrollbarState::new().set_content(100, 5);
        assert_eq!(state.scroll_increment(), 1);
    }
}

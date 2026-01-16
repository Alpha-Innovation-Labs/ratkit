//! Stateful widget implementation for [`ClickableScrollbar`].
//!
//! This module contains the rendering logic for the scrollbar widget,
//! implementing the StatefulWidget pattern to render with mutable state.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{ScrollbarState, StatefulWidget};

use crate::clickable_scrollbar::methods::stateful_widget_trait::ClickableScrollbarStatefulWidgetExt;
use crate::clickable_scrollbar::{ClickableScrollbar, ClickableScrollbarState};

impl ClickableScrollbarStatefulWidgetExt for ClickableScrollbar<'_> {
    type State = ClickableScrollbarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.area = area;
        state.orientation = self.orientation;

        if area.is_empty() {
            return;
        }

        let mut scrollbar_state = ScrollbarState::new(state.max_offset)
            .position(state.offset)
            .viewport_content_length(state.page_len);

        StatefulWidget::render(self.scrollbar, area, buf, &mut scrollbar_state);
    }
}

impl StatefulWidget for ClickableScrollbar<'_> {
    type State = ClickableScrollbarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        ClickableScrollbarStatefulWidgetExt::render(self, area, buf, state);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;
    use ratatui::widgets::ScrollbarOrientation;

    use super::*;

    #[test]
    fn test_render_empty_area() {
        let scrollbar = ClickableScrollbar::vertical();
        let mut state = ClickableScrollbarState::new();
        let buf = &mut Buffer::empty(Rect::new(0, 0, 10, 10));

        StatefulWidget::render(scrollbar, Rect::default(), buf, &mut state);

        assert_eq!(state.area, Rect::default());
    }

    #[test]
    fn test_render_updates_state() {
        let scrollbar = ClickableScrollbar::vertical();
        let mut state = ClickableScrollbarState::new();
        let area = Rect::new(0, 0, 10, 20);
        let buf = &mut Buffer::empty(area);

        StatefulWidget::render(scrollbar, area, buf, &mut state);

        assert_eq!(state.area, area);
        assert_eq!(state.orientation, ScrollbarOrientation::VerticalRight);
    }
}

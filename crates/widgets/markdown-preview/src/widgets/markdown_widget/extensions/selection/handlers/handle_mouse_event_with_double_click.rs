//! Handle mouse event with double-click detection.

use ratatui::layout::Rect;

use crate::widgets::markdown_widget::foundation::events::MarkdownDoubleClickEvent;
use crate::widgets::markdown_widget::foundation::helpers::{get_line_at_position, is_in_area};
use crate::widgets::markdown_widget::state::{CollapseState, DoubleClickState, ScrollState};

/// Handle mouse event with double-click detection.
///
/// Returns `(handled, Option<MarkdownDoubleClickEvent>)`.
/// - `handled` is true if the event triggered an action (scroll)
/// - The event is `Some` if a double-click was detected
///
/// NOTE: This function does NOT process single-click actions (like heading collapse)
/// to avoid content shifting between clicks. Use `handle_mouse_event` separately
/// if you need single-click behavior, or check `pending_single_click()` for deferred handling.
///
/// # Arguments
///
/// * `event` - The mouse event
/// * `area` - The widget area
/// * `content` - The markdown content
/// * `scroll` - The scroll state
/// * `collapse` - The collapse state
/// * `double_click_state` - The double-click state tracker
///
/// # Returns
///
/// A tuple of `(handled, Option<MarkdownDoubleClickEvent>)`.
pub fn handle_mouse_event_with_double_click(
    event: &crossterm::event::MouseEvent,
    area: Rect,
    content: &str,
    scroll: &mut ScrollState,
    collapse: &CollapseState,
    double_click_state: &mut DoubleClickState,
) -> (bool, Option<MarkdownDoubleClickEvent>) {
    if !is_in_area(event.column, event.row, area) {
        return (false, None);
    }

    let relative_y = event.row.saturating_sub(area.y) as usize;
    let width = area.width as usize;

    match event.kind {
        crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            let (is_double, _should_process_pending) =
                double_click_state.process_click(event.column, event.row, scroll.scroll_offset);

            if is_double {
                // Double-click: return line info
                if let Some(evt) =
                    get_line_at_position(relative_y, width, content, scroll, collapse)
                {
                    return (true, Some(evt));
                }
            }
            // Single click: don't process heading toggles here to avoid content shifting
            // The caller should use handle_mouse_event separately if needed
            (false, None)
        }
        crossterm::event::MouseEventKind::ScrollUp => {
            scroll.scroll_up(5);
            (true, None)
        }
        crossterm::event::MouseEventKind::ScrollDown => {
            scroll.scroll_down(5);
            (true, None)
        }
        _ => (false, None),
    }
}

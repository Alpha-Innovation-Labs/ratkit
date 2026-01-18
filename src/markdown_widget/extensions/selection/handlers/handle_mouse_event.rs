//! Handle mouse event for the markdown widget.

use ratatui::layout::Rect;

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;
use crate::markdown_widget::foundation::helpers::is_in_area;

use super::super::helpers::{handle_click, should_render_line};

/// Handle mouse event for the markdown widget.
///
/// # Arguments
///
/// * `event` - The mouse event
/// * `area` - The widget area
/// * `content` - The markdown content
/// * `scroll` - The scroll manager
///
/// # Returns
///
/// `true` if the event was handled.
pub fn handle_mouse_event(
    event: &crossterm::event::MouseEvent,
    area: Rect,
    content: &str,
    scroll: &mut MarkdownScrollManager,
) -> bool {
    if !is_in_area(event.column, event.row, area) {
        return false;
    }

    let relative_y = event.row.saturating_sub(area.y) as usize;
    let relative_x = event.column.saturating_sub(area.x) as usize;

    let width = area.width as usize;

    match event.kind {
        crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            handle_click(relative_x, relative_y, width, content, scroll)
        }
        crossterm::event::MouseEventKind::ScrollUp => {
            scroll.scroll_up(5);
            true
        }
        crossterm::event::MouseEventKind::ScrollDown => {
            scroll.scroll_down(5);
            true
        }
        _ => false,
    }
}

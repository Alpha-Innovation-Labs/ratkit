//! Handle mouse event with text selection support.

use ratatui::layout::Rect;
use ratatui::text::Line;

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

use super::helpers::is_in_area;
use super::selection_state::SelectionState;

/// Result of handling a mouse event with selection.
#[derive(Debug, Clone, Copy, Default)]
pub struct SelectionMouseResult {
    /// Whether the event was handled.
    pub handled: bool,
    /// Whether text was copied to clipboard.
    pub copied: bool,
}

impl SelectionMouseResult {
    /// Create a new result indicating the event was handled.
    pub fn handled() -> Self {
        Self {
            handled: true,
            copied: false,
        }
    }

    /// Create a new result indicating nothing happened.
    pub fn none() -> Self {
        Self {
            handled: false,
            copied: false,
        }
    }

    /// Create a new result indicating text was copied.
    pub fn copied() -> Self {
        Self {
            handled: false,
            copied: true,
        }
    }
}

/// Handle mouse event with selection support.
///
/// This function extends `handle_mouse_event` to also handle text selection
/// via mouse drag. When the user drags the mouse, text is selected.
/// Text is automatically copied to clipboard when selection is complete.
///
/// # Arguments
///
/// * `event` - The mouse event
/// * `area` - The widget area
/// * `content` - The markdown content
/// * `scroll` - The scroll manager
/// * `selection` - The selection state
/// * `rendered_lines` - Currently rendered lines (for selection text extraction)
///
/// # Returns
///
/// A `SelectionMouseResult` indicating what happened.
pub fn handle_mouse_event_with_selection(
    event: &crossterm::event::MouseEvent,
    area: Rect,
    _content: &str,
    scroll: &mut MarkdownScrollManager,
    selection: &mut SelectionState,
    rendered_lines: &[Line<'static>],
) -> SelectionMouseResult {
    use crossterm::event::{MouseButton, MouseEventKind};

    if !is_in_area(event.column, event.row, area) {
        // Click outside area exits selection mode
        if selection.is_active() {
            selection.exit();
            return SelectionMouseResult::handled();
        }
        return SelectionMouseResult::none();
    }

    let relative_y = event.row.saturating_sub(area.y) as usize;
    let relative_x = event.column.saturating_sub(area.x) as usize;

    // Convert to document coordinates (accounting for scroll)
    let document_y = (relative_y + scroll.scroll_offset) as i32;
    let document_x = relative_x as i32;

    let width = area.width as usize;

    match event.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if selection.is_active() {
                // Exit selection mode on click (user clicked without dragging)
                selection.exit();
            }
            // Don't enter selection mode on click - only on drag
            // Don't handle click here - let the caller handle it via deferred processing
            SelectionMouseResult::none()
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            if !selection.is_active() {
                // Start selection on drag
                selection.enter(document_x, document_y, rendered_lines.to_vec(), width);
                selection.anchor = Some(super::selection_state::SelectionPos::new(
                    document_x, document_y,
                ));
            }
            // Update cursor position during drag
            selection.update_cursor(document_x, document_y);
            SelectionMouseResult::handled()
        }
        MouseEventKind::Up(MouseButton::Left) => {
            // Selection complete - auto-copy to clipboard
            if selection.is_active() && selection.has_selection() {
                // Update frozen lines with current rendered lines
                selection.frozen_lines = Some(rendered_lines.to_vec());
                selection.frozen_width = width;
                // Auto-copy to clipboard
                if let Some(text) = selection.get_selected_text() {
                    if !text.is_empty() {
                        if let Ok(mut clipboard) = arboard::Clipboard::new() {
                            if clipboard.set_text(&text).is_ok() {
                                return SelectionMouseResult::copied();
                            }
                        }
                    }
                }
            }
            SelectionMouseResult::none()
        }
        MouseEventKind::ScrollUp => {
            scroll.scroll_up(5);
            SelectionMouseResult::handled()
        }
        MouseEventKind::ScrollDown => {
            scroll.scroll_down(5);
            SelectionMouseResult::handled()
        }
        _ => SelectionMouseResult::none(),
    }
}

/// Copy selected text to clipboard.
///
/// # Arguments
///
/// * `selection` - The selection state
///
/// # Returns
///
/// `true` if text was copied successfully.
pub fn copy_selection_to_clipboard(selection: &SelectionState) -> bool {
    if let Some(text) = selection.get_selected_text() {
        if !text.is_empty() {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if clipboard.set_text(&text).is_ok() {
                    return true;
                }
            }
        }
    }
    false
}

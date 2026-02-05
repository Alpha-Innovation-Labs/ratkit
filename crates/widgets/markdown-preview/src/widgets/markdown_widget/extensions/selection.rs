//! Selection extension for markdown widget.
//!
//! Provides text selection via click-and-drag with auto-copy to clipboard.
//! Contains handlers for selection events, NOT state (state lives in state/selection).
//!
//! # Mouse Capture Requirement
//!
//! For text selection to work (drag to select, click to highlight),
//! you must enable mouse capture with crossterm:
//!
//! ```rust,ignore
//! use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
//! execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//! ```
//!
//! Without `EnableMouseCapture`, click and drag events will not be received.

//! Handle mouse event for the markdown widget.

use ratatui::layout::Rect;

use crate::widgets::markdown_widget::foundation::helpers::is_in_area;
use crate::widgets::markdown_widget::state::{
    CacheState, CollapseState, DoubleClickState, ExpandableState, ScrollState,
};

/// Handle mouse event for the markdown widget.
///
/// # Arguments
///
/// * `event` - The mouse event
/// * `area` - The widget area
/// * `content` - The markdown content
/// * `scroll` - The scroll state
/// * `collapse` - The collapse state
/// * `expandable` - The expandable state
/// * `cache` - The cache state
///
/// # Returns
///
/// `true` if the event was handled.
#[allow(clippy::too_many_arguments)]
pub fn handle_mouse_event(
    event: &crossterm::event::MouseEvent,
    area: Rect,
    content: &str,
    scroll: &mut ScrollState,
    collapse: &mut CollapseState,
    expandable: &mut ExpandableState,
    cache: &mut CacheState,
) -> bool {
    if !is_in_area(event.column, event.row, area) {
        return false;
    }

    let relative_y = event.row.saturating_sub(area.y) as usize;
    let relative_x = event.column.saturating_sub(area.x) as usize;

    let width = area.width as usize;

    match event.kind {
        crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            handle_click(
                relative_x, relative_y, width, content, scroll, collapse, expandable, cache,
            )
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

/// Handle mouse event with double-click detection.
use crate::widgets::markdown_widget::foundation::events::MarkdownDoubleClickEvent;
use crate::widgets::markdown_widget::foundation::helpers::get_line_at_position;

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

/// Handle click event at the given position.
use crate::widgets::markdown_widget::foundation::elements::{render, ElementKind, MarkdownElement};
use crate::widgets::markdown_widget::foundation::parser::render_markdown_to_elements;

/// Handle click event at the given position.
///
/// # Arguments
///
/// * `_x` - X coordinate (unused)
/// * `y` - Y coordinate relative to the widget
/// * `width` - Width of the widget
/// * `content` - The markdown content
/// * `scroll` - The scroll state
/// * `collapse` - The collapse state
/// * `expandable` - The expandable state
/// * `cache` - The cache state
///
/// # Returns
///
/// `true` if the click was handled.
#[allow(clippy::too_many_arguments)]
pub fn handle_click(
    _x: usize,
    y: usize,
    width: usize,
    content: &str,
    scroll: &ScrollState,
    collapse: &mut CollapseState,
    expandable: &mut ExpandableState,
    cache: &mut CacheState,
) -> bool {
    let elements = render_markdown_to_elements(content, true);

    // Account for scroll offset - y is relative to visible area
    let document_y = y + scroll.scroll_offset;
    let mut line_idx = 0;

    for (idx, element) in elements.iter().enumerate() {
        // Skip elements that shouldn't be rendered (collapsed sections)
        if !should_render_line(element, idx, collapse) {
            continue;
        }

        let rendered = render(element, width);
        let line_count = rendered.len();

        if document_y >= line_idx && document_y < line_idx + line_count {
            match &element.kind {
                ElementKind::Heading {
                    section_id,
                    collapsed: _,
                    ..
                } => {
                    collapse.toggle_section(*section_id);
                    cache.invalidate();
                    return true;
                }
                ElementKind::Frontmatter { .. } => {
                    collapse.toggle_section(0);
                    cache.invalidate();
                    return true;
                }
                ElementKind::FrontmatterStart { .. } => {
                    collapse.toggle_section(0);
                    cache.invalidate();
                    return true;
                }
                ElementKind::ExpandToggle { content_id, .. } => {
                    expandable.toggle(content_id);
                    cache.invalidate();
                    return true;
                }
                _ => {}
            }
        }

        line_idx += line_count;
    }

    false
}

/// Check if a line should be rendered based on collapse state.

/// Check if a markdown element should be rendered based on collapse state.
///
/// # Arguments
///
/// * `element` - The element to check
/// * `_idx` - The index of the element (unused but kept for API compatibility)
/// * `collapse` - The collapse state containing section collapse information
///
/// # Returns
///
/// `true` if the element should be rendered.
pub fn should_render_line(
    element: &MarkdownElement,
    _idx: usize,
    collapse: &CollapseState,
) -> bool {
    // Headings: visible unless a parent section is collapsed (hierarchical collapse)
    if let ElementKind::Heading { section_id, .. } = &element.kind {
        // Check if any parent section is collapsed
        if let Some((_, Some(parent))) = collapse.get_hierarchy(*section_id) {
            // If parent is collapsed, this heading is hidden
            if collapse.is_section_collapsed(parent) {
                return false;
            }
        }
        return true;
    }

    // Legacy Frontmatter block is always visible
    if matches!(element.kind, ElementKind::Frontmatter { .. }) {
        return true;
    }

    // FrontmatterStart is always visible (contains collapse toggle)
    if matches!(element.kind, ElementKind::FrontmatterStart { .. }) {
        return true;
    }

    // FrontmatterField and FrontmatterEnd are hidden when frontmatter is collapsed
    if matches!(
        element.kind,
        ElementKind::FrontmatterField { .. } | ElementKind::FrontmatterEnd
    ) {
        // Frontmatter uses section_id 0 for collapse state
        if collapse.is_section_collapsed(0) {
            return false;
        }
        return true;
    }

    // Check if this element belongs to a collapsed section
    if let Some(section_id) = element.section_id {
        if collapse.is_section_collapsed(section_id) {
            return false;
        }
    }

    true
}

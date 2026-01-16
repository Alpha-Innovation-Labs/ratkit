//! Markdown renderer widget with scroll and interaction support.
//!
//! This module provides a complete markdown rendering widget that handles
//! scrolling, click interactions, and collapse/expand of sections.

use super::{
    render_styled_line, scroll_manager::MarkdownScrollManager, styled_line::StyledLine,
    styled_line::StyledLineKind,
};
use ratatui::{
    layout::Rect,
    text::{Line, Text},
    widgets::Widget,
};

/// A scrollable, interactive markdown widget.
///
/// This widget renders markdown content with:
/// - Scroll support (keyboard and mouse)
/// - Clickable headings to collapse/expand sections
/// - Clickable frontmatter to collapse/expand
/// - Expandable content blocks ("Show more"/"Show less")
///
/// Note: This widget requires external scroll management. Use the
/// `render_markdown_scrollable` function along with `MarkdownScrollManager`
/// for full interactive support.
#[derive(Debug)]
pub struct MarkdownWidget<'a> {
    content: &'a str,
    scroll: &'a mut MarkdownScrollManager,
}

impl<'a> MarkdownWidget<'a> {
    /// Create a new MarkdownWidget with the given content and scroll manager.
    pub fn new(content: &'a str, scroll: &'a mut MarkdownScrollManager) -> Self {
        Self { content, scroll }
    }
}

impl<'a> Widget for MarkdownWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let inner_area = area;

        self.scroll.update_viewport(area);

        let text = render_markdown_interactive(self.content, self.scroll, inner_area);

        for (i, line) in text.lines.iter().enumerate() {
            if i < inner_area.height as usize {
                let y = inner_area.y + i as u16;
                for (j, span) in line.spans.iter().enumerate() {
                    if j < inner_area.width as usize {
                        let x = inner_area.x + j as u16;
                        buf.set_string(x, y, &span.content, span.style);
                    }
                }
            }
        }
    }
}

/// Render markdown with interactive scroll and collapse state.
pub fn render_markdown_interactive(
    content: &str,
    scroll: &mut MarkdownScrollManager,
    area: Rect,
) -> Text<'static> {
    let styled_lines = super::render_markdown_to_styled_lines(content);

    let inner_height = area.height.saturating_sub(2);
    scroll.update_total_lines(styled_lines.len());

    let mut visible_lines = Vec::new();
    let mut line_idx = 0;

    for (idx, styled_line) in styled_lines.iter().enumerate() {
        if should_render_line(styled_line, idx, scroll) {
            let rendered = render_styled_line(styled_line, area.width as usize);
            for line in rendered {
                if line_idx >= scroll.scroll_offset
                    && line_idx < scroll.scroll_offset + inner_height as usize
                {
                    visible_lines.push(line);
                }
                line_idx += 1;
            }
        }
    }

    Text::from(visible_lines)
}

/// Check if a line should be rendered based on collapse state.
fn should_render_line(
    styled_line: &StyledLine,
    _idx: usize,
    scroll: &MarkdownScrollManager,
) -> bool {
    match &styled_line.kind {
        StyledLineKind::Heading {
            section_id,
            collapsed,
            ..
        } => !(*collapsed && scroll.is_section_collapsed(*section_id)),
        StyledLineKind::Frontmatter { collapsed, .. } => {
            !(*collapsed && scroll.is_section_collapsed(0))
        }
        _ => true,
    }
}

/// Handle mouse event for the markdown widget.
///
/// Returns true if the event was handled.
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

    match event.kind {
        crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            handle_click(relative_x, relative_y, content, scroll)
        }
        crossterm::event::MouseEventKind::ScrollUp => {
            scroll.scroll_up(3);
            true
        }
        crossterm::event::MouseEventKind::ScrollDown => {
            scroll.scroll_down(3);
            true
        }
        _ => false,
    }
}

/// Handle click event at the given position.
fn handle_click(_x: usize, y: usize, content: &str, scroll: &mut MarkdownScrollManager) -> bool {
    let styled_lines = super::render_markdown_to_styled_lines(content);
    let mut line_idx = 0;

    for (idx, styled_line) in styled_lines.iter().enumerate() {
        let rendered = render_styled_line(styled_line, 80);
        let line_count = rendered.len();

        if y >= line_idx && y < line_idx + line_count {
            match &styled_line.kind {
                StyledLineKind::Heading {
                    section_id,
                    collapsed: _,
                    ..
                } => {
                    scroll.toggle_section_collapse(*section_id);
                    return true;
                }
                StyledLineKind::Frontmatter { .. } => {
                    scroll.toggle_section_collapse(0);
                    return true;
                }
                StyledLineKind::ExpandToggle { content_id, .. } => {
                    scroll.toggle_expandable(content_id);
                    return true;
                }
                _ => {}
            }
        }

        if should_render_line(styled_line, idx, scroll) {
            line_idx += line_count;
        }
    }

    false
}

/// Check if a position is within an area.
fn is_in_area(x: u16, y: u16, area: Rect) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}

/// Render markdown to styled lines with scroll state applied.
#[allow(dead_code)]
pub fn render_markdown_scrollable(
    content: &str,
    scroll: &MarkdownScrollManager,
    width: usize,
) -> Vec<Line<'static>> {
    let styled_lines = super::render_markdown_to_styled_lines(content);
    let mut result = Vec::new();

    for (idx, styled_line) in styled_lines.iter().enumerate() {
        if should_render_line(styled_line, idx, scroll) {
            let rendered = render_styled_line(styled_line, width);
            result.extend(rendered);
        }
    }

    result
}

/// Find the styled line at the given screen position.
#[allow(dead_code)]
pub fn find_line_at_position(
    content: &str,
    screen_y: usize,
    scroll: &MarkdownScrollManager,
) -> Option<(usize, StyledLine)> {
    let styled_lines = super::render_markdown_to_styled_lines(content);
    let mut current_y = 0;

    for (idx, styled_line) in styled_lines.iter().enumerate() {
        if should_render_line(styled_line, idx, scroll) {
            let rendered = render_styled_line(styled_line, 80);
            let line_count = rendered.len();

            if screen_y >= current_y && screen_y < current_y + line_count {
                return Some((idx, styled_line.clone()));
            }

            current_y += line_count;
        }
    }

    None
}

/// Check if a position is on a clickable element.
#[allow(dead_code)]
pub fn is_clickable_at_position(
    content: &str,
    _x: usize,
    y: usize,
    scroll: &MarkdownScrollManager,
) -> bool {
    if let Some((_, styled_line)) = find_line_at_position(content, y, scroll) {
        matches!(
            styled_line.kind,
            StyledLineKind::Heading { .. }
                | StyledLineKind::Frontmatter { .. }
                | StyledLineKind::ExpandToggle { .. }
        )
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::markdown_renderer::styled_line::StyledLine;
    use crate::markdown_renderer::{
        scroll_manager::MarkdownScrollManager, styled_line::StyledLineKind, TextSegment,
    };
    use ratatui::layout::Rect;

    use super::{is_in_area, should_render_line};

    #[test]
    fn test_should_render_heading_when_not_collapsed() {
        let scroll = MarkdownScrollManager::new();
        let styled_line = StyledLine {
            kind: StyledLineKind::Heading {
                level: 1,
                text: vec![TextSegment::Plain("Test".to_string())],
                section_id: 0,
                collapsed: false,
            },
        };
        assert!(should_render_line(&styled_line, 0, &scroll));
    }

    #[test]
    fn test_should_not_render_collapsed_heading() {
        let mut scroll = MarkdownScrollManager::new();
        scroll.collapse_section(0);

        let styled_line = StyledLine {
            kind: StyledLineKind::Heading {
                level: 1,
                text: vec![TextSegment::Plain("Test".to_string())],
                section_id: 0,
                collapsed: true,
            },
        };
        assert!(!should_render_line(&styled_line, 0, &scroll));
    }

    #[test]
    fn test_is_in_area() {
        let area = Rect::new(10, 5, 40, 20);
        assert!(is_in_area(15, 10, area));
        assert!(!is_in_area(5, 10, area));
        assert!(!is_in_area(55, 10, area));
    }
}

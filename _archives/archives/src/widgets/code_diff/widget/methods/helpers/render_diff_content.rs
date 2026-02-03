//! Helper function to render the diff content panel.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, Widget};

use crate::widgets::code_diff::code_diff::methods::helpers::{render_header, render_side_by_side};
use crate::widgets::code_diff::code_diff::CodeDiff;
use crate::widgets::code_diff::enums::DiffStyle;

/// Renders the diff content panel with border.
///
/// # Arguments
///
/// * `diff` - The CodeDiff instance
/// * `area` - The area to render the diff in
/// * `buf` - The buffer to render to
/// * `show_border` - Whether to show a border around the diff
pub fn render_diff_content(diff: &CodeDiff, area: Rect, buf: &mut Buffer, show_border: bool) {
    if area.width < 3 || area.height < 3 {
        return;
    }

    let theme = &diff.theme;

    let content_area = if show_border {
        // Create border with focus indicator
        let border_style = if !diff.sidebar_focused && diff.config.sidebar_enabled {
            Style::default().fg(theme.border_active)
        } else {
            Style::default().fg(theme.border)
        };

        let title = diff
            .file_path
            .as_ref()
            .map(|p| format!(" {} ", p))
            .unwrap_or_else(|| " Diff ".to_string());

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(title);

        let inner_area = block.inner(area);
        block.render(area, buf);
        inner_area
    } else {
        area
    };

    if content_area.width == 0 || content_area.height == 0 {
        return;
    }

    // Render header (file stats)
    let header_height = render_header(diff, content_area, buf);

    // Calculate area for diff content
    let diff_area = Rect {
        x: content_area.x,
        y: content_area.y + header_height,
        width: content_area.width,
        height: content_area.height.saturating_sub(header_height),
    };

    if diff_area.height == 0 {
        return;
    }

    // Render based on style
    match diff.config.style {
        DiffStyle::SideBySide => {
            render_side_by_side(diff, diff_area, buf);
        }
        DiffStyle::Unified | DiffStyle::Inline => {
            // For now, unified and inline fall back to side-by-side
            render_side_by_side(diff, diff_area, buf);
        }
    }
}

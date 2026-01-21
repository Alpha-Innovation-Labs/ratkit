//! Render the diff in side-by-side mode.

use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Style;

use crate::services::theme::AppTheme;
use crate::widgets::code_diff::code_diff::CodeDiff;
use crate::widgets::code_diff::enums::DiffLineKind;

use super::build_aligned_lines::build_aligned_lines;
use super::render_line::{render_line, RenderLineContext};
use super::render_line_number::render_line_number;

/// Renders the diff in side-by-side mode.
///
/// # Arguments
///
/// * `diff` - The diff widget to render
/// * `area` - The area to render in (below the header)
/// * `buf` - The buffer to render to
pub fn render_side_by_side(diff: &CodeDiff, area: Rect, buf: &mut Buffer) {
    if area.height == 0 || area.width == 0 {
        return;
    }

    let theme = &diff.theme;

    // Calculate column widths
    // Each side gets: line_number + gutter + content
    let line_num_width = if diff.config.show_line_numbers {
        diff.config.line_number_width + 1 // +1 for separator
    } else {
        0
    };

    let gutter_width = diff.config.gutter_width;
    let total_overhead = (line_num_width + gutter_width) * 2 + 1; // +1 for center divider

    if area.width < total_overhead + 2 {
        return; // Not enough space
    }

    let content_width = (area.width - total_overhead) / 2;
    let left_width = line_num_width + gutter_width + content_width;
    let right_start = area.x + left_width + 1; // +1 for divider
    let right_width = area.width - left_width - 1;

    // Build a flat list of renderable rows
    let mut rows: Vec<RenderRow> = Vec::new();

    for hunk in &diff.hunks {
        // Add hunk header row
        rows.push(RenderRow::HunkHeader(hunk.header_text()));

        // Add aligned line pairs
        let aligned = build_aligned_lines(hunk);
        for pair in aligned {
            rows.push(RenderRow::LinePair {
                left: pair.left,
                right: pair.right,
            });
        }
    }

    // Render visible rows
    let visible_start = diff.scroll_offset;

    for (row_idx, row) in rows
        .iter()
        .enumerate()
        .skip(visible_start)
        .take(area.height as usize)
    {
        let y = area.y + (row_idx - visible_start) as u16;

        match row {
            RenderRow::HunkHeader(text) => {
                // Render hunk header spanning full width
                render_full_width_header(text, &diff.config, area.x, y, area.width, buf);
            }
            RenderRow::LinePair { left, right } => {
                // Render left side
                let left_bg = get_line_bg(left.as_ref(), &diff.config, theme, true);
                let mut x = area.x;

                x = render_line_number(
                    left.as_ref().and_then(|l| l.old_line_num),
                    &diff.config,
                    x,
                    y,
                    buf,
                    left_bg,
                );

                let ctx = RenderLineContext {
                    buf,
                    x,
                    y,
                    width: left_width - (x - area.x),
                    is_left: true,
                };
                render_line(left.as_ref(), &diff.config, theme, ctx);

                // Render center divider
                let divider_x = area.x + left_width;
                if let Some(cell) = buf.cell_mut(Position::new(divider_x, y)) {
                    cell.set_char('|');
                    cell.set_style(Style::default().fg(theme.border));
                }

                // Render right side
                let right_bg = get_line_bg(right.as_ref(), &diff.config, theme, false);
                let mut x = right_start;

                x = render_line_number(
                    right.as_ref().and_then(|l| l.new_line_num),
                    &diff.config,
                    x,
                    y,
                    buf,
                    right_bg,
                );

                let ctx = RenderLineContext {
                    buf,
                    x,
                    y,
                    width: right_width.saturating_sub(x - right_start),
                    is_left: false,
                };
                render_line(right.as_ref(), &diff.config, theme, ctx);
            }
        }
    }
}

/// A row to render in the side-by-side view.
enum RenderRow {
    HunkHeader(String),
    LinePair {
        left: Option<crate::widgets::code_diff::diff_line::DiffLine>,
        right: Option<crate::widgets::code_diff::diff_line::DiffLine>,
    },
}

/// Gets the background style for a line.
fn get_line_bg(
    line: Option<&crate::widgets::code_diff::diff_line::DiffLine>,
    config: &crate::widgets::code_diff::diff_config::DiffConfig,
    theme: &AppTheme,
    is_left: bool,
) -> Style {
    match line.map(|l| l.kind) {
        Some(DiffLineKind::Added) => Style::default().bg(theme.diff.added_bg),
        Some(DiffLineKind::Removed) => Style::default().bg(theme.diff.removed_bg),
        Some(DiffLineKind::Context) => Style::default(),
        Some(DiffLineKind::HunkHeader) => Style::default().bg(config.hunk_header_bg),
        None => {
            // Empty filler - use a slightly tinted background from theme
            let bg = if is_left {
                theme.diff.removed_bg
            } else {
                theme.diff.added_bg
            };
            Style::default().bg(bg)
        }
    }
}

/// Renders a hunk header spanning the full width.
fn render_full_width_header(
    text: &str,
    config: &crate::widgets::code_diff::diff_config::DiffConfig,
    x: u16,
    y: u16,
    width: u16,
    buf: &mut Buffer,
) {
    let style = Style::default()
        .bg(config.hunk_header_bg)
        .fg(config.hunk_header_fg);

    // Clear the row
    for col in x..x + width {
        if let Some(cell) = buf.cell_mut(Position::new(col, y)) {
            cell.set_char(' ');
            cell.set_style(style);
        }
    }

    // Render text
    let mut current_x = x + 1;
    for ch in text.chars() {
        if current_x >= x + width {
            break;
        }
        if let Some(cell) = buf.cell_mut(Position::new(current_x, y)) {
            cell.set_char(ch);
            cell.set_style(style);
        }
        current_x += 1;
    }
}

//! Render the file header bar for the diff widget.

use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Modifier, Style};

use crate::code_diff::code_diff::CodeDiff;

/// Renders the file header bar at the top of the diff widget.
///
/// The header shows:
/// - File path (if available)
/// - Stats (+N -M)
///
/// # Arguments
///
/// * `diff` - The diff widget to render
/// * `area` - The area to render the header in
/// * `buf` - The buffer to render to
///
/// # Returns
///
/// The number of rows used by the header (1 if rendered, 0 if no header needed)
pub fn render_header(diff: &CodeDiff, area: Rect, buf: &mut Buffer) -> u16 {
    if diff.file_path.is_none() && diff.hunks.is_empty() {
        return 0;
    }

    if area.height == 0 {
        return 0;
    }

    let theme = &diff.theme;
    let header_bg = theme.background_panel;
    let header_fg = theme.text;

    let header_style = Style::default()
        .bg(header_bg)
        .fg(header_fg)
        .add_modifier(Modifier::BOLD);

    // Clear the header row
    for x in area.x..area.x + area.width {
        if let Some(cell) = buf.cell_mut(Position::new(x, area.y)) {
            cell.set_style(header_style);
            cell.set_char(' ');
        }
    }

    // Render file path
    let mut x = area.x + 1;
    if let Some(ref path) = diff.file_path {
        let path_style = Style::default()
            .bg(header_bg)
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD);

        for ch in path.chars() {
            if x >= area.x + area.width - 15 {
                break;
            }
            if let Some(cell) = buf.cell_mut(Position::new(x, area.y)) {
                cell.set_char(ch);
                cell.set_style(path_style);
            }
            x += 1;
        }
    }

    // Render stats on the right side
    let stats = diff.stats_text();
    let stats_x = area.x + area.width.saturating_sub(stats.len() as u16 + 2);

    // Added count in success color (green)
    let added_count = format!("+{}", diff.added_count());
    let mut sx = stats_x;
    for ch in added_count.chars() {
        if sx < area.x + area.width {
            if let Some(cell) = buf.cell_mut(Position::new(sx, area.y)) {
                cell.set_char(ch);
                cell.set_style(Style::default().bg(header_bg).fg(theme.diff.added));
            }
            sx += 1;
        }
    }

    // Space
    if sx < area.x + area.width {
        if let Some(cell) = buf.cell_mut(Position::new(sx, area.y)) {
            cell.set_char(' ');
        }
        sx += 1;
    }

    // Removed count in error color (red)
    let removed_count = format!("-{}", diff.removed_count());
    for ch in removed_count.chars() {
        if sx < area.x + area.width {
            if let Some(cell) = buf.cell_mut(Position::new(sx, area.y)) {
                cell.set_char(ch);
                cell.set_style(Style::default().bg(header_bg).fg(theme.diff.removed));
            }
            sx += 1;
        }
    }

    1
}

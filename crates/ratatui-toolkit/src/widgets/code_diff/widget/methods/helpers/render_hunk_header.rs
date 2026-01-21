use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Style;

use crate::widgets::code_diff::diff_config::DiffConfig;
use crate::widgets::code_diff::diff_hunk::DiffHunk;

/// Renders a hunk header line (e.g., "@@ -1,4 +1,5 @@ fn main()").
///
/// # Arguments
///
/// * `hunk` - The diff hunk whose header to render
/// * `config` - Display configuration
/// * `area` - The area to render in (single line)
/// * `buf` - The buffer to render to
#[allow(dead_code)]
pub fn render_hunk_header(hunk: &DiffHunk, config: &DiffConfig, area: Rect, buf: &mut Buffer) {
    if area.height == 0 || area.width == 0 {
        return;
    }

    let style = Style::default()
        .bg(config.hunk_header_bg)
        .fg(config.hunk_header_fg);

    // Clear the entire row with the hunk header style
    for x in area.x..area.x + area.width {
        if let Some(cell) = buf.cell_mut(Position::new(x, area.y)) {
            cell.set_style(style);
            cell.set_char(' ');
        }
    }

    // Render the header text
    let header_text = hunk.header_text();
    let mut x = area.x + 1;

    for ch in header_text.chars() {
        if x >= area.x + area.width {
            break;
        }
        if let Some(cell) = buf.cell_mut(Position::new(x, area.y)) {
            cell.set_char(ch);
            cell.set_style(style);
        }
        x += 1;
    }
}

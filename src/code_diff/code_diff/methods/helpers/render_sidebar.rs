//! Helper function to render the sidebar file tree.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, Widget};

use crate::code_diff::code_diff::CodeDiff;

/// Renders the sidebar file tree.
///
/// # Arguments
///
/// * `diff` - The CodeDiff instance
/// * `area` - The area to render the sidebar in
/// * `buf` - The buffer to render to
pub fn render_sidebar(diff: &CodeDiff, area: Rect, buf: &mut Buffer) {
    if area.width < 3 || area.height < 3 {
        return;
    }

    let theme = &diff.theme;

    // Create border with focus indicator
    let border_style = if diff.sidebar_focused {
        Style::default().fg(theme.border_active)
    } else {
        Style::default().fg(theme.border)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style)
        .title(" Files ");

    let inner_area = block.inner(area);
    block.render(area, buf);

    // Render the file tree widget
    (&diff.file_tree).render(inner_area, buf);
}

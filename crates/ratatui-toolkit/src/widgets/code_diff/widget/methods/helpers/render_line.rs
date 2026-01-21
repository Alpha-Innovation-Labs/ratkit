//! Render a single diff line.

use ratatui::buffer::Buffer;
use ratatui::layout::Position;
use ratatui::style::Style;

use crate::services::theme::AppTheme;
use crate::widgets::code_diff::diff_config::DiffConfig;
use crate::widgets::code_diff::diff_line::DiffLine;
use crate::widgets::code_diff::enums::DiffLineKind;

/// Context for rendering a diff line.
pub struct RenderLineContext<'a> {
    /// Buffer to render to.
    pub buf: &'a mut Buffer,
    /// Starting x position (after line number).
    pub x: u16,
    /// Y position.
    pub y: u16,
    /// Available width for content.
    pub width: u16,
    /// Whether this is the left (old) or right (new) panel.
    pub is_left: bool,
}

/// Renders a single diff line content.
///
/// # Arguments
///
/// * `line` - The diff line to render (None for empty filler)
/// * `config` - Display configuration
/// * `theme` - Application theme for colors
/// * `ctx` - Rendering context with position, width, buffer, and panel info
pub fn render_line(
    line: Option<&DiffLine>,
    config: &DiffConfig,
    theme: &AppTheme,
    ctx: RenderLineContext,
) {
    let (bg_color, fg_color, prefix) = match line.map(|l| l.kind) {
        Some(DiffLineKind::Added) => (theme.diff.added_bg, theme.diff.added, '+'),
        Some(DiffLineKind::Removed) => (theme.diff.removed_bg, theme.diff.removed, '-'),
        Some(DiffLineKind::Context) => (theme.background, theme.text, ' '),
        Some(DiffLineKind::HunkHeader) => (theme.background_panel, theme.text_muted, '@'),
        None => {
            // Empty filler - use the diff background colors
            let filler_bg = if ctx.is_left {
                theme.diff.removed_bg
            } else {
                theme.diff.added_bg
            };
            (filler_bg, theme.text_muted, ' ')
        }
    };

    let style = Style::default().bg(bg_color).fg(fg_color);

    // Clear the line area with background
    for col in ctx.x..ctx.x + ctx.width {
        if let Some(cell) = ctx.buf.cell_mut(Position::new(col, ctx.y)) {
            cell.set_char(' ');
            cell.set_style(style);
        }
    }

    // Render gutter prefix
    let mut current_x = ctx.x;
    if config.gutter_width > 0 {
        if let Some(cell) = ctx.buf.cell_mut(Position::new(current_x, ctx.y)) {
            cell.set_char(prefix);
            cell.set_style(style);
        }
        current_x += 1;

        // Padding after prefix
        for _ in 1..config.gutter_width {
            if current_x < ctx.x + ctx.width {
                if let Some(cell) = ctx.buf.cell_mut(Position::new(current_x, ctx.y)) {
                    cell.set_char(' ');
                    cell.set_style(style);
                }
                current_x += 1;
            }
        }
    }

    // Render content
    if let Some(line) = line {
        for ch in line.content.chars() {
            if current_x >= ctx.x + ctx.width {
                break;
            }
            if let Some(cell) = ctx.buf.cell_mut(Position::new(current_x, ctx.y)) {
                cell.set_char(ch);
                cell.set_style(style);
            }
            current_x += 1;
        }
    }
}

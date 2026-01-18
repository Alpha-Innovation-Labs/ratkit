//! Standalone statusline rendering function for markdown widgets.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Widget,
};
use unicode_width::UnicodeWidthStr;

use super::markdown_widget::{GitStats, MarkdownWidgetMode};
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;
use crate::statusline_stacked::{StatusLineStacked, SLANT_BL_TR, SLANT_TL_BR};

/// Render a markdown statusline with powerline style.
///
/// This standalone function renders a statusline that can be used independently
/// of the MarkdownWidget, for cases where custom content rendering is needed.
///
/// The statusline displays:
/// - Mode indicator (NORMAL/DRAG) on the left with colored background
/// - Filename with git stats (no background on git icons)
/// - Scroll position (percentage/total lines) on the right
///
/// # Arguments
///
/// * `area` - The area to render the statusline in
/// * `buf` - The buffer to render to
/// * `mode` - The current widget mode (Normal or Drag)
/// * `filename` - Optional filename to display
/// * `git_stats` - Optional git statistics to display
/// * `current_line` - Current scroll position (line number)
/// * `total_lines` - Total number of lines in the content
pub fn render_markdown_statusline(
    area: Rect,
    buf: &mut Buffer,
    mode: MarkdownWidgetMode,
    filename: Option<&str>,
    git_stats: Option<GitStats>,
    current_line: usize,
    total_lines: usize,
) {
    // Mode colors and text
    let (mode_text, mode_color) = match mode {
        MarkdownWidgetMode::Normal => (" NORMAL ", Color::Rgb(97, 175, 239)), // blue
        MarkdownWidgetMode::Drag => (" DRAG ", Color::Rgb(229, 192, 123)),    // yellow/orange
    };

    let file_bg = Color::Rgb(58, 58, 58); // slightly darker than #686868

    // Position info
    let percentage = if total_lines == 0 {
        0
    } else {
        (current_line * 100) / total_lines.max(1)
    };
    let position_text = format!(" {}%/{} ", percentage, total_lines);
    let position_bg = Color::Rgb(171, 178, 191);

    // Build the statusline
    let mut statusline = StatusLineStacked::new()
        // Mode segment (left)
        .start(
            Span::from(mode_text).style(
                Style::new()
                    .fg(Color::Black)
                    .bg(mode_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(SLANT_TL_BR).style(Style::new().fg(mode_color).bg(file_bg)),
        );

    // Filename segment with git stats (git stats have no bg - just colored text)
    if let Some(name) = filename {
        let file_segment = format!(" {} ", name);
        statusline = statusline.start(
            Span::from(file_segment).style(Style::new().fg(Color::White).bg(file_bg)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(file_bg)),
        );
    }

    // Git stats rendered directly after the slant (no background)
    // We render these manually after the StatusLineStacked
    let git_stats_start_x = if filename.is_some() {
        // Calculate position after mode + filename segments
        let mode_len = mode_text.len() as u16 + 1; // +1 for slant
        let file_len = filename.map(|n| n.len() + 2).unwrap_or(0) as u16 + 1; // +2 for spaces, +1 for slant
        area.x + mode_len + file_len
    } else {
        let mode_len = mode_text.len() as u16 + 1;
        area.x + mode_len
    };

    // Position segment (right)
    statusline = statusline.end(
        Span::from(position_text).style(Style::new().fg(Color::Black).bg(position_bg)),
        Span::from(SLANT_BL_TR).style(Style::new().fg(position_bg)),
    );

    // Render the statusline base
    statusline.render(area, buf);

    // Now render git stats with colored icons (no background)
    // Icons from lvim: LineAdded (U+EADC), LineModified (U+EADE), LineRemoved (U+EADF)
    if let Some(stats) = git_stats {
        let green = Style::new().fg(Color::Rgb(152, 195, 121)); // green for adds
        let yellow = Style::new().fg(Color::Rgb(229, 192, 123)); // yellow for modified
        let red = Style::new().fg(Color::Rgb(224, 108, 117)); // red for deletions
        let dim = Style::new().fg(Color::Rgb(92, 99, 112)); // dim for separators

        let mut x = git_stats_start_x;

        // Add margin after filename
        buf.set_string(x, area.y, "  ", dim);
        x += 2;

        // Added: icon space number space
        let add_icon = "\u{EADC}";
        let add_num = format!("{}", stats.additions);
        buf.set_string(x, area.y, add_icon, green);
        x += add_icon.width() as u16;
        buf.set_string(x, area.y, " ", green);
        x += 1;
        buf.set_string(x, area.y, &add_num, green);
        x += add_num.width() as u16;
        buf.set_string(x, area.y, " ", dim);
        x += 1;

        // Modified: icon space number space
        let mod_icon = "\u{EADE}";
        let mod_num = format!("{}", stats.modified);
        buf.set_string(x, area.y, mod_icon, yellow);
        x += mod_icon.width() as u16;
        buf.set_string(x, area.y, " ", yellow);
        x += 1;
        buf.set_string(x, area.y, &mod_num, yellow);
        x += mod_num.width() as u16;
        buf.set_string(x, area.y, " ", dim);
        x += 1;

        // Removed: icon space number
        let del_icon = "\u{EADF}";
        let del_num = format!("{}", stats.deletions);
        buf.set_string(x, area.y, del_icon, red);
        x += del_icon.width() as u16;
        buf.set_string(x, area.y, " ", red);
        x += 1;
        buf.set_string(x, area.y, &del_num, red);
    }
}

/// Render a markdown statusline with optional theme support.
///
/// This function is like `render_markdown_statusline` but with optional theme colors.
/// When a theme is provided, uses theme colors for mode indicators, git stats, etc.
///
/// # Arguments
///
/// * `area` - The area to render the statusline in
/// * `buf` - The buffer to render to
/// * `mode` - The current widget mode (Normal or Drag)
/// * `filename` - Optional filename to display
/// * `git_stats` - Optional git statistics to display
/// * `current_line` - Current scroll position (line number)
/// * `total_lines` - Total number of lines in the content
/// * `app_theme` - Optional application theme for styling
pub fn render_markdown_statusline_themed(
    area: Rect,
    buf: &mut Buffer,
    mode: MarkdownWidgetMode,
    filename: Option<&str>,
    git_stats: Option<GitStats>,
    current_line: usize,
    total_lines: usize,
    app_theme: Option<&crate::theme::AppTheme>,
) {
    // Mode colors and text - use theme if available
    let (mode_text, mode_color) = match mode {
        MarkdownWidgetMode::Normal => {
            let color = app_theme
                .map(|t| t.info)
                .unwrap_or(Color::Rgb(97, 175, 239)); // blue
            (" NORMAL ", color)
        }
        MarkdownWidgetMode::Drag => {
            let color = app_theme
                .map(|t| t.warning)
                .unwrap_or(Color::Rgb(229, 192, 123)); // yellow/orange
            (" DRAG ", color)
        }
    };

    // File segment background - use theme background_panel or default
    let file_bg = app_theme
        .map(|t| t.background_panel)
        .unwrap_or(Color::Rgb(58, 58, 58));

    // Mode text foreground - use theme background or default black
    let mode_fg = app_theme.map(|t| t.background).unwrap_or(Color::Black);

    // File text color - use theme text or default white
    let file_fg = app_theme.map(|t| t.text).unwrap_or(Color::White);

    // Position info
    let percentage = if total_lines == 0 {
        0
    } else {
        (current_line * 100) / total_lines.max(1)
    };
    let position_text = format!(" {}%/{} ", percentage, total_lines);

    // Position segment background - use theme text_muted or default
    let position_bg = app_theme
        .map(|t| t.text_muted)
        .unwrap_or(Color::Rgb(171, 178, 191));

    // Position text foreground - use theme background or default black
    let position_fg = app_theme.map(|t| t.background).unwrap_or(Color::Black);

    // Build the statusline
    let mut statusline = StatusLineStacked::new()
        // Mode segment (left)
        .start(
            Span::from(mode_text).style(
                Style::new()
                    .fg(mode_fg)
                    .bg(mode_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(SLANT_TL_BR).style(Style::new().fg(mode_color).bg(file_bg)),
        );

    // Filename segment
    if let Some(name) = filename {
        let file_segment = format!(" {} ", name);
        statusline = statusline.start(
            Span::from(file_segment).style(Style::new().fg(file_fg).bg(file_bg)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(file_bg)),
        );
    }

    // Git stats rendered directly after the slant (no background)
    let git_stats_start_x = if filename.is_some() {
        let mode_len = mode_text.len() as u16 + 1;
        let file_len = filename.map(|n| n.len() + 2).unwrap_or(0) as u16 + 1;
        area.x + mode_len + file_len
    } else {
        let mode_len = mode_text.len() as u16 + 1;
        area.x + mode_len
    };

    // Position segment (right)
    statusline = statusline.end(
        Span::from(position_text).style(Style::new().fg(position_fg).bg(position_bg)),
        Span::from(SLANT_BL_TR).style(Style::new().fg(position_bg)),
    );

    // Render the statusline base
    statusline.render(area, buf);

    // Now render git stats with theme colors
    if let Some(stats) = git_stats {
        let green = Style::new().fg(
            app_theme
                .map(|t| t.success)
                .unwrap_or(Color::Rgb(152, 195, 121)),
        );
        let yellow = Style::new().fg(
            app_theme
                .map(|t| t.warning)
                .unwrap_or(Color::Rgb(229, 192, 123)),
        );
        let red = Style::new().fg(
            app_theme
                .map(|t| t.error)
                .unwrap_or(Color::Rgb(224, 108, 117)),
        );
        let dim = Style::new().fg(
            app_theme
                .map(|t| t.text_muted)
                .unwrap_or(Color::Rgb(92, 99, 112)),
        );

        let mut x = git_stats_start_x;

        buf.set_string(x, area.y, "  ", dim);
        x += 2;

        let add_icon = "\u{EADC}";
        let add_num = format!("{}", stats.additions);
        buf.set_string(x, area.y, add_icon, green);
        x += add_icon.width() as u16;
        buf.set_string(x, area.y, " ", green);
        x += 1;
        buf.set_string(x, area.y, &add_num, green);
        x += add_num.width() as u16;
        buf.set_string(x, area.y, " ", dim);
        x += 1;

        let mod_icon = "\u{EADE}";
        let mod_num = format!("{}", stats.modified);
        buf.set_string(x, area.y, mod_icon, yellow);
        x += mod_icon.width() as u16;
        buf.set_string(x, area.y, " ", yellow);
        x += 1;
        buf.set_string(x, area.y, &mod_num, yellow);
        x += mod_num.width() as u16;
        buf.set_string(x, area.y, " ", dim);
        x += 1;

        let del_icon = "\u{EADF}";
        let del_num = format!("{}", stats.deletions);
        buf.set_string(x, area.y, del_icon, red);
        x += del_icon.width() as u16;
        buf.set_string(x, area.y, " ", red);
        x += 1;
        buf.set_string(x, area.y, &del_num, red);
    }
}

/// Render a markdown statusline directly from a scroll manager.
///
/// This is a convenience function that extracts all needed information from
/// the scroll manager and renders the statusline. Use this when you have
/// a `MarkdownScrollManager` and want the simplest API.
///
/// # Arguments
///
/// * `area` - The area to render the statusline in
/// * `buf` - The buffer to render to
/// * `scroll` - The scroll manager containing all state
/// * `selection_active` - Whether text selection mode is active
pub fn render_markdown_statusline_from_scroll(
    area: Rect,
    buf: &mut Buffer,
    scroll: &MarkdownScrollManager,
    selection_active: bool,
) {
    let mode = if selection_active {
        MarkdownWidgetMode::Drag
    } else {
        MarkdownWidgetMode::Normal
    };

    let filename = scroll
        .source_path()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str());

    // Use source_line_count for accurate display (falls back to total_lines if 0)
    let display_total = if scroll.source_line_count > 0 {
        scroll.source_line_count
    } else {
        scroll.total_lines
    };

    render_markdown_statusline(
        area,
        buf,
        mode,
        filename,
        scroll.git_stats(),
        scroll.current_line,
        display_total,
    );
}

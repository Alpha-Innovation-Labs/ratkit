//! Statusline rendering for MarkdownWidget.

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
};
use unicode_width::UnicodeWidthStr;

use crate::primitives::statusline::{StatusLineStacked, SLANT_BL_TR, SLANT_TL_BR};
use crate::widgets::markdown_widget::widget::enums::MarkdownWidgetMode;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Render the statusline using StatusLineStacked (powerline style).
    ///
    /// The statusline displays:
    /// - Mode indicator (NORMAL/DRAG/FILTER) on the left with colored background
    /// - Filename with git stats (no background on git icons)
    /// - Scroll position (percentage/total lines) on the right
    ///
    /// If an app theme is set, uses theme colors. Otherwise falls back to defaults.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the statusline in
    /// * `buf` - The buffer to render to
    pub(crate) fn render_statusline(&self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Get theme-based or default colors
        let (mode_text, mode_color) = match self.mode {
            MarkdownWidgetMode::Normal => {
                let color = self
                    .app_theme
                    .as_ref()
                    .map(|t| t.info)
                    .unwrap_or(Color::Rgb(97, 175, 239)); // blue
                (" NORMAL ".to_string(), color)
            }
            MarkdownWidgetMode::Drag => {
                let color = self
                    .app_theme
                    .as_ref()
                    .map(|t| t.warning)
                    .unwrap_or(Color::Rgb(229, 192, 123)); // yellow/orange
                (" DRAG ".to_string(), color)
            }
            MarkdownWidgetMode::Filter => {
                let color = self
                    .app_theme
                    .as_ref()
                    .map(|t| t.success)
                    .unwrap_or(Color::Rgb(152, 195, 121)); // green
                let filter_text = self.filter.as_deref().unwrap_or("");
                let display_text = format!(" /{} ", filter_text);
                (display_text, color)
            }
        };

        // File segment background - use theme background_panel or default
        let file_bg = self
            .app_theme
            .as_ref()
            .map(|t| t.background_panel)
            .unwrap_or(Color::Rgb(58, 58, 58));

        // Mode text foreground - use theme background or default black
        let mode_fg = self
            .app_theme
            .as_ref()
            .map(|t| t.background)
            .unwrap_or(Color::Black);

        // File text color - use theme text or default white
        let file_fg = self
            .app_theme
            .as_ref()
            .map(|t| t.text)
            .unwrap_or(Color::White);

        // Get filename from source path
        let filename = self
            .source
            .source_path()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());

        // Position info - use source line count for accurate display
        let source_line_count = self.source.line_count();
        let display_total = if source_line_count > 0 {
            source_line_count
        } else {
            self.scroll.total_lines
        };
        let current_line = self.scroll.current_line;
        let percentage = if display_total == 0 {
            0
        } else {
            (current_line * 100) / display_total.max(1)
        };
        let position_text = format!(" {}%/{} ", percentage, display_total);

        // Position segment background - use theme text_muted or default
        let position_bg = self
            .app_theme
            .as_ref()
            .map(|t| t.text_muted)
            .unwrap_or(Color::Rgb(171, 178, 191));

        // Position text foreground - use theme background or default black
        let position_fg = self
            .app_theme
            .as_ref()
            .map(|t| t.background)
            .unwrap_or(Color::Black);

        // Calculate git stats start position
        let git_stats_start_x = {
            let mode_len = mode_text.len() as u16 + 1; // +1 for slant
            let file_len = filename.map(|n| n.len() + 2).unwrap_or(0) as u16 + 1; // +2 for spaces, +1 for slant
            area.x + mode_len + file_len
        };

        // Build the statusline
        let mut statusline = StatusLineStacked::new()
            // Mode segment (left)
            .start(
                Span::from(mode_text.clone()).style(
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

        // Position segment (right)
        statusline = statusline.end(
            Span::from(position_text).style(Style::new().fg(position_fg).bg(position_bg)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(position_bg)),
        );

        // Render the statusline base
        ratatui::widgets::Widget::render(statusline, area, buf);

        // Now render git stats with colored icons (no background)
        // Icons from lvim: LineAdded (U+EADC), LineModified (U+EADE), LineRemoved (U+EADF)
        // Get git stats from git_stats_state (which manages the caching/updates)
        // or fall back to manually-set widget stats
        let git_stats = self.git_stats_state.git_stats().or(self.git_stats);
        if let Some(stats) = &git_stats {
            // Use theme colors for git stats or fall back to defaults
            let green = Style::new().fg(self
                .app_theme
                .as_ref()
                .map(|t| t.success)
                .unwrap_or(Color::Rgb(152, 195, 121)));
            let yellow = Style::new().fg(self
                .app_theme
                .as_ref()
                .map(|t| t.warning)
                .unwrap_or(Color::Rgb(229, 192, 123)));
            let red = Style::new().fg(self
                .app_theme
                .as_ref()
                .map(|t| t.error)
                .unwrap_or(Color::Rgb(224, 108, 117)));
            let dim = Style::new().fg(self
                .app_theme
                .as_ref()
                .map(|t| t.text_muted)
                .unwrap_or(Color::Rgb(92, 99, 112)));

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
}

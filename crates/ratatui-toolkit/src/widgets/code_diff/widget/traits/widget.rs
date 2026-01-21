use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::widgets::code_diff::code_diff::methods::helpers::{
    render_diff_content, render_header, render_side_by_side, render_sidebar,
};
use crate::widgets::code_diff::code_diff::CodeDiff;
use crate::widgets::code_diff::enums::DiffStyle;

impl Widget for CodeDiff {
    /// Renders the diff widget to the given buffer.
    ///
    /// The widget renders:
    /// 1. If sidebar enabled and visible: file tree on left, diff on right
    /// 2. Otherwise: just the diff content
    ///
    /// Each panel includes:
    /// - A header bar with file path and stats (if file path is set)
    /// - Diff hunks in the configured style (side-by-side or unified)
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the widget in
    /// * `buf` - The buffer to render to
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &CodeDiff {
    /// Renders the diff widget from a reference.
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Check if we should show sidebar
        if self.show_sidebar && self.config.sidebar_enabled {
            // Calculate split areas using ResizableSplit's percentage
            let sidebar_width =
                (area.width as u32 * self.sidebar_split.split_percent as u32 / 100) as u16;
            let sidebar_width = sidebar_width.max(1).min(area.width.saturating_sub(10));

            let sidebar_area = Rect {
                x: area.x,
                y: area.y,
                width: sidebar_width,
                height: area.height,
            };

            let diff_area = Rect {
                x: area.x + sidebar_width,
                y: area.y,
                width: area.width.saturating_sub(sidebar_width),
                height: area.height,
            };

            // Render sidebar
            render_sidebar(self, sidebar_area, buf);

            // Render diff content with border
            render_diff_content(self, diff_area, buf, true);
        } else {
            // Just render diff without sidebar
            // Use the old direct rendering for backwards compatibility
            let header_height = render_header(self, area, buf);

            let content_area = Rect {
                x: area.x,
                y: area.y + header_height,
                width: area.width,
                height: area.height.saturating_sub(header_height),
            };

            if content_area.height == 0 {
                return;
            }

            match self.config.style {
                DiffStyle::SideBySide => {
                    render_side_by_side(self, content_area, buf);
                }
                DiffStyle::Unified | DiffStyle::Inline => {
                    render_side_by_side(self, content_area, buf);
                }
            }
        }
    }
}

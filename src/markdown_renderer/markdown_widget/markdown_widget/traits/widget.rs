//! Widget trait implementation for MarkdownWidget.

use ratatui::{layout::Rect, widgets::Widget};

use super::super::MarkdownWidget;
use crate::markdown_renderer::markdown_widget::render_markdown_interactive_with_options;
use crate::markdown_renderer::minimap::Minimap;

impl<'a> Widget for MarkdownWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Reserve space for statusline if enabled
        let (main_area, statusline_area) = if self.show_statusline && area.height > 1 {
            (
                Rect {
                    height: area.height.saturating_sub(1),
                    ..area
                },
                Some(Rect {
                    y: area.y + area.height.saturating_sub(1),
                    height: 1,
                    ..area
                }),
            )
        } else {
            (area, None)
        };

        // Calculate minimap overlay area (small box in top-right corner, overlays content)
        // When hovered, expand the minimap for better visibility
        let hover_scale: u16 = if self.minimap_hovered { 2 } else { 1 };
        let minimap_width = self.minimap_config.width * hover_scale;
        let minimap_height = (self.minimap_config.height * hover_scale).min(main_area.height.saturating_sub(1));
        let padding_right: u16 = 2;
        let padding_top: u16 = 1;
        let content_area = main_area;
        let minimap_area = if self.show_minimap && main_area.width > minimap_width + padding_right + 2 {
            Some(Rect {
                x: main_area.x + main_area.width.saturating_sub(minimap_width + padding_right),
                y: main_area.y + padding_top,
                width: minimap_width,
                height: minimap_height,
            })
        } else {
            None
        };

        self.scroll.update_viewport(content_area);

        let text = render_markdown_interactive_with_options(
            self.content,
            self.scroll,
            content_area,
            self.is_resizing,
        );

        // Render markdown content
        for (i, line) in text.lines.iter().enumerate() {
            if i < content_area.height as usize {
                let y = content_area.y + i as u16;
                let mut x = content_area.x;
                for span in line.spans.iter() {
                    let span_width = span.content.chars().count() as u16;
                    if x.saturating_sub(content_area.x) < content_area.width {
                        buf.set_string(x, y, &span.content, span.style);
                        x = x.saturating_add(span_width);
                    }
                }
            }
        }

        // Render minimap
        if let Some(mm_area) = minimap_area {
            let viewport_start = self.scroll.scroll_offset;
            let viewport_end = viewport_start + content_area.height as usize;
            let total_lines = self.scroll.total_lines;

            let minimap = Minimap::new(self.content)
                .width(mm_area.width)
                .viewport(viewport_start, viewport_end, total_lines)
                .config(self.minimap_config.clone());

            minimap.render(mm_area, buf);
        }

        // Render statusline
        if let Some(sl_area) = statusline_area {
            self.render_statusline(sl_area, buf);
        }
    }
}

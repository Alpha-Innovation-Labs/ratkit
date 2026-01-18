//! Widget trait implementation for MarkdownWidget.

use ratatui::{layout::Rect, widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget}};

use super::super::MarkdownWidget;
use crate::markdown_renderer::markdown_widget::render_markdown_interactive_with_selection::render_markdown_interactive_with_selection_themed;
use crate::markdown_renderer::minimap::Minimap;
use crate::markdown_renderer::toc::Toc;

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

        let padding_right: u16 = 2;
        let padding_top: u16 = 1;
        let content_area = main_area;

        // Calculate overlay area for TOC or minimap
        let overlay_area = if self.show_toc {
            // TOC: compact when not hovered, expanded when hovered
            // Dynamic width based on content for expanded mode
            let toc_width = if self.toc_hovered {
                Toc::required_expanded_width(self.content, self.toc_config.show_border)
                    .min(main_area.width.saturating_sub(padding_right + 4))
            } else {
                self.toc_config.compact_width
            };
            // Dynamic height based on content
            let toc_height = if self.toc_hovered {
                // Expanded: one row per entry
                Toc::required_height(self.content, self.toc_config.show_border)
                    .min(main_area.height.saturating_sub(1))
            } else {
                // Compact: based on entries and line_spacing
                Toc::required_compact_height(
                    self.content,
                    self.toc_config.line_spacing,
                    self.toc_config.show_border,
                )
                .min(main_area.height.saturating_sub(1))
            };

            if main_area.width > toc_width + padding_right + 2 {
                Some(Rect {
                    x: main_area.x + main_area.width.saturating_sub(toc_width + padding_right),
                    y: main_area.y + padding_top,
                    width: toc_width,
                    height: toc_height,
                })
            } else {
                None
            }
        } else if self.show_minimap {
            // Minimap: scale up when hovered
            let hover_scale: u16 = if self.minimap_hovered { 2 } else { 1 };
            let minimap_width = self.minimap_config.width * hover_scale;
            let minimap_height =
                (self.minimap_config.height * hover_scale).min(main_area.height.saturating_sub(1));

            if main_area.width > minimap_width + padding_right + 2 {
                Some(Rect {
                    x: main_area.x
                        + main_area
                            .width
                            .saturating_sub(minimap_width + padding_right),
                    y: main_area.y + padding_top,
                    width: minimap_width,
                    height: minimap_height,
                })
            } else {
                None
            }
        } else {
            None
        };

        self.scroll.update_viewport(content_area);

        // Render markdown with selection highlighting
        let (text, _all_lines) = render_markdown_interactive_with_selection_themed(
            self.content,
            self.scroll,
            content_area,
            self.is_resizing,
            self.selection,
            self.app_theme,
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

        // Render scrollbar if enabled
        if self.show_scrollbar && self.scroll.total_lines > content_area.height as usize {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None);
            let mut scrollbar_state = ScrollbarState::new(self.scroll.total_lines)
                .position(self.scroll.scroll_offset)
                .viewport_content_length(content_area.height as usize);
            StatefulWidget::render(scrollbar, content_area, buf, &mut scrollbar_state);
        }

        // Render TOC or minimap overlay
        if let Some(ov_area) = overlay_area {
            if self.show_toc {
                // Render TOC
                let viewport_start = self.scroll.scroll_offset;
                let viewport_height = content_area.height as usize;
                let total_lines = self.scroll.total_lines;

                let toc = Toc::new(self.content)
                    .expanded(self.toc_hovered)
                    .viewport(viewport_start, viewport_height, total_lines)
                    .hovered(self.toc_hovered_entry)
                    .toc_scroll(self.toc_scroll_offset)
                    .config(self.toc_config.clone());

                toc.render(ov_area, buf);
            } else if self.show_minimap {
                // Render minimap
                let viewport_start = self.scroll.scroll_offset;
                let viewport_end = viewport_start + content_area.height as usize;
                let total_lines = self.scroll.total_lines;

                let minimap = Minimap::new(self.content)
                    .width(ov_area.width)
                    .viewport(viewport_start, viewport_end, total_lines)
                    .config(self.minimap_config.clone());

                minimap.render(ov_area, buf);
            }
        }

        // Render statusline
        if let Some(sl_area) = statusline_area {
            self.render_statusline(sl_area, buf);
        }
    }
}

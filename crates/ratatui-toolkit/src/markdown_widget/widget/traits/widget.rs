//! Widget trait implementation for MarkdownWidget.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

use crate::markdown_widget::extensions::scrollbar::CustomScrollbar;
use crate::markdown_widget::extensions::selection::should_render_line;
use crate::markdown_widget::extensions::toc::Toc;
use crate::markdown_widget::foundation::elements::{render_with_options, RenderOptions};
use crate::markdown_widget::foundation::helpers::hash_content;
use crate::markdown_widget::foundation::parser::render_markdown_to_elements;
use crate::markdown_widget::state::toc_state::TocState;
use crate::markdown_widget::state::{ParsedCache, RenderCache};
use crate::markdown_widget::widget::helpers::apply_selection_highlighting;
use crate::markdown_widget::widget::MarkdownWidget;
use crate::primitives::pane::Pane;

/// Current line highlight color (normal - dark blue-gray)
const CURRENT_LINE_BG: Color = Color::Rgb(38, 52, 63);
/// Current line highlight color when dragging (selection is active)
const CURRENT_LINE_DRAG_BG: Color = Color::Rgb(70, 80, 100);

impl<'a> Widget for MarkdownWidget<'a> {
    fn render(mut self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Handle pane wrapping if enabled
        let (area, _pane_footer_area) = if self.has_pane {
            let title = self
                .pane_title
                .clone()
                .unwrap_or_else(|| "Markdown".to_string());
            let pane = self.pane.take().unwrap_or_else(|| {
                let mut p = Pane::new(title);
                if let Some(color) = self.pane_color {
                    p = p.border_style(ratatui::style::Style::default().fg(color));
                }
                p
            });

            // Build the block
            let mut block = Block::default()
                .borders(Borders::ALL)
                .border_type(pane.border_type)
                .border_style(pane.border_style)
                .title(pane.title);

            if let Some(icon) = &pane.icon {
                use ratatui::text::Span;
                let title = format!(" {} ", icon);
                block = block.title(Line::from(vec![Span::styled(
                    title,
                    pane.title_style.clone(),
                )]));
            }

            if let Some(ref footer) = pane.text_footer {
                block = block.title_bottom(footer.clone().style(pane.footer_style));
            }

            // Get the inner area (inside the border) BEFORE rendering
            let inner = block.inner(area);

            // Render the block
            block.render(area, buf);

            // Calculate footer area if needed
            let (inner, pane_footer) = if pane.footer_height > 0 {
                let chunks = ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints([
                        ratatui::layout::Constraint::Min(0),
                        ratatui::layout::Constraint::Length(pane.footer_height),
                    ])
                    .split(inner);
                (chunks[0], Some(chunks[1]))
            } else {
                (inner, None)
            };

            // Calculate padded inner area
            let padded = Rect {
                x: inner.x + pane.padding.3,
                y: inner.y + pane.padding.0,
                width: inner.width.saturating_sub(pane.padding.1 + pane.padding.3),
                height: inner.height.saturating_sub(pane.padding.0 + pane.padding.2),
            };

            // Render footer if exists
            if let Some(footer_area) = pane_footer {
                if let Some(ref footer) = pane.text_footer {
                    footer.render(footer_area, buf);
                }
            }

            (padded, None::<Rect>)
        } else {
            (area, None::<Rect>)
        };

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

        // Calculate overlay area for TOC
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
        } else {
            None
        };

        self.scroll.update_viewport(content_area);

        // Calculate line number width if document line numbers are enabled
        // Fixed width of 6 chars: "  1 │ " to "999 │ " covers most documents
        let line_num_width = if self.display.show_document_line_numbers {
            6
        } else {
            0
        };

        // Render markdown content (subtract line number width from available width)
        let width = (content_area.width as usize).saturating_sub(line_num_width);
        let content_hash = hash_content(self.content);
        let show_line_numbers = self.display.show_line_numbers;
        let theme = self.display.code_block_theme;

        // Hash app theme for cache invalidation
        let app_theme_hash = self
            .app_theme
            .map(|t| {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                format!(
                    "{:?}{:?}{:?}{:?}{:?}",
                    t.primary, t.text, t.background, t.markdown.heading, t.markdown.code
                )
                .hash(&mut hasher);
                hasher.finish()
            })
            .unwrap_or(0);

        // Check if we can use fully cached rendered lines
        // Note: Never use cache when in filter mode (filter affects what's shown)
        let show_heading_collapse = self.display.show_heading_collapse;
        let render_cache_valid = !self.filter_mode
            && self
                .cache
                .render
                .as_ref()
                .map(|c| {
                    c.content_hash == content_hash
                        && c.width == width
                        && c.show_line_numbers == show_line_numbers
                        && c.theme == theme
                        && c.app_theme_hash == app_theme_hash
                        && c.show_heading_collapse == show_heading_collapse
                })
                .unwrap_or(false);

        // Get rendered lines and boundaries (from cache or fresh render)
        let (all_lines, line_boundaries): (Vec<Line<'static>>, Vec<(usize, usize)>) =
            if render_cache_valid {
                // Use fully cached rendered lines
                let cache = self.cache.render.as_ref().unwrap();
                (cache.lines.clone(), cache.line_boundaries.clone())
            } else {
                // Check if we can use cached parsed elements
                let parsed_cache_valid = self
                    .cache
                    .parsed
                    .as_ref()
                    .map(|c| c.content_hash == content_hash)
                    .unwrap_or(false);

                let elements = if parsed_cache_valid {
                    // Use cached parsed elements
                    self.cache.parsed.as_ref().unwrap().elements.clone()
                } else {
                    // Parse markdown and cache
                    let parsed = render_markdown_to_elements(self.content, true);
                    self.cache.parsed = Some(ParsedCache {
                        content_hash,
                        elements: parsed.clone(),
                    });
                    parsed
                };

                let render_options = RenderOptions {
                    show_line_numbers,
                    theme,
                    app_theme: self.app_theme,
                    show_heading_collapse: self.display.show_heading_collapse,
                };

                // Get filter text if in filter mode
                let filter_lower = self
                    .filter_mode
                    .then(|| self.filter.as_deref().unwrap_or("").to_lowercase());

                // Build all rendered lines and track line boundaries
                let mut lines: Vec<Line<'static>> = Vec::new();
                let mut boundaries: Vec<(usize, usize)> = Vec::new();

                for (idx, element) in elements.iter().enumerate() {
                    if !should_render_line(element, idx, self.collapse) {
                        continue;
                    }

                    // Check if element matches filter (skip if not matching)
                    if let Some(ref filter) = filter_lower {
                        let text =
                            super::super::helpers::element_to_plain_text_for_filter(&element.kind)
                                .to_lowercase();
                        if !text.contains(filter) {
                            continue;
                        }
                    }

                    let start_idx = lines.len();
                    let rendered = render_with_options(element, width, render_options);
                    let line_count = rendered.len();
                    lines.extend(rendered);
                    boundaries.push((start_idx, line_count));
                }

                // Cache the rendered lines
                self.cache.render = Some(RenderCache {
                    content_hash,
                    width,
                    show_line_numbers,
                    theme,
                    app_theme_hash,
                    show_heading_collapse,
                    lines: lines.clone(),
                    line_boundaries: boundaries.clone(),
                });

                (lines, boundaries)
            };

        // Update total lines
        self.scroll.update_total_lines(all_lines.len());

        // Update cache for selection text extraction
        self.rendered_lines = all_lines.clone();

        // Extract visible portion
        let start = self.scroll.scroll_offset.min(all_lines.len());
        let end = (self.scroll.scroll_offset + content_area.height as usize).min(all_lines.len());
        let visible_lines: Vec<Line<'static>> = all_lines[start..end].to_vec();

        // Apply selection highlighting if selection is active
        let visible_lines = if self.selection_active {
            apply_selection_highlighting(visible_lines, self.selection, self.scroll.scroll_offset)
        } else {
            visible_lines
        };

        // Current line for highlighting (0-indexed visual line)
        let current_visual_line = self.scroll.current_line.saturating_sub(1);

        // Add document line numbers if enabled and apply current line highlighting
        let final_lines: Vec<Line<'_>> = if self.display.show_document_line_numbers {
            // Get colors from the code block theme for consistency
            let theme_colors = self.display.code_block_theme.colors();
            let line_num_style = Style::default()
                .fg(theme_colors.line_number)
                .bg(theme_colors.background);
            let border_style = Style::default()
                .fg(theme_colors.border)
                .bg(theme_colors.background);

            // Build a map: visual_line_idx -> (logical_line_num, is_first_line_of_logical)
            let mut visual_to_logical: Vec<(usize, bool)> = Vec::with_capacity(all_lines.len());
            for (logical_idx, (_start_idx, count)) in line_boundaries.iter().enumerate() {
                for offset in 0..*count {
                    let is_first = offset == 0;
                    visual_to_logical.push((logical_idx + 1, is_first));
                }
            }

            visible_lines
                .into_iter()
                .enumerate()
                .map(|(i, mut line)| {
                    let visual_idx = start + i;
                    let is_current = visual_idx == current_visual_line;
                    let (logical_num, is_first) = visual_to_logical
                        .get(visual_idx)
                        .copied()
                        .unwrap_or((visual_idx + 1, true));

                    // Fixed width of 3 digits + " │ " = 6 chars total
                    let (num_str, border_str) = if is_first {
                        (format!("{:>3} ", logical_num), "│ ".to_string())
                    } else {
                        ("    ".to_string(), "│ ".to_string()) // Continuation line
                    };

                    let num_span = Span::styled(num_str, line_num_style);
                    let border_span = Span::styled(border_str, border_style);

                    let mut new_spans = vec![num_span, border_span];

                    // Apply current line highlighting to content
                    let highlight_bg = if self.selection_active {
                        CURRENT_LINE_DRAG_BG
                    } else {
                        CURRENT_LINE_BG
                    };
                    if is_current {
                        let mut content_width = 0usize;
                        for span in line.spans.drain(..) {
                            content_width += span.content.chars().count();
                            if span.content.contains('▋') {
                                // Keep blockquote marker without highlight
                                new_spans.push(span);
                            } else {
                                new_spans
                                    .push(Span::styled(span.content, span.style.bg(highlight_bg)));
                            }
                        }
                        // Add padding to fill the rest of the line
                        let total_content_width = line_num_width + content_width;
                        if total_content_width < content_area.width as usize {
                            let padding =
                                " ".repeat(content_area.width as usize - total_content_width);
                            new_spans
                                .push(Span::styled(padding, Style::default().bg(highlight_bg)));
                        }
                    } else {
                        new_spans.extend(line.spans.drain(..));
                    }

                    Line::from(new_spans)
                })
                .collect()
        } else {
            // No line numbers, but still apply current line highlighting
            let highlight_bg = if self.selection_active {
                CURRENT_LINE_DRAG_BG
            } else {
                CURRENT_LINE_BG
            };
            visible_lines
                .into_iter()
                .enumerate()
                .map(|(i, mut line)| {
                    let visual_idx = start + i;
                    let is_current = visual_idx == current_visual_line;

                    if is_current {
                        let mut new_spans = Vec::new();
                        let mut content_width = 0usize;
                        for span in line.spans.drain(..) {
                            content_width += span.content.chars().count();
                            if span.content.contains('▋') {
                                new_spans.push(span);
                            } else {
                                new_spans
                                    .push(Span::styled(span.content, span.style.bg(highlight_bg)));
                            }
                        }
                        // Add padding to fill the rest of the line
                        if content_width < content_area.width as usize {
                            let padding = " ".repeat(content_area.width as usize - content_width);
                            new_spans
                                .push(Span::styled(padding, Style::default().bg(highlight_bg)));
                        }
                        Line::from(new_spans)
                    } else {
                        line
                    }
                })
                .collect()
        };

        // Render markdown content to buffer
        for (i, line) in final_lines.iter().enumerate() {
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

        // Render TOC overlay
        if let Some(ov_area) = overlay_area {
            // Create state from content with widget's hover state
            let mut auto_state = TocState::from_content(self.content);
            auto_state.hovered = self.toc_hovered;
            auto_state.hovered_entry = self.toc_hovered_entry;
            auto_state.scroll_offset = self.toc_scroll_offset;

            // Use provided state if it has entries, otherwise use auto-extracted
            let final_state = if let Some(provided) = self.toc_state {
                if provided.entries.is_empty() {
                    &auto_state
                } else {
                    provided
                }
            } else {
                &auto_state
            };

            let toc = Toc::new(final_state)
                .expanded(self.toc_hovered)
                .config(self.toc_config.clone());

            toc.render(ov_area, buf);
        }

        // Render statusline
        if let Some(sl_area) = statusline_area {
            self.render_statusline(sl_area, buf);
        }

        // Render scrollbar LAST so it's on top of everything
        if self.show_scrollbar && self.scroll.total_lines > content_area.height as usize {
            let scrollbar_width = self.scrollbar_config.width;
            let scrollbar_area = Rect {
                x: content_area.x + content_area.width.saturating_sub(scrollbar_width),
                y: content_area.y,
                width: scrollbar_width,
                height: content_area.height,
            };

            let scrollbar = CustomScrollbar::new(self.scroll)
                .config(self.scrollbar_config.clone())
                .show_percentage(false);

            scrollbar.render(scrollbar_area, buf);
        }
    }
}

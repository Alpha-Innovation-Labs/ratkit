//! Render markdown with interactive scroll and collapse state, with resize optimization.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
};

use crate::markdown_renderer::markdown_elements::methods::render::RenderOptions;
use crate::markdown_renderer::markdown_elements::ElementKind;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

use super::helpers::{hash_content, should_render_line};

/// Render markdown with interactive scroll and collapse state, with resize optimization.
///
/// # Arguments
///
/// * `content` - The markdown content to render
/// * `scroll` - The scroll manager
/// * `area` - The area to render into
/// * `_is_resizing` - Whether the widget is being resized (unused)
/// * `app_theme` - Optional application theme for consistent styling
///
/// # Returns
///
/// The rendered text.
pub fn render_markdown_interactive_with_options(
    content: &str,
    scroll: &mut MarkdownScrollManager,
    area: Rect,
    _is_resizing: bool,
    app_theme: Option<&crate::services::theme::AppTheme>,
) -> Text<'static> {
    // Calculate line number width if document line numbers are enabled
    // Use fixed width of 6 chars: "  1 │ " to "999 │ " covers most documents
    let line_num_width = if scroll.show_document_line_numbers {
        6
    } else {
        0
    };

    let width = (area.width as usize).saturating_sub(line_num_width);
    let inner_height = area.height as usize;
    let content_hash = hash_content(content);
    let show_line_numbers = scroll.show_line_numbers;
    let theme = scroll.code_block_theme;

    // Hash key parts of the app_theme for cache invalidation when theme changes
    // We hash a few representative colors to detect theme changes
    let app_theme_hash = app_theme
        .map(|t| {
            use std::hash::{Hash, Hasher};
            use std::collections::hash_map::DefaultHasher;
            let mut hasher = DefaultHasher::new();
            // Hash a few key colors that are likely different between themes
            format!("{:?}{:?}{:?}{:?}{:?}",
                t.primary, t.text, t.background, t.markdown.heading, t.markdown.code
            ).hash(&mut hasher);
            hasher.finish()
        })
        .unwrap_or(0);

    // Check if we can use fully cached rendered lines
    let render_cache_valid = scroll
        .render_cache
        .as_ref()
        .map(|c| {
            c.content_hash == content_hash
                && c.width == width
                && c.show_line_numbers == show_line_numbers
                && c.theme == theme
                && c.app_theme_hash == app_theme_hash
        })
        .unwrap_or(false);

    // Track line boundaries for proper line numbering (logical line -> visual lines)
    let (all_rendered_lines, line_boundaries) = if render_cache_valid {
        // Use fully cached rendered lines (same content AND same width)
        let cache = scroll.render_cache.as_ref().unwrap();
        (cache.lines.clone(), cache.line_boundaries.clone())
    } else {
        // Check if we can use cached parsed lines (parsing is expensive)
        let parsed_cache_valid = scroll
            .parsed_cache
            .as_ref()
            .map(|c| c.content_hash == content_hash)
            .unwrap_or(false);

        let mut elements = if parsed_cache_valid {
            // Use cached parsed elements - skip expensive parsing
            scroll.parsed_cache.as_ref().unwrap().elements.clone()
        } else {
            // Parse markdown and cache
            let parsed = crate::markdown_renderer::render_markdown_to_elements(content, true);
            scroll.parsed_cache = Some(crate::markdown_renderer::scroll_manager::ParsedCache {
                content_hash,
                elements: parsed.clone(),
            });
            parsed
        };

        // Build section hierarchy if not already built
        // This allows hierarchical collapse (H1 collapses all H2s under it, etc.)
        if scroll.section_hierarchy.is_empty() {
            // Stack of (level, section_id) for tracking parent sections
            let mut section_stack: Vec<(u8, usize)> = Vec::new();

            for element in &elements {
                if let ElementKind::Heading {
                    level, section_id, ..
                } = &element.kind
                {
                    // Pop sections with level >= current level (they can't be parents)
                    while section_stack
                        .last()
                        .map(|(l, _)| *l >= *level)
                        .unwrap_or(false)
                    {
                        section_stack.pop();
                    }

                    // The parent is now at the top of the stack (if any)
                    let parent_id = section_stack.last().map(|(_, id)| *id);

                    // Register this section with its parent
                    scroll.register_section(*section_id, *level, parent_id);

                    // Push this section onto the stack
                    section_stack.push((*level, *section_id));
                }
            }
        }

        // Update collapse states from scroll manager before rendering
        for element in &mut elements {
            match &mut element.kind {
                ElementKind::Heading {
                    section_id,
                    collapsed,
                    ..
                } => {
                    *collapsed = scroll.is_section_collapsed(*section_id);
                }
                ElementKind::Frontmatter { collapsed, .. } => {
                    *collapsed = scroll.is_section_collapsed(0);
                }
                ElementKind::FrontmatterStart { collapsed, .. } => {
                    *collapsed = scroll.is_section_collapsed(0);
                }
                _ => {}
            }
        }

        // Render from markdown elements (this is the part that depends on width)
        let render_options = RenderOptions {
            show_line_numbers,
            theme,
            app_theme,
        };

        let mut lines = Vec::new();
        let mut boundaries = Vec::new(); // (start_idx, line_count) for each logical line
        let mut max_source_line: usize = 0;

        for (idx, element) in elements.iter().enumerate() {
            // Track max source line for accurate total count
            max_source_line = max_source_line.max(element.source_line);

            if should_render_line(element, idx, scroll) {
                let start_idx = lines.len();
                let rendered =
                    crate::markdown_renderer::markdown_elements::methods::render::render_with_options(
                        element,
                        width,
                        render_options,
                    );
                let line_count = rendered.len();
                lines.extend(rendered);
                boundaries.push((start_idx, line_count));
            }
        }

        // Store max source line in scroll manager for status bar
        scroll.source_line_count = max_source_line;

        // Cache the rendered lines
        scroll.render_cache = Some(crate::markdown_renderer::scroll_manager::RenderCache {
            content_hash,
            width,
            show_line_numbers,
            theme,
            app_theme_hash,
            lines: lines.clone(),
            line_boundaries: boundaries.clone(),
        });

        (lines, boundaries)
    };

    // Update total lines with actual rendered line count
    scroll.update_total_lines(all_rendered_lines.len());

    // Extract visible portion
    let start = scroll.scroll_offset.min(all_rendered_lines.len());
    let end = (scroll.scroll_offset + inner_height).min(all_rendered_lines.len());
    let visible_lines: Vec<Line<'static>> = all_rendered_lines[start..end].to_vec();

    // Check if a visual line is the current line
    let current_visual_line = scroll.current_line.saturating_sub(1); // Convert to 0-indexed

    // Add document line numbers if enabled and apply current line highlighting
    let final_lines: Vec<Line<'_>> = if scroll.show_document_line_numbers {
        // Get colors from the code block theme for consistency
        let theme_colors = theme.colors();
        let line_num_style = Style::default()
            .fg(theme_colors.line_number)
            .bg(theme_colors.background);
        let border_style = Style::default()
            .fg(theme_colors.border)
            .bg(theme_colors.background);

        // Build a map: visual_line_idx -> (logical_line_num, is_first_line_of_logical)
        let mut visual_to_logical: Vec<(usize, bool)> =
            Vec::with_capacity(all_rendered_lines.len());
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

                // Line number and border bar NEVER get highlighted - only content does
                let num_style = line_num_style;
                let bord_style = border_style;

                let num_span = Span::styled(num_str, num_style);
                let border_span = Span::styled(border_str, bord_style);

                let mut new_spans = vec![num_span, border_span];

                // Apply current line highlighting to content
                // Skip blockquote markers (▋) to keep them visually distinct
                if is_current {
                    let highlight_bg = Color::Rgb(38, 52, 63);
                    let mut content_width = 0usize;
                    for span in line.spans.drain(..) {
                        content_width += span.content.chars().count();
                        if span.content.contains('▋') {
                            // Keep blockquote marker without highlight
                            new_spans.push(span);
                        } else {
                            new_spans.push(Span::styled(
                                span.content,
                                span.style.bg(highlight_bg),
                            ));
                        }
                    }
                    // Add padding to fill the rest of the line (account for line number width)
                    let total_content_width = line_num_width + content_width;
                    if total_content_width < area.width as usize {
                        let padding = " ".repeat(area.width as usize - total_content_width);
                        new_spans.push(Span::styled(padding, Style::default().bg(highlight_bg)));
                    }
                } else {
                    new_spans.extend(line.spans.drain(..));
                }

                Line::from(new_spans)
            })
            .collect()
    } else {
        // No line numbers, but still apply current line highlighting
        // Skip blockquote markers (▋) to keep them visually distinct
        visible_lines
            .into_iter()
            .enumerate()
            .map(|(i, mut line)| {
                let visual_idx = start + i;
                let is_current = visual_idx == current_visual_line;

                if is_current {
                    let highlight_bg = Color::Rgb(38, 52, 63);
                    let mut content_width = 0usize;
                    let mut new_spans = Vec::new();
                    for span in line.spans.drain(..) {
                        content_width += span.content.chars().count();
                        if span.content.contains('▋') {
                            new_spans.push(span);
                        } else {
                            new_spans.push(Span::styled(span.content, span.style.bg(highlight_bg)));
                        }
                    }
                    // Add padding to fill the rest of the line
                    if content_width < area.width as usize {
                        let padding = " ".repeat(area.width as usize - content_width);
                        new_spans.push(Span::styled(padding, Style::default().bg(highlight_bg)));
                    }
                    Line::from(new_spans)
                } else {
                    line
                }
            })
            .collect()
    };

    Text::from(final_lines)
}

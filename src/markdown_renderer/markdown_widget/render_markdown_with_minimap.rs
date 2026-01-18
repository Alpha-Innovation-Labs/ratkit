//! Render markdown with optional minimap/TOC and statusline.

use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};

use crate::markdown_renderer::minimap::{Minimap, MinimapConfig};
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;
use crate::markdown_renderer::toc::{Toc, TocConfig};
use crate::theme::AppTheme;

use super::markdown_widget::MarkdownWidgetMode;
use super::render_markdown_interactive_with_selection::render_markdown_interactive_with_selection_themed;
use super::render_markdown_statusline::render_markdown_statusline_themed;
use super::selection_state::SelectionState;

/// Options for rendering markdown with minimap/TOC and statusline.
#[derive(Debug, Clone)]
pub struct MarkdownRenderOptions<'a> {
    /// Whether to show the minimap.
    pub show_minimap: bool,
    /// Minimap configuration.
    pub minimap_config: MinimapConfig,
    /// Whether the minimap is currently hovered (expands when hovered).
    pub minimap_hovered: bool,
    /// Whether to show the TOC (replaces minimap when enabled).
    pub show_toc: bool,
    /// TOC configuration.
    pub toc_config: TocConfig,
    /// Whether the TOC is currently hovered (expands to show text).
    pub toc_hovered: bool,
    /// Index of the hovered TOC entry.
    pub toc_hovered_entry: Option<usize>,
    /// Scroll offset for the TOC list.
    pub toc_scroll_offset: usize,
    /// Whether to show the statusline at the bottom.
    pub show_statusline: bool,
    /// Whether selection mode is active (affects statusline mode display).
    pub selection_active: bool,
    /// Optional application theme for consistent styling.
    pub app_theme: Option<&'a AppTheme>,
}

impl Default for MarkdownRenderOptions<'_> {
    fn default() -> Self {
        Self {
            show_minimap: false,
            minimap_config: MinimapConfig::default(),
            minimap_hovered: false,
            show_toc: false,
            toc_config: TocConfig::default(),
            toc_hovered: false,
            toc_hovered_entry: None,
            toc_scroll_offset: 0,
            show_statusline: false,
            selection_active: false,
            app_theme: None,
        }
    }
}

impl<'a> MarkdownRenderOptions<'a> {
    /// Create new options with minimap enabled.
    pub fn with_minimap() -> Self {
        Self {
            show_minimap: true,
            ..Default::default()
        }
    }

    /// Create new options with TOC enabled.
    pub fn with_toc() -> Self {
        Self {
            show_toc: true,
            ..Default::default()
        }
    }

    /// Set minimap visibility.
    pub fn show_minimap(mut self, show: bool) -> Self {
        self.show_minimap = show;
        self
    }

    /// Set minimap width.
    pub fn minimap_width(mut self, width: u16) -> Self {
        self.minimap_config.width = width;
        self
    }

    /// Set minimap height.
    pub fn minimap_height(mut self, height: u16) -> Self {
        self.minimap_config.height = height;
        self
    }

    /// Set minimap configuration.
    pub fn minimap_config(mut self, config: MinimapConfig) -> Self {
        self.minimap_config = config;
        self
    }

    /// Set minimap hovered state (expands minimap when hovered).
    pub fn minimap_hovered(mut self, hovered: bool) -> Self {
        self.minimap_hovered = hovered;
        self
    }

    /// Set TOC visibility.
    pub fn show_toc(mut self, show: bool) -> Self {
        self.show_toc = show;
        self
    }

    /// Set TOC configuration.
    pub fn toc_config(mut self, config: TocConfig) -> Self {
        self.toc_config = config;
        self
    }

    /// Set TOC hovered state (expands TOC to show text).
    pub fn toc_hovered(mut self, hovered: bool) -> Self {
        self.toc_hovered = hovered;
        self
    }

    /// Set the hovered TOC entry index.
    pub fn toc_hovered_entry(mut self, index: Option<usize>) -> Self {
        self.toc_hovered_entry = index;
        self
    }

    /// Set the TOC scroll offset.
    pub fn toc_scroll_offset(mut self, offset: usize) -> Self {
        self.toc_scroll_offset = offset;
        self
    }

    /// Set statusline visibility.
    pub fn show_statusline(mut self, show: bool) -> Self {
        self.show_statusline = show;
        self
    }

    /// Set selection active state (for statusline mode display).
    pub fn selection_active(mut self, active: bool) -> Self {
        self.selection_active = active;
        self
    }

    /// Set the application theme for consistent styling.
    ///
    /// When a theme is set, the minimap, TOC, and statusline will use
    /// theme-derived colors instead of hardcoded defaults.
    pub fn with_theme(mut self, theme: &'a AppTheme) -> Self {
        self.app_theme = Some(theme);
        // Also update configs with theme colors
        self.minimap_config = self.minimap_config.with_theme(theme);
        self.toc_config = self.toc_config.with_theme(theme);
        self
    }
}

/// Render markdown with selection and optional minimap/TOC directly to buffer.
///
/// This function handles the complete rendering including:
/// - Markdown content with selection highlighting
/// - Optional minimap or TOC overlay in the top-right corner
/// - Optional statusline at the bottom
///
/// # Arguments
///
/// * `content` - The markdown content to render
/// * `scroll` - The scroll manager
/// * `area` - The area to render into
/// * `buf` - The buffer to render to
/// * `is_resizing` - Whether the widget is being resized
/// * `selection` - The selection state
/// * `options` - Render options including minimap/TOC and statusline settings
///
/// # Returns
///
/// All rendered lines for selection text extraction.
pub fn render_markdown_with_minimap(
    content: &str,
    scroll: &mut MarkdownScrollManager,
    area: Rect,
    buf: &mut Buffer,
    is_resizing: bool,
    selection: &SelectionState,
    options: &MarkdownRenderOptions<'_>,
) -> Vec<Line<'static>> {
    // Reserve space for statusline if enabled
    let (main_area, statusline_area) = if options.show_statusline && area.height > 1 {
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
    let overlay_area = if options.show_toc {
        // TOC: compact when not hovered, expanded when hovered
        // Dynamic width based on content for expanded mode
        let toc_width = if options.toc_hovered {
            Toc::required_expanded_width(content, options.toc_config.show_border)
                .min(main_area.width.saturating_sub(padding_right + 4))
        } else {
            options.toc_config.compact_width
        };
        // Dynamic height based on content
        let toc_height = if options.toc_hovered {
            Toc::required_height(content, options.toc_config.show_border)
                .min(main_area.height.saturating_sub(1))
        } else {
            Toc::required_compact_height(
                content,
                options.toc_config.line_spacing,
                options.toc_config.show_border,
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
    } else if options.show_minimap {
        // Minimap: scale up when hovered
        let hover_scale: u16 = if options.minimap_hovered { 2 } else { 1 };
        let minimap_width = options.minimap_config.width * hover_scale;
        let minimap_height =
            (options.minimap_config.height * hover_scale).min(main_area.height.saturating_sub(1));

        if main_area.width > minimap_width + padding_right + 10 {
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

    // Update viewport height for scroll calculations
    scroll.update_viewport(content_area);

    // Render markdown content with optional theme
    let (text, all_lines) = render_markdown_interactive_with_selection_themed(
        content,
        scroll,
        content_area,
        is_resizing,
        selection,
        options.app_theme,
    );

    // Render content to buffer
    for (i, line) in text.lines.iter().enumerate() {
        if i >= content_area.height as usize {
            break;
        }

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

    // Render TOC or minimap overlay
    if let Some(ov_area) = overlay_area {
        if options.show_toc {
            // Render TOC
            let viewport_start = scroll.scroll_offset;
            let viewport_height = content_area.height as usize;
            let total_lines = scroll.total_lines;

            let toc = Toc::new(content)
                .expanded(options.toc_hovered)
                .viewport(viewport_start, viewport_height, total_lines)
                .hovered(options.toc_hovered_entry)
                .toc_scroll(options.toc_scroll_offset)
                .config(options.toc_config.clone());

            toc.render(ov_area, buf);
        } else if options.show_minimap {
            // Render minimap
            let viewport_start = scroll.scroll_offset;
            let viewport_end = viewport_start + content_area.height as usize;
            let total_lines = scroll.total_lines;

            let minimap = Minimap::new(content)
                .width(ov_area.width)
                .viewport(viewport_start, viewport_end, total_lines)
                .config(options.minimap_config.clone());

            minimap.render(ov_area, buf);
        }
    }

    // Render statusline if enabled
    if let Some(sl_area) = statusline_area {
        let mode = if options.selection_active {
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

        render_markdown_statusline_themed(
            sl_area,
            buf,
            mode,
            filename,
            scroll.git_stats(),
            scroll.current_line,
            display_total,
            options.app_theme,
        );
    }

    all_lines
}

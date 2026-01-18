//! Render markdown with optional minimap and statusline.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::Widget,
};

use crate::markdown_renderer::minimap::{Minimap, MinimapConfig};
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

use super::markdown_widget::MarkdownWidgetMode;
use super::render_markdown_interactive_with_selection::render_markdown_interactive_with_selection;
use super::render_markdown_statusline::render_markdown_statusline;
use super::selection_state::SelectionState;

/// Options for rendering markdown with minimap and statusline.
#[derive(Debug, Clone)]
pub struct MarkdownRenderOptions {
    /// Whether to show the minimap.
    pub show_minimap: bool,
    /// Minimap configuration.
    pub minimap_config: MinimapConfig,
    /// Whether to show the statusline at the bottom.
    pub show_statusline: bool,
    /// Whether selection mode is active (affects statusline mode display).
    pub selection_active: bool,
    /// Whether the minimap is currently hovered (expands when hovered).
    pub minimap_hovered: bool,
}

impl Default for MarkdownRenderOptions {
    fn default() -> Self {
        Self {
            show_minimap: false,
            minimap_config: MinimapConfig::default(),
            show_statusline: false,
            selection_active: false,
            minimap_hovered: false,
        }
    }
}

impl MarkdownRenderOptions {
    /// Create new options with minimap enabled.
    pub fn with_minimap() -> Self {
        Self {
            show_minimap: true,
            minimap_config: MinimapConfig::default(),
            show_statusline: false,
            selection_active: false,
            minimap_hovered: false,
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

    /// Set minimap hovered state (expands minimap when hovered).
    pub fn minimap_hovered(mut self, hovered: bool) -> Self {
        self.minimap_hovered = hovered;
        self
    }
}

/// Render markdown with selection and optional minimap directly to buffer.
///
/// This function handles the complete rendering including:
/// - Markdown content with selection highlighting
/// - Optional minimap overlay in the top-right corner
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
/// * `options` - Render options including minimap and statusline settings
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
    options: &MarkdownRenderOptions,
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

    // Calculate minimap overlay area (small box in top-right corner, overlays content)
    // When hovered, expand the minimap for better visibility
    let hover_scale: u16 = if options.minimap_hovered { 2 } else { 1 };
    let minimap_width = options.minimap_config.width * hover_scale;
    let minimap_height = (options.minimap_config.height * hover_scale).min(main_area.height.saturating_sub(1));
    let padding_right: u16 = 2;
    let padding_top: u16 = 1;
    let content_area = main_area;
    let minimap_area = if options.show_minimap && main_area.width > minimap_width + padding_right + 10 {
        Some(Rect {
            x: main_area.x + main_area.width.saturating_sub(minimap_width + padding_right),
            y: main_area.y + padding_top,
            width: minimap_width,
            height: minimap_height,
        })
    } else {
        None
    };

    // Update viewport height for scroll calculations
    scroll.update_viewport(content_area);

    // Render markdown content
    let (text, all_lines) = render_markdown_interactive_with_selection(
        content,
        scroll,
        content_area,
        is_resizing,
        selection,
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

    // Render minimap if enabled
    if let Some(mm_area) = minimap_area {
        let viewport_start = scroll.scroll_offset;
        let viewport_end = viewport_start + content_area.height as usize;
        let total_lines = scroll.total_lines;

        let minimap = Minimap::new(content)
            .width(mm_area.width)
            .viewport(viewport_start, viewport_end, total_lines)
            .config(options.minimap_config.clone());

        minimap.render(mm_area, buf);
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

        render_markdown_statusline(
            sl_area,
            buf,
            mode,
            filename,
            scroll.git_stats(),
            scroll.current_line,
            display_total,
        );
    }

    all_lines
}

//! Constructor for MarkdownWidget.

use crate::markdown_renderer::minimap::MinimapConfig;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;
use crate::markdown_renderer::toc::TocConfig;

use super::super::super::double_click_state::DoubleClickState;
use super::super::super::selection_state::SelectionState;
use super::super::{MarkdownWidget, MarkdownWidgetMode};

impl<'a> MarkdownWidget<'a> {
    /// Create a new MarkdownWidget with the given content and state managers.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to render
    /// * `scroll` - The scroll manager for handling scroll state
    /// * `selection` - The selection state for text selection/copy
    /// * `double_click` - The double-click state for detection
    ///
    /// # Returns
    ///
    /// A new `MarkdownWidget` instance.
    pub fn new(
        content: &'a str,
        scroll: &'a mut MarkdownScrollManager,
        selection: &'a mut SelectionState,
        double_click: &'a mut DoubleClickState,
    ) -> Self {
        Self {
            content,
            scroll,
            selection,
            double_click,
            is_resizing: false,
            mode: MarkdownWidgetMode::Normal,
            show_statusline: true,
            show_scrollbar: false,
            selection_active: false,
            git_stats: None,
            show_minimap: false,
            minimap_config: MinimapConfig::default(),
            minimap_hovered: false,
            show_toc: false,
            toc_config: TocConfig::default(),
            toc_hovered: false,
            toc_hovered_entry: None,
            toc_scroll_offset: 0,
            rendered_lines: Vec::new(),
            app_theme: None,
        }
    }

    /// Enable or disable the minimap.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the minimap
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_minimap(mut self, show: bool) -> Self {
        self.show_minimap = show;
        self
    }

    /// Set the minimap configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The minimap configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn minimap_config(mut self, config: MinimapConfig) -> Self {
        self.minimap_config = config;
        self
    }

    /// Set the minimap width.
    ///
    /// # Arguments
    ///
    /// * `width` - Width in terminal columns
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn minimap_width(mut self, width: u16) -> Self {
        self.minimap_config.width = width;
        self
    }

    /// Set the minimap height.
    ///
    /// # Arguments
    ///
    /// * `height` - Height in terminal rows
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn minimap_height(mut self, height: u16) -> Self {
        self.minimap_config.height = height;
        self
    }

    /// Enable or disable the TOC (Table of Contents).
    ///
    /// When enabled, shows heading navigation in the top-right corner.
    /// Compact mode shows lines, expanded mode (on hover) shows text.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the TOC
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_toc(mut self, show: bool) -> Self {
        self.show_toc = show;
        self
    }

    /// Set the TOC configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The TOC configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_config(mut self, config: TocConfig) -> Self {
        self.toc_config = config;
        self
    }

    /// Enable or disable the scrollbar.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the scrollbar
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }

    /// Set whether selection mode is active.
    ///
    /// This affects the mode displayed in the statusline (Normal vs Drag).
    ///
    /// # Arguments
    ///
    /// * `active` - Whether selection is active
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn selection_active(mut self, active: bool) -> Self {
        self.selection_active = active;
        self
    }

    /// Set the TOC hovered state.
    ///
    /// When hovered, the TOC expands to show heading text.
    ///
    /// # Arguments
    ///
    /// * `hovered` - Whether the TOC is hovered
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_hovered(mut self, hovered: bool) -> Self {
        self.toc_hovered = hovered;
        self
    }

    /// Set the hovered TOC entry index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the hovered entry, or None
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_hovered_entry(mut self, index: Option<usize>) -> Self {
        self.toc_hovered_entry = index;
        self
    }

    /// Set the TOC scroll offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The scroll offset for the TOC list
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_scroll_offset(mut self, offset: usize) -> Self {
        self.toc_scroll_offset = offset;
        self
    }

    /// Set the minimap hovered state.
    ///
    /// When hovered, the minimap scales up.
    ///
    /// # Arguments
    ///
    /// * `hovered` - Whether the minimap is hovered
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn minimap_hovered(mut self, hovered: bool) -> Self {
        self.minimap_hovered = hovered;
        self
    }
}

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
            git_stats: None,
            show_minimap: false,
            minimap_config: MinimapConfig::default(),
            minimap_hovered: false,
            show_toc: false,
            toc_config: TocConfig::default(),
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
}

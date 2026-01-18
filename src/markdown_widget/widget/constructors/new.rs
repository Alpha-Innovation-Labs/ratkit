//! Constructor for MarkdownWidget.

use crate::markdown_widget::extensions::minimap::MinimapConfig;
use crate::markdown_widget::extensions::toc::TocConfig;
use crate::markdown_widget::state::double_click_state::DoubleClickState;
use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;
use crate::markdown_widget::state::selection_state::SelectionState;
use crate::markdown_widget::widget::enums::MarkdownWidgetMode;
use crate::markdown_widget::widget::MarkdownWidget;

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
            toc_state: None,
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
}

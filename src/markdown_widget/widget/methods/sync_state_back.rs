//! Sync widget state back to MarkdownState.

use crate::markdown_widget::state::MarkdownState;
use crate::markdown_widget::widget::MarkdownWidget;

/// State captured from MarkdownWidget that needs to be synced back to MarkdownState.
///
/// This struct holds the values that the widget may modify during event handling
/// that need to persist back to the application state.
#[derive(Debug, Clone)]
pub struct WidgetStateSync {
    /// Whether the TOC is currently hovered.
    pub toc_hovered: bool,
    /// Index of the hovered TOC entry.
    pub toc_hovered_entry: Option<usize>,
    /// Scroll offset for the TOC list.
    pub toc_scroll_offset: usize,
    /// Whether selection mode is active.
    pub selection_active: bool,
    /// Last double-click info (line number, kind, content).
    pub last_double_click: Option<(usize, String, String)>,
    /// Current filter text (when in filter mode).
    pub filter: Option<String>,
    /// Whether filter mode is currently active.
    pub filter_mode: bool,
}

impl WidgetStateSync {
    /// Apply this sync state to a MarkdownState.
    ///
    /// # Arguments
    ///
    /// * `state` - The MarkdownState to sync state to
    pub fn apply_to(&self, state: &mut MarkdownState) {
        state.toc_hovered = self.toc_hovered;
        state.toc_hovered_entry = self.toc_hovered_entry;
        state.toc_scroll_offset = self.toc_scroll_offset;
        state.selection_active = self.selection_active;
        state.filter = self.filter.clone();
        state.filter_mode = self.filter_mode;
    }

    /// Check if there was a double-click and consume it.
    pub fn take_double_click(&mut self) -> Option<(usize, String, String)> {
        self.last_double_click.take()
    }
}

impl<'a> MarkdownWidget<'a> {
    /// Get the state that needs to be synced back to MarkdownState.
    ///
    /// This method captures the TOC and selection state from the widget
    /// so it can be synced back after the widget is dropped.
    ///
    /// # Returns
    ///
    /// A `WidgetStateSync` struct containing the state values to sync.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let sync_state = {
    ///     let mut widget = MarkdownWidget::from_state(&content, &mut state).show_toc(true);
    ///     widget.handle_toc_hover(&mouse, render_area);
    ///     widget.handle_toc_click(&mouse, render_area);
    ///     widget.handle_mouse_event(&mouse, render_area);
    ///     widget.get_state_sync()
    /// };
    /// sync_state.apply_to(&mut state);
    /// ```
    pub fn get_state_sync(&mut self) -> WidgetStateSync {
        WidgetStateSync {
            toc_hovered: self.toc_hovered,
            toc_hovered_entry: self.toc_hovered_entry,
            toc_scroll_offset: self.toc_scroll_offset,
            selection_active: self.selection.is_active(),
            last_double_click: self.last_double_click.take(),
            filter: self.filter.clone(),
            filter_mode: self.filter_mode,
        }
    }

    /// Sync widget state back to MarkdownState by consuming self.
    ///
    /// This method consumes the widget and syncs TOC and selection state back to
    /// the MarkdownState, ensuring state persistence between frames.
    ///
    /// Call this after handling mouse events to preserve hover and selection state.
    ///
    /// # Arguments
    ///
    /// * `state` - The MarkdownState to sync state back to
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut widget = MarkdownWidget::from_state(&content, &mut state).show_toc(true);
    /// widget.handle_toc_hover(&mouse, render_area);
    /// widget.handle_toc_click(&mouse, render_area);
    /// widget.handle_mouse_event(&mouse, render_area);
    /// widget.sync_state_back(&mut state);
    /// ```
    pub fn sync_state_back(self, state: &mut MarkdownState) {
        state.toc_hovered = self.toc_hovered;
        state.toc_hovered_entry = self.toc_hovered_entry;
        state.toc_scroll_offset = self.toc_scroll_offset;
        state.selection_active = self.selection.is_active();
        state.filter = self.filter;
        state.filter_mode = self.filter_mode;
    }
}

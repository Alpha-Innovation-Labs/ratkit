//! Widget trait implementations for DiffFileTree.
//!
//! Uses TreeViewRef to avoid cloning nodes on every render.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

use crate::diff_file_tree::{DiffFileEntry, DiffFileTree};
use crate::services::theme::AppTheme;
use crate::tree_view::{matches_filter, TreeViewRef};

use super::render_entry::render_entry;

/// Filter function for DiffFileEntry nodes.
///
/// Returns true if the entry's name matches the filter (case-insensitive contains).
fn entry_matches_filter(entry: &DiffFileEntry, filter: &Option<String>) -> bool {
    matches_filter(&entry.name, filter)
}

/// Renders the filter input line at the bottom of the tree.
fn render_filter_line(
    filter_text: Option<&str>,
    filter_mode: bool,
    area: Rect,
    buf: &mut Buffer,
    theme: &AppTheme,
) {
    if area.height == 0 {
        return;
    }

    let y = area.y + area.height - 1;

    // Build the filter line
    let filter_str = filter_text.unwrap_or("");
    let cursor = if filter_mode { "_" } else { "" };

    let line = Line::from(vec![
        Span::styled(
            "/ ",
            Style::default()
                .fg(theme.warning)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(filter_str, Style::default().fg(theme.text)),
        Span::styled(
            cursor,
            Style::default()
                .fg(theme.warning)
                .add_modifier(Modifier::SLOW_BLINK),
        ),
    ]);

    // Fill background for the filter line
    let bg_style = Style::default().bg(theme.background_panel);
    for x in area.x..(area.x + area.width) {
        buf[(x, y)].set_style(bg_style);
    }

    buf.set_line(area.x, y, &line, area.width);
}

impl Widget for DiffFileTree {
    /// Renders the diff file tree widget to the given buffer.
    ///
    /// Uses TreeViewRef internally to render the tree with custom styling.
    /// When filter mode is active, shows a filter input at the bottom.
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let focused = self.focused;
        let theme = self.theme.clone();
        let filter_mode = self.state.filter_mode;
        let has_filter = self.state.filter.as_ref().map_or(false, |f| !f.is_empty());
        let show_filter_line = filter_mode || has_filter;

        // Calculate tree area (leave room for filter line if needed)
        let tree_area = if show_filter_line && area.height > 1 {
            Rect {
                height: area.height - 1,
                ..area
            }
        } else {
            area
        };

        let highlight_bg = theme.background_element;
        let icon_style = Style::default().fg(theme.info);
        let theme_for_render = theme.clone();

        let tree_view = TreeViewRef::new(&self.nodes)
            .icons("\u{F07B}", "\u{F07C}") // Nerd font folder icons (closed, open)
            .icon_style(icon_style)
            .render_fn(move |entry, node_state| {
                render_entry(entry, node_state, focused, &theme_for_render)
            })
            .filter_fn(entry_matches_filter)
            .highlight_style(Style::default().bg(highlight_bg));

        ratatui::widgets::StatefulWidget::render(tree_view, tree_area, buf, &mut self.state);

        // Render filter line if needed
        if show_filter_line && area.height > 1 {
            render_filter_line(self.state.filter.as_deref(), filter_mode, area, buf, &theme);
        }
    }
}

impl Widget for &DiffFileTree {
    /// Renders the diff file tree widget from a reference.
    ///
    /// Note: This creates a clone of the state for rendering.
    /// When filter mode is active, shows a filter input at the bottom.
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let focused = self.focused;
        let theme = self.theme.clone();
        let filter_mode = self.state.filter_mode;
        let has_filter = self.state.filter.as_ref().map_or(false, |f| !f.is_empty());
        let show_filter_line = filter_mode || has_filter;
        let mut state = self.state.clone();

        // Calculate tree area (leave room for filter line if needed)
        let tree_area = if show_filter_line && area.height > 1 {
            Rect {
                height: area.height - 1,
                ..area
            }
        } else {
            area
        };

        let highlight_bg = theme.background_element;
        let icon_style = Style::default().fg(theme.info);
        let theme_for_render = theme.clone();

        let tree_view = TreeViewRef::new(&self.nodes)
            .icons("\u{F07B}", "\u{F07C}") // Nerd font folder icons (closed, open)
            .icon_style(icon_style)
            .render_fn(move |entry, node_state| {
                render_entry(entry, node_state, focused, &theme_for_render)
            })
            .filter_fn(entry_matches_filter)
            .highlight_style(Style::default().bg(highlight_bg));

        ratatui::widgets::StatefulWidget::render(tree_view, tree_area, buf, &mut state);

        // Render filter line if needed
        if show_filter_line && area.height > 1 {
            render_filter_line(self.state.filter.as_deref(), filter_mode, area, buf, &theme);
        }
    }
}

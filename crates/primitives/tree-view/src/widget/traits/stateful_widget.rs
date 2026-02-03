//! StatefulWidget implementation for TreeView.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{StatefulWidget, Widget},
};

use crate::tree_view_state::TreeViewState;
use crate::widget::TreeView;

impl<'a, T> StatefulWidget for TreeView<'a, T> {
    type State = TreeViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = match self.block {
            Some(ref b) => {
                let inner = b.inner(area);
                b.clone().render(area, buf);
                inner
            }
            None => area,
        };

        if area.height == 0 {
            return;
        }

        let filter_mode = state.filter_mode;
        let has_filter = state.filter.as_ref().is_some_and(|f| !f.is_empty());
        let show_filter_line = self.show_filter_ui && (filter_mode || has_filter);

        let tree_area = if show_filter_line && area.height > 1 {
            Rect {
                height: area.height - 1,
                ..area
            }
        } else {
            area
        };

        let items = self.flatten_tree(state);
        let visible_height = tree_area.height as usize;

        if let Some(ref selected) = state.selected_path {
            if let Some(selected_idx) = items.iter().position(|(_, path)| path == selected) {
                if selected_idx < state.offset {
                    state.offset = selected_idx;
                } else if selected_idx >= state.offset + visible_height {
                    state.offset = selected_idx.saturating_sub(visible_height - 1);
                }
            }
        }

        for (i, (line, path)) in items
            .iter()
            .skip(state.offset)
            .take(visible_height)
            .enumerate()
        {
            let y = tree_area.y + i as u16;

            let is_selected = state.selected_path.as_ref() == Some(path);
            if is_selected && self.highlight_style.is_some() {
                let style = self.highlight_style.unwrap();
                for x in tree_area.x..(tree_area.x + tree_area.width) {
                    buf[(x, y)].set_style(style);
                }
            }

            buf.set_line(tree_area.x, y, line, tree_area.width);
        }

        if show_filter_line && area.height > 1 {
            self.render_filter_line(area, buf, state);
        }
    }
}

impl<'a, T> TreeView<'a, T> {
    fn render_filter_line(&self, area: Rect, buf: &mut Buffer, state: &TreeViewState) {
        let y = area.y + area.height - 1;
        let filter_text = state.filter.as_deref().unwrap_or("");
        let cursor = if state.filter_mode { "_" } else { "" };

        let line = Line::from(vec![
            ratatui::text::Span::styled(
                "/ ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            ratatui::text::Span::styled(filter_text, Style::default().fg(Color::White)),
            ratatui::text::Span::styled(
                cursor,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::SLOW_BLINK),
            ),
        ]);

        let bg_style = Style::default().bg(Color::DarkGray);
        for x in area.x..(area.x + area.width) {
            buf[(x, y)].set_style(bg_style);
        }

        buf.set_line(area.x, y, &line, area.width);
    }
}

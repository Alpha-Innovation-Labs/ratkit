//! StatefulWidget implementation for TreeView.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};

use crate::primitives::tree_view::tree_view_state::TreeViewState;
use crate::primitives::tree_view::widget::TreeView;

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

        let items = self.flatten_tree(state);
        let visible_height = area.height as usize;

        // Adjust scroll offset to ensure selected item is visible
        if let Some(ref selected) = state.selected_path {
            if let Some(selected_idx) = items.iter().position(|(_, path)| path == selected) {
                if selected_idx < state.offset {
                    state.offset = selected_idx;
                } else if selected_idx >= state.offset + visible_height {
                    state.offset = selected_idx.saturating_sub(visible_height - 1);
                }
            }
        }

        // Render visible items
        for (i, (line, path)) in items
            .iter()
            .skip(state.offset)
            .take(visible_height)
            .enumerate()
        {
            let y = area.y + i as u16;

            // Fill background for selected row (full-width highlight like Yazi)
            let is_selected = state.selected_path.as_ref() == Some(path);
            if is_selected && self.highlight_style.is_some() {
                let style = self.highlight_style.unwrap();
                for x in area.x..(area.x + area.width) {
                    buf[(x, y)].set_style(style);
                }
            }

            buf.set_line(area.x, y, line, area.width);
        }
    }
}

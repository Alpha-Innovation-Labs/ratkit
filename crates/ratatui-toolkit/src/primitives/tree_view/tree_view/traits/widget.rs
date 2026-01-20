//! Widget implementation for &TreeView.

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::primitives::tree_view::tree_view::TreeView;
use crate::primitives::tree_view::tree_view_state::TreeViewState;

impl<'a, T> Widget for &TreeView<'a, T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let state = TreeViewState::default();

        let area = match &self.block {
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

        let items = self.flatten_tree(&state);
        let visible_height = area.height as usize;

        // Render visible items
        for (i, (line, _)) in items
            .iter()
            .skip(state.offset)
            .take(visible_height)
            .enumerate()
        {
            let y = area.y + i as u16;
            buf.set_line(area.x, y, line, area.width);
        }
    }
}

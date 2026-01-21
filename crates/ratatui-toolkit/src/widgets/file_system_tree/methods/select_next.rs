use crate::primitives::tree_view::TreeViewState;
use crate::widgets::file_system_tree::FileSystemTree;

impl<'a> FileSystemTree<'a> {
    pub fn select_next(&mut self, state: &mut TreeViewState) {
        let visible_paths = self.get_visible_paths(state);
        if visible_paths.is_empty() {
            return;
        }

        if let Some(current_path) = &state.selected_path {
            if let Some(current_idx) = visible_paths.iter().position(|p| p == current_path) {
                if current_idx < visible_paths.len() - 1 {
                    state.select(visible_paths[current_idx + 1].clone());
                }
            }
        } else {
            state.select(visible_paths[0].clone());
        }
    }
}

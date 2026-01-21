use anyhow::Result;

use crate::primitives::tree_view::TreeViewState;
use crate::widgets::file_system_tree::FileSystemTree;

impl<'a> FileSystemTree<'a> {
    pub fn toggle_selected(&mut self, state: &mut TreeViewState) -> Result<()> {
        if let Some(path) = state.selected_path.clone() {
            if let Some(entry) = self.get_entry_at_path(&path) {
                if entry.is_dir {
                    if !state.is_expanded(&path) {
                        self.expand_directory(&path)?;
                    }
                    state.toggle_expansion(path);
                }
            }
        }
        Ok(())
    }
}

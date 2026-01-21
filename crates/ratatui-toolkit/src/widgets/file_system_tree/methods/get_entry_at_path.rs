use crate::primitives::tree_view::TreeNode;
use crate::widgets::file_system_tree::{FileSystemEntry, FileSystemTree};

impl<'a> FileSystemTree<'a> {
    pub fn get_entry_at_path(&self, path: &[usize]) -> Option<&FileSystemEntry> {
        fn find_entry<'a>(
            nodes: &'a [TreeNode<FileSystemEntry>],
            path: &[usize],
        ) -> Option<&'a FileSystemEntry> {
            if path.is_empty() {
                return None;
            }

            if let Some(node) = nodes.get(path[0]) {
                if path.len() == 1 {
                    return Some(&node.data);
                }
                return find_entry(&node.children, &path[1..]);
            }
            None
        }

        find_entry(&self.nodes, path)
    }
}

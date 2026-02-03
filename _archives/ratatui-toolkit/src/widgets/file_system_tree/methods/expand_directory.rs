use anyhow::Result;

use crate::primitives::tree_view::TreeNode;
use crate::widgets::file_system_tree::{FileSystemEntry, FileSystemTree, FileSystemTreeConfig};

impl<'a> FileSystemTree<'a> {
    pub fn expand_directory(&mut self, path: &[usize]) -> Result<()> {
        fn find_and_expand(
            nodes: &mut [TreeNode<FileSystemEntry>],
            path: &[usize],
            config: &FileSystemTreeConfig,
        ) -> Result<()> {
            if path.is_empty() {
                return Ok(());
            }

            if path.len() == 1 {
                if let Some(node) = nodes.get_mut(path[0]) {
                    if node.data.is_dir && node.children.is_empty() {
                        node.children = FileSystemTree::load_directory(&node.data.path, config)?;
                    }
                }
                return Ok(());
            }

            if let Some(node) = nodes.get_mut(path[0]) {
                find_and_expand(&mut node.children, &path[1..], config)?;
            }

            Ok(())
        }

        find_and_expand(&mut self.nodes, path, &self.config)
    }
}

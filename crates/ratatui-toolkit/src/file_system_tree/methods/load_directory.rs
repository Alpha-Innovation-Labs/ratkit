use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::file_system_tree::{FileSystemEntry, FileSystemTree, FileSystemTreeConfig};
use crate::primitives::tree_view::TreeNode;

impl<'a> FileSystemTree<'a> {
    pub(crate) fn load_directory(
        path: &Path,
        config: &FileSystemTreeConfig,
    ) -> Result<Vec<TreeNode<FileSystemEntry>>> {
        let mut entries = Vec::new();

        let read_dir = fs::read_dir(path).context("Failed to read directory")?;

        for entry in read_dir {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            let fs_entry = FileSystemEntry::new(path.clone())?;

            if fs_entry.is_hidden && !config.show_hidden {
                continue;
            }

            let node = if fs_entry.is_dir {
                TreeNode {
                    data: fs_entry,
                    children: Vec::new(),
                    expandable: true,
                }
            } else {
                TreeNode::new(fs_entry)
            };

            entries.push(node);
        }

        entries.sort_by(|a, b| match (a.data.is_dir, b.data.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase()),
        });

        Ok(entries)
    }
}

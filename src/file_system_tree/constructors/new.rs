use anyhow::Result;
use std::path::PathBuf;

use crate::file_system_tree::{FileSystemTree, FileSystemTreeConfig};
use ratatui::widgets::Block;

impl<'a> FileSystemTree<'a> {
    pub fn new(root_path: PathBuf) -> Result<Self> {
        let config = FileSystemTreeConfig::default();
        let nodes = Self::load_directory(&root_path, &config)?;

        Ok(Self {
            root_path,
            nodes,
            config,
            block: None,
        })
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

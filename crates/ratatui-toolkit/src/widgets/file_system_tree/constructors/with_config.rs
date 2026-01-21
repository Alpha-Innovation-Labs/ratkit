use anyhow::Result;
use std::path::PathBuf;

use crate::widgets::file_system_tree::{FileSystemTree, FileSystemTreeConfig};

impl<'a> FileSystemTree<'a> {
    pub fn with_config(root_path: PathBuf, config: FileSystemTreeConfig) -> Result<Self> {
        let nodes = Self::load_directory(&root_path, &config)?;

        Ok(Self {
            root_path,
            nodes,
            config,
            block: None,
        })
    }
}

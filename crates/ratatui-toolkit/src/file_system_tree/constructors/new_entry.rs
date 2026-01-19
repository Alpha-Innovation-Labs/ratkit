use anyhow::Result;
use std::path::PathBuf;

use crate::file_system_tree::FileSystemEntry;

impl FileSystemEntry {
    pub fn new(path: PathBuf) -> Result<Self> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let is_dir = path.is_dir();
        let is_hidden = name.starts_with('.');

        Ok(Self {
            name,
            path,
            is_dir,
            is_hidden,
        })
    }
}

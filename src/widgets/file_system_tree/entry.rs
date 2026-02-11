use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileSystemEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_hidden: bool,
}

impl FileSystemEntry {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
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

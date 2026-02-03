//! Source module for markdown source.

use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct MarkdownSource {
    content: String,
    file_path: Option<PathBuf>,
    last_modified: Option<std::time::SystemTime>,
}

impl MarkdownSource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_content(content: &str) -> Self {
        Self {
            content: content.to_string(),
            file_path: None,
            last_modified: None,
        }
    }

    pub fn from_file(path: &PathBuf) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let last_modified = std::fs::metadata(path)?.modified().ok();

        Ok(Self {
            content,
            file_path: Some(path.clone()),
            last_modified,
        })
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}

use crate::file_system_tree::FileSystemTreeConfig;

#[allow(dead_code)]
pub fn with_show_hidden(config: FileSystemTreeConfig, show_hidden: bool) -> FileSystemTreeConfig {
    FileSystemTreeConfig {
        show_hidden,
        ..config
    }
}

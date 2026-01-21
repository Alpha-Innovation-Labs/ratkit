use crate::widgets::file_system_tree::FileSystemTreeConfig;

#[allow(dead_code)]
pub fn with_dark_theme(config: FileSystemTreeConfig, use_dark: bool) -> FileSystemTreeConfig {
    FileSystemTreeConfig {
        use_dark_theme: use_dark,
        ..config
    }
}

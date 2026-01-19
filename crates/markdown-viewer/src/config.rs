//! Viewer configuration

use ratatui::crossterm::event::KeyCode;

use ratatui_toolkit::{AppTheme, FileSystemTreeConfig};

/// Configuration for the markdown viewer
#[derive(Debug, Clone)]
pub struct ViewerConfig {
    /// Initial split ratio (0.0-1.0)
    pub initial_split_ratio: f32,
    /// Minimum pane width
    pub min_pane_width: u16,
    /// Maximum pane width (None for no limit)
    pub max_pane_width: Option<u16>,
    /// Show file tree
    pub show_file_tree: bool,
    /// Key to toggle file tree visibility
    pub hide_file_tree_key: KeyCode,
    /// File tree configuration
    pub file_tree_config: FileSystemTreeConfig,
    /// Markdown panel configuration
    pub markdown_config: MarkdownPanelConfig,
}

impl Default for ViewerConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewerConfig {
    /// Create a new configuration with defaults
    pub fn new() -> Self {
        Self {
            initial_split_ratio: 0.25,
            min_pane_width: 20,
            max_pane_width: None,
            show_file_tree: true,
            hide_file_tree_key: KeyCode::Char('['),
            file_tree_config: FileSystemTreeConfig {
                show_hidden: true,
                use_dark_theme: true,
                ..Default::default()
            },
            markdown_config: MarkdownPanelConfig::new(),
        }
    }

    /// Set initial split ratio
    pub fn with_split_ratio(mut self, ratio: f32) -> Self {
        self.initial_split_ratio = ratio.clamp(0.1, 0.9);
        self
    }

    /// Set minimum pane width
    pub fn with_min_pane_width(mut self, width: u16) -> Self {
        self.min_pane_width = width;
        self
    }

    /// Set maximum pane width
    pub fn with_max_pane_width(mut self, width: u16) -> Self {
        self.max_pane_width = Some(width);
        self
    }

    /// Set file tree visibility
    pub fn with_file_tree_visible(mut self, visible: bool) -> Self {
        self.show_file_tree = visible;
        self
    }

    /// Set key to toggle file tree visibility
    pub fn with_toggle_key(mut self, key: KeyCode) -> Self {
        self.hide_file_tree_key = key;
        self
    }

    /// Set file tree configuration
    pub fn with_file_tree_config(mut self, config: FileSystemTreeConfig) -> Self {
        self.file_tree_config = config;
        self
    }

    /// Set markdown panel configuration
    pub fn with_markdown_config(mut self, config: MarkdownPanelConfig) -> Self {
        self.markdown_config = config;
        self
    }

    /// Set theme
    pub fn with_theme(self, _theme: AppTheme) -> Self {
        // TODO: Implement theme support when API is available
        self
    }
}

/// Configuration for the markdown panel
#[derive(Debug, Clone)]
pub struct MarkdownPanelConfig {
    /// Show table of contents
    pub show_toc: bool,
    /// Show status line
    pub show_statusline: bool,
    /// Show scrollbar
    pub show_scrollbar: bool,
}

impl Default for MarkdownPanelConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownPanelConfig {
    /// Create a new configuration with defaults
    pub fn new() -> Self {
        Self {
            show_toc: true,
            show_statusline: true,
            show_scrollbar: true,
        }
    }

    /// Set table of contents visibility
    pub fn with_toc(mut self, show: bool) -> Self {
        self.show_toc = show;
        self
    }

    /// Set status line visibility
    pub fn with_statusline(mut self, show: bool) -> Self {
        self.show_statusline = show;
        self
    }

    /// Set scrollbar visibility
    pub fn with_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }
}

use anyhow::{Context, Result};
use devicons::{icon_for_file, Theme as DevIconTheme};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, StatefulWidget},
};
use std::fs;
use std::path::{Path, PathBuf};

use crate::tree_view::{TreeNode, TreeView, TreeViewState};

/// Represents a file system entry (file or directory)
#[derive(Debug, Clone)]
pub struct FileSystemEntry {
    /// Name of the file/directory
    pub name: String,
    /// Full path
    pub path: PathBuf,
    /// Whether this is a directory
    pub is_dir: bool,
    /// Whether this entry is hidden (starts with .)
    pub is_hidden: bool,
}

impl FileSystemEntry {
    /// Create a new file system entry
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

/// Configuration for the file system tree
#[derive(Debug, Clone, Copy)]
pub struct FileSystemTreeConfig {
    /// Show hidden files (starting with .)
    pub show_hidden: bool,
    /// Use dark theme for icons (true = dark, false = light)
    pub use_dark_theme: bool,
    /// Style for directories
    pub dir_style: Style,
    /// Style for files
    pub file_style: Style,
    /// Style for selected items
    pub selected_style: Style,
}

impl Default for FileSystemTreeConfig {
    fn default() -> Self {
        Self {
            show_hidden: false,
            use_dark_theme: true,
            dir_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            file_style: Style::default().fg(Color::White),
            selected_style: Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        }
    }
}

impl FileSystemTreeConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_show_hidden(mut self, show_hidden: bool) -> Self {
        self.show_hidden = show_hidden;
        self
    }

    pub fn with_dark_theme(mut self, use_dark: bool) -> Self {
        self.use_dark_theme = use_dark;
        self
    }

    pub fn with_dir_style(mut self, style: Style) -> Self {
        self.dir_style = style;
        self
    }

    pub fn with_file_style(mut self, style: Style) -> Self {
        self.file_style = style;
        self
    }

    pub fn with_selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }
}

/// File system tree browser widget
#[derive(Clone)]
pub struct FileSystemTree<'a> {
    /// Root directory to browse
    pub root_path: PathBuf,
    /// Tree nodes built from file system
    pub nodes: Vec<TreeNode<FileSystemEntry>>,
    /// Configuration
    pub(crate) config: FileSystemTreeConfig,
    /// Optional block wrapper
    block: Option<Block<'a>>,
}

impl<'a> FileSystemTree<'a> {
    /// Create a new file system tree starting at the given path
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

    /// Create with custom configuration
    pub fn with_config(root_path: PathBuf, config: FileSystemTreeConfig) -> Result<Self> {
        let nodes = Self::load_directory(&root_path, &config)?;

        Ok(Self {
            root_path,
            nodes,
            config,
            block: None,
        })
    }

    /// Set the block wrapper
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Load a directory and return tree nodes
    fn load_directory(
        path: &Path,
        config: &FileSystemTreeConfig,
    ) -> Result<Vec<TreeNode<FileSystemEntry>>> {
        let mut entries = Vec::new();

        let read_dir = fs::read_dir(path).context("Failed to read directory")?;

        for entry in read_dir {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            let fs_entry = FileSystemEntry::new(path.clone())?;

            // Skip hidden files if configured
            if fs_entry.is_hidden && !config.show_hidden {
                continue;
            }

            // Create tree node
            let node = if fs_entry.is_dir {
                // For directories, mark as expandable but don't load children yet
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

        // Sort: directories first, then files, both alphabetically
        entries.sort_by(|a, b| match (a.data.is_dir, b.data.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase()),
        });

        Ok(entries)
    }

    /// Expand a directory node by loading its children
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
                        // Load children for this directory
                        node.children = FileSystemTree::load_directory(&node.data.path, config)?;
                    }
                }
                return Ok(());
            }

            // Recurse deeper
            if let Some(node) = nodes.get_mut(path[0]) {
                find_and_expand(&mut node.children, &path[1..], config)?;
            }

            Ok(())
        }

        find_and_expand(&mut self.nodes, path, &self.config)
    }

    /// Get the entry at the currently selected path
    pub fn get_selected_entry(&self, state: &TreeViewState) -> Option<FileSystemEntry> {
        if let Some(path) = &state.selected_path {
            self.get_entry_at_path(path)
        } else {
            None
        }
    }

    /// Get entry at a specific path
    fn get_entry_at_path(&self, path: &[usize]) -> Option<FileSystemEntry> {
        fn find_entry(
            nodes: &[TreeNode<FileSystemEntry>],
            path: &[usize],
        ) -> Option<FileSystemEntry> {
            if path.is_empty() {
                return None;
            }

            if path.len() == 1 {
                return nodes.get(path[0]).map(|n| n.data.clone());
            }

            if let Some(node) = nodes.get(path[0]) {
                return find_entry(&node.children, &path[1..]);
            }

            None
        }

        find_entry(&self.nodes, path)
    }
}

impl<'a> StatefulWidget for FileSystemTree<'a> {
    type State = TreeViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let config = self.config;
        let block = self.block;

        // Create the tree view with custom render function
        let tree_view = TreeView::new(self.nodes)
            .icons("", "") // No expand/collapse arrows
            .render_fn(move |entry, node_state| {
                let theme = if config.use_dark_theme {
                    DevIconTheme::Dark
                } else {
                    DevIconTheme::Light
                };

                // Use custom folder icons and Ayu Dark theme colors
                let (icon_glyph, icon_color) = if entry.is_dir {
                    // Ayu Dark folder color: #1f6f88 (teal)
                    if node_state.is_expanded {
                        ('\u{f07c}', Color::Rgb(31, 111, 136)) // Open folder  - Ayu Dark teal
                    } else {
                        ('\u{f07b}', Color::Rgb(31, 111, 136)) // Closed folder  - Ayu Dark teal
                    }
                } else {
                    // Get icon from devicons or custom icons
                    let icon_char = if let Some((custom_icon, _)) = get_custom_icon(&entry.name) {
                        custom_icon
                    } else {
                        let file_icon = icon_for_file(&entry.name, &Some(theme));
                        file_icon.icon
                    };

                    // But use Ayu Dark colors instead of devicons colors
                    let color = get_ayu_dark_color(&entry.name);
                    (icon_char, color)
                };

                // Always use icon color for the filename (full-row highlight handles selection)
                let style = Style::default().fg(icon_color);

                Line::from(vec![
                    Span::styled(format!("{} ", icon_glyph), Style::default().fg(icon_color)),
                    Span::styled(entry.name.clone(), style),
                ])
            })
            .highlight_style(Style::default().bg(Color::Rgb(15, 25, 40))); // Darker blue selection bg: #0f1928

        let tree_view = if let Some(block) = block {
            tree_view.block(block)
        } else {
            tree_view
        };

        tree_view.render(area, buf, state);
    }
}

/// Map file to Ayu Dark theme color based on file type
fn get_ayu_dark_color(filename: &str) -> Color {
    let lower = filename.to_lowercase();

    // Check if executable (simplified - just check common script extensions)
    if lower.ends_with(".sh")
        || lower.ends_with(".bash")
        || lower.ends_with(".zsh")
        || lower.ends_with(".fish")
        || lower.ends_with(".py")
        || lower.ends_with(".rb")
    {
        return Color::Rgb(126, 147, 80); // Green for executables/scripts (#7e9350)
    }

    // Images
    if lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".gif")
        || lower.ends_with(".svg")
        || lower.ends_with(".ico")
        || lower.ends_with(".webp")
        || lower.ends_with(".bmp")
    {
        return Color::Rgb(194, 160, 92); // Yellow/gold (#c2a05c)
    }

    // Media (audio/video)
    if lower.ends_with(".mp3")
        || lower.ends_with(".mp4")
        || lower.ends_with(".wav")
        || lower.ends_with(".avi")
        || lower.ends_with(".mkv")
        || lower.ends_with(".flac")
        || lower.ends_with(".ogg")
        || lower.ends_with(".webm")
    {
        return Color::Rgb(126, 147, 80); // Green (#7e9350)
    }

    // Archives
    if lower.ends_with(".zip")
        || lower.ends_with(".tar")
        || lower.ends_with(".gz")
        || lower.ends_with(".bz2")
        || lower.ends_with(".xz")
        || lower.ends_with(".7z")
        || lower.ends_with(".rar")
    {
        return Color::Rgb(168, 83, 97); // Red (#a85361)
    }

    // Documents
    if lower.ends_with(".pdf")
        || lower.ends_with(".doc")
        || lower.ends_with(".docx")
        || lower.ends_with(".rtf")
        || lower.ends_with(".odt")
    {
        return Color::Rgb(31, 111, 136); // Teal (#1f6f88)
    }

    // Config/data files
    if lower.ends_with(".json")
        || lower.ends_with(".js")
        || lower.ends_with(".ts")
        || lower.ends_with(".jsx")
        || lower.ends_with(".tsx")
    {
        return Color::Rgb(194, 160, 92); // Yellow (#c2a05c)
    }

    if lower.ends_with(".yml") || lower.ends_with(".yaml") {
        return Color::Rgb(31, 111, 136); // Teal (#1f6f88)
    }

    if lower.ends_with(".toml") {
        return Color::Rgb(148, 100, 182); // Purple (#9464b6)
    }

    // Rust files
    if lower.ends_with(".rs") {
        return Color::Rgb(194, 160, 92); // Yellow (#c2a05c)
    }

    // C/C++
    if lower.ends_with(".c")
        || lower.ends_with(".cpp")
        || lower.ends_with(".h")
        || lower.ends_with(".hpp")
    {
        return Color::Rgb(31, 111, 136); // Teal (#1f6f88)
    }

    // Go
    if lower.ends_with(".go") {
        return Color::Rgb(31, 111, 136); // Teal (#1f6f88)
    }

    // Markdown/text (keep these slightly dimmed)
    if lower.ends_with(".md") || lower.ends_with(".txt") || lower.ends_with(".log") {
        return Color::Rgb(230, 225, 207); // Ayu Dark foreground (#e6e1cf)
    }

    // Default: Ayu Dark foreground color (warm white/beige) for regular files
    Color::Rgb(230, 225, 207) // #e6e1cf
}

/// Get custom icon for files that devicons might not recognize
/// Returns (icon_char, color) if a custom icon is available
fn get_custom_icon(filename: &str) -> Option<(char, Color)> {
    let lower = filename.to_lowercase();

    // === BUILD TOOLS ===

    // .just files (justfile build tool)
    if lower.ends_with(".just") || lower == "justfile" || lower == ".justfile" {
        return Some(('\u{e779}', Color::Rgb(194, 160, 92))); //  - makefile/build icon in yellow
    }

    // Makefile
    if lower == "makefile" || lower.starts_with("makefile.") || lower == "gnumakefile" {
        return Some(('\u{e779}', Color::Rgb(109, 128, 134))); //  - makefile icon
    }

    // === RUBY ===

    // Gemfile
    if lower == "gemfile" || lower == "gemfile.lock" {
        return Some(('\u{e21e}', Color::Rgb(112, 21, 22))); //  - ruby red
    }

    // === ENVIRONMENT/CONFIG ===

    // .env files (all variants)
    if lower == ".env" || lower.starts_with(".env.") {
        return Some(('\u{f462}', Color::Rgb(251, 192, 45))); //  - env yellow
    }

    // === LICENSE ===

    // License files
    if lower == "license"
        || lower == "license.txt"
        || lower == "license.md"
        || lower == "licence"
        || lower == "licence.txt"
        || lower == "copying"
    {
        return Some(('\u{f48a}', Color::Rgb(216, 187, 98))); //  - license yellow
    }

    // === CI/CD ===

    // Jenkins
    if lower == "jenkinsfile" || lower.starts_with("jenkinsfile.") {
        return Some(('\u{e767}', Color::Rgb(217, 69, 57))); //  - jenkins red
    }

    // === macOS ===

    // .DS_Store
    if lower == ".ds_store" {
        return Some(('\u{f179}', Color::Rgb(126, 142, 168))); //  - apple icon gray
    }

    None
}

/// Helper methods for keyboard navigation
impl<'a> FileSystemTree<'a> {
    /// Get all visible paths (flattened tree with expansion state)
    fn get_visible_paths(&self, state: &TreeViewState) -> Vec<Vec<usize>> {
        let mut paths = Vec::new();

        fn traverse(
            nodes: &[TreeNode<FileSystemEntry>],
            current_path: Vec<usize>,
            state: &TreeViewState,
            paths: &mut Vec<Vec<usize>>,
        ) {
            for (idx, node) in nodes.iter().enumerate() {
                let mut path = current_path.clone();
                path.push(idx);
                paths.push(path.clone());

                // If expanded, recurse into children
                if state.is_expanded(&path) && !node.children.is_empty() {
                    traverse(&node.children, path, state, paths);
                }
            }
        }

        traverse(&self.nodes, Vec::new(), state, &mut paths);
        paths
    }

    /// Move selection up
    pub fn select_previous(&self, state: &mut TreeViewState) {
        let visible_paths = self.get_visible_paths(state);
        if visible_paths.is_empty() {
            return;
        }

        if let Some(current_path) = &state.selected_path {
            if let Some(current_idx) = visible_paths.iter().position(|p| p == current_path) {
                if current_idx > 0 {
                    state.select(visible_paths[current_idx - 1].clone());
                }
            }
        } else {
            // Select first item
            state.select(visible_paths[0].clone());
        }
    }

    /// Move selection down
    pub fn select_next(&self, state: &mut TreeViewState) {
        let visible_paths = self.get_visible_paths(state);
        if visible_paths.is_empty() {
            return;
        }

        if let Some(current_path) = &state.selected_path {
            if let Some(current_idx) = visible_paths.iter().position(|p| p == current_path) {
                if current_idx < visible_paths.len() - 1 {
                    state.select(visible_paths[current_idx + 1].clone());
                }
            }
        } else {
            // Select first item
            state.select(visible_paths[0].clone());
        }
    }

    /// Toggle expansion of selected directory
    pub fn toggle_selected(&mut self, state: &mut TreeViewState) -> Result<()> {
        if let Some(path) = state.selected_path.clone() {
            if let Some(entry) = self.get_entry_at_path(&path) {
                if entry.is_dir {
                    if !state.is_expanded(&path) {
                        // Expand: load directory contents
                        self.expand_directory(&path)?;
                    }
                    // Toggle expansion state
                    state.toggle_expansion(path);
                }
            }
        }
        Ok(())
    }
}

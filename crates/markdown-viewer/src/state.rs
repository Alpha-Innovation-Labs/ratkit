//! Viewer state management

use std::path::{Path, PathBuf};

use crate::resizable_panes::ResizablePanesState;

/// Unified state for the markdown viewer
#[derive(Debug, Clone)]
pub struct ViewerState {
    /// Resizable panes state
    pub panes: ResizablePanesState,
    /// File tree root path
    pub file_tree_root: PathBuf,
    /// Currently selected file path
    pub selected_file: Option<PathBuf>,
    /// Last open file (for persistence)
    pub last_open_file: Option<PathBuf>,
    /// Saved split ratio (for persistence)
    pub saved_split_ratio: Option<f32>,
    /// Whether file tree is hidden
    pub file_tree_hidden: bool,
    /// File tree is focused
    pub file_tree_focused: bool,
    /// Markdown panel is focused
    pub markdown_focused: bool,
}

impl Default for ViewerState {
    fn default() -> Self {
        Self::new(".")
    }
}

impl ViewerState {
    /// Create a new viewer state
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            panes: ResizablePanesState::new(),
            file_tree_root: root.as_ref().to_path_buf(),
            selected_file: None,
            last_open_file: None,
            saved_split_ratio: None,
            file_tree_hidden: false,
            file_tree_focused: true,
            markdown_focused: false,
        }
    }

    /// Create with custom split ratio
    pub fn with_split_ratio(root: impl AsRef<Path>, ratio: f32) -> Self {
        let mut state = Self::new(root);
        state.panes.set_split_ratio(ratio);
        state
    }

    /// Select a file
    pub fn select_file(&mut self, path: &Path) {
        self.selected_file = Some(path.to_path_buf());
        self.last_open_file = Some(path.to_path_buf());
        self.focus_markdown();
    }

    /// Focus file tree
    pub fn focus_file_tree(&mut self) {
        self.file_tree_focused = true;
        self.markdown_focused = false;
    }

    /// Focus markdown panel
    pub fn focus_markdown(&mut self) {
        self.file_tree_focused = false;
        self.markdown_focused = true;
    }

    /// Toggle file tree focus
    pub fn toggle_focus(&mut self) {
        if self.file_tree_focused {
            self.focus_markdown();
        } else {
            self.focus_file_tree();
        }
    }

    /// Check if file tree is focused
    pub fn is_file_tree_focused(&self) -> bool {
        self.file_tree_focused
    }

    /// Check if markdown panel is focused
    pub fn is_markdown_focused(&self) -> bool {
        self.markdown_focused
    }

    /// Toggle file tree visibility
    pub fn toggle_file_tree(&mut self) {
        self.file_tree_hidden = !self.file_tree_hidden;
    }

    /// Check if file tree is hidden
    pub fn is_file_tree_hidden(&self) -> bool {
        self.file_tree_hidden
    }

    /// Get split ratio
    pub fn split_ratio(&self) -> f32 {
        self.panes.split_ratio()
    }

    /// Set split ratio
    pub fn set_split_ratio(&mut self, ratio: f32) {
        self.panes.set_split_ratio(ratio);
        self.saved_split_ratio = Some(ratio);
    }

    /// Save state (for persistence)
    pub fn save_state(&self) -> serde_json::Value {
        serde_json::json!({
            "last_open_file": self.last_open_file,
            "split_ratio": self.split_ratio(),
            "file_tree_hidden": self.file_tree_hidden,
        })
    }

    /// Load state (for persistence)
    pub fn load_state(&mut self, state: serde_json::Value) -> anyhow::Result<()> {
        if let Some(ratio) = state.get("split_ratio").and_then(|v| v.as_f64()) {
            self.set_split_ratio(ratio as f32);
        }

        if let Some(file) = state.get("last_open_file").and_then(|v| v.as_str()) {
            let path = PathBuf::from(file);
            if path.exists() {
                self.last_open_file = Some(path);
            }
        }

        if let Some(hidden) = state.get("file_tree_hidden").and_then(|v| v.as_bool()) {
            self.file_tree_hidden = hidden;
        }

        Ok(())
    }
}

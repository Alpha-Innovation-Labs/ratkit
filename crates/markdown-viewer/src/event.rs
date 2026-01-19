//! Viewer events

use std::path::PathBuf;

use ratatui::crossterm::event::KeyCode;

/// Events emitted by viewer to parent application
#[derive(Debug, Clone, PartialEq)]
pub enum ViewerEvent {
    /// A file was selected in the file tree
    FileSelected { path: PathBuf },

    /// A file was loaded into the markdown panel
    FileLoaded { path: PathBuf },

    /// Panes were resized
    PanesResized { ratio: f32 },

    /// File tree visibility toggled
    FileTreeHidden { hidden: bool },

    /// No event
    None,
}

impl From<KeyCode> for ViewerEvent {
    fn from(_key: KeyCode) -> Self {
        ViewerEvent::None
    }
}

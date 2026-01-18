//! Get all tabs and tab names.

use super::super::DemoTab;

impl DemoTab {
    /// Get all available demo tabs.
    pub fn all() -> Vec<Self> {
        vec![
            Self::Markdown,
            Self::CodeDiff,
            Self::Tree,
            Self::Dialogs,
            Self::Scrollbar,
            Self::StatusLine,
            Self::Terminal,
        ]
    }

    /// Get the display name for this tab.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Markdown => "Markdown",
            Self::CodeDiff => "Code Diff",
            Self::Tree => "Tree View",
            Self::Dialogs => "Dialogs",
            Self::Scrollbar => "Scrollbar",
            Self::StatusLine => "StatusLine",
            Self::Terminal => "Terminal",
        }
    }
}

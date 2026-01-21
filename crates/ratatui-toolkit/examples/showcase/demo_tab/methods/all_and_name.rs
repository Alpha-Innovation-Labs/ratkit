//! Get all tabs and tab names.

use super::super::DemoTab;

impl DemoTab {
    /// Get all available demo tabs.
    pub fn all() -> Vec<Self> {
        vec![
            Self::Markdown,
            Self::CodeDiff,
            Self::Tree,
            Self::Terminal,
            Self::SplitLayoutGrid,
            Self::AiChat,
        ]
    }

    /// Get display name for this tab.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Markdown => "Markdown",
            Self::CodeDiff => "Code Diff",
            Self::Tree => "Trees",
            Self::Terminal => "Terminal",
            Self::SplitLayoutGrid => "Split Grid",
            Self::AiChat => "AI Chat",
        }
    }
}

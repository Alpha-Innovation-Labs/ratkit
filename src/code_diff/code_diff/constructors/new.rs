use std::collections::HashMap;

use crate::code_diff::code_diff::CodeDiff;
use crate::code_diff::diff_config::DiffConfig;
use crate::diff_file_tree::DiffFileTree;
use crate::resizable_split::ResizableSplit;
use crate::theme::AppTheme;

impl CodeDiff {
    /// Creates a new empty diff widget.
    ///
    /// # Returns
    ///
    /// A new `CodeDiff` instance with no hunks and default configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// let diff = CodeDiff::new();
    /// assert!(diff.hunks.is_empty());
    /// ```
    pub fn new() -> Self {
        let config = DiffConfig::new();
        let mut sidebar_split = ResizableSplit::new(config.sidebar_default_width);
        sidebar_split.min_percent = config.sidebar_min_width;
        sidebar_split.max_percent = config.sidebar_max_width;
        Self {
            file_path: None,
            hunks: Vec::new(),
            scroll_offset: 0,
            file_tree: DiffFileTree::new(),
            file_diffs: HashMap::new(),
            show_sidebar: config.sidebar_enabled,
            sidebar_split,
            sidebar_focused: true,
            config,
            theme: AppTheme::default(),
        }
    }
}

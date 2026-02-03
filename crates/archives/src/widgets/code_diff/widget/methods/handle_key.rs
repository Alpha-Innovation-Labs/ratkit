//! Method to handle keyboard input.

use crossterm::event::KeyCode;

use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Handles a keyboard event and returns whether it was consumed.
    ///
    /// # Key Bindings
    ///
    /// - `[` - Toggle sidebar visibility
    /// - `Tab` - Switch focus between sidebar and diff
    /// - `/` - Enter filter mode (sidebar only)
    /// - `h` - Collapse expanded directory, or go to parent if collapsed (sidebar only, no-op on files)
    /// - `l` - Expand collapsed directory, or show diff for files (sidebar only)
    /// - `j` / `Down` - Navigate down (files in sidebar, scroll in diff)
    /// - `k` / `Up` - Navigate up (files in sidebar, scroll in diff)
    /// - `g` - Go to top (first file or top of diff)
    /// - `G` - Go to bottom (last file or bottom of diff)
    /// - `Space` / `Enter` - Toggle directory expand / select file (sidebar only)
    /// - `H` / `<` - Decrease sidebar width
    /// - `L` / `>` - Increase sidebar width
    /// - `r` - Refresh diff from git
    ///
    /// # Filter Mode
    ///
    /// When filter mode is active (triggered by `/`):
    /// - `Esc` - Clear filter and exit filter mode
    /// - `Enter` - Exit filter mode (keep filter active)
    /// - `Backspace` - Delete last character
    /// - Any character - Append to filter
    ///
    /// # Arguments
    ///
    /// * `key` - The key code that was pressed
    ///
    /// # Returns
    ///
    /// `true` if the key was handled, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use crossterm::event::KeyCode;
    /// use ratatui_toolkit::code_diff::{CodeDiff, DiffConfig};
    ///
    /// let mut diff = CodeDiff::new()
    ///     .with_config(DiffConfig::new().sidebar_enabled(true));
    ///
    /// // Toggle sidebar
    /// diff.handle_key(KeyCode::Char('['));
    ///
    /// // Navigate
    /// diff.handle_key(KeyCode::Char('j'));
    ///
    /// // Enter filter mode
    /// diff.handle_key(KeyCode::Char('/'));
    /// ```
    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        // If in filter mode, delegate to file tree's filter handler
        if self.file_tree.is_filter_mode() {
            return self.file_tree.handle_filter_key(key);
        }

        match key {
            // Toggle sidebar
            KeyCode::Char('[') => {
                self.toggle_sidebar();
                true
            }

            // Focus switching - Tab only
            KeyCode::Tab => {
                if self.show_sidebar && self.config.sidebar_enabled {
                    self.toggle_focus();
                }
                true
            }

            // Vim-style: h = collapse directory (sidebar only)
            // On expanded directory: collapse it
            // On collapsed directory: go to parent
            // On file: do nothing (files can't be collapsed)
            KeyCode::Char('h') => {
                if self.sidebar_focused && self.show_sidebar {
                    // Check if current selection is a directory
                    if let Some(is_dir) = self.file_tree.selected_is_dir() {
                        if is_dir {
                            // It's a directory - try to collapse it
                            // If already collapsed, go to parent
                            if !self.file_tree.collapse_selected() {
                                self.file_tree.go_to_parent();
                            }
                        }
                        // For files, do nothing - h only affects directories
                    }
                    true
                } else {
                    false
                }
            }

            // Vim-style: l = expand directory or show diff (sidebar only)
            // On collapsed directory: expand it
            // On expanded directory: descend (select first child) or show diff if no children
            // On file: show the diff
            KeyCode::Char('l') | KeyCode::Enter => {
                if self.sidebar_focused && self.show_sidebar {
                    // Check if current selection is a directory
                    if let Some(is_dir) = self.file_tree.selected_is_dir() {
                        if is_dir {
                            // It's a directory - try to expand it
                            // If already expanded, we could descend but for now just expand
                            self.file_tree.expand_selected();
                        } else {
                            // It's a file - show the diff
                            self.sync_diff_from_selection();
                        }
                    }
                    true
                } else {
                    false
                }
            }

            // Navigation down
            KeyCode::Char('j') | KeyCode::Down => {
                if self.sidebar_focused && self.show_sidebar {
                    self.select_next_file();
                } else {
                    self.scroll_down(1);
                }
                true
            }

            // Navigation up
            KeyCode::Char('k') | KeyCode::Up => {
                if self.sidebar_focused && self.show_sidebar {
                    self.select_prev_file();
                } else {
                    self.scroll_up(1);
                }
                true
            }

            // Go to top
            KeyCode::Char('g') => {
                if self.sidebar_focused && self.show_sidebar {
                    self.file_tree.set_selected_index(0);
                    self.sync_diff_from_selection();
                } else {
                    self.scroll_offset = 0;
                }
                true
            }

            // Go to bottom
            KeyCode::Char('G') => {
                if self.sidebar_focused && self.show_sidebar {
                    let count = self.file_tree.visible_count();
                    self.file_tree.set_selected_index(count.saturating_sub(1));
                    self.sync_diff_from_selection();
                } else {
                    let total = self.total_lines();
                    self.scroll_offset = total.saturating_sub(1);
                }
                true
            }

            // Toggle directory expand (sidebar only)
            KeyCode::Char(' ') => {
                if self.sidebar_focused && self.show_sidebar {
                    self.file_tree.toggle_expand();
                    true
                } else {
                    false
                }
            }

            // Resize sidebar narrower
            KeyCode::Char('H') | KeyCode::Char('<') => {
                if self.config.sidebar_enabled {
                    self.resize_sidebar(-5);
                }
                true
            }

            // Resize sidebar wider
            KeyCode::Char('L') | KeyCode::Char('>') => {
                if self.config.sidebar_enabled {
                    self.resize_sidebar(5);
                }
                true
            }

            // Refresh diff from git
            KeyCode::Char('r') => {
                self.refresh();
                true
            }

            // Enter filter mode (sidebar only)
            KeyCode::Char('/') => {
                if self.sidebar_focused && self.show_sidebar {
                    self.file_tree.enter_filter_mode();
                    true
                } else {
                    false
                }
            }

            _ => false,
        }
    }
}

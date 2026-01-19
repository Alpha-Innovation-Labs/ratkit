//! Main viewer component

use std::path::Path;

use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};

use ratatui_toolkit::{render_markdown, FileSystemTree, TreeViewState};

use crate::config::ViewerConfig;
use crate::event::ViewerEvent;
use crate::persistence::Persistence;
use crate::resizable_panes::{DividerWidget, ResizablePanes};
use crate::state::ViewerState;

/// Main viewer component
pub struct Viewer<'a> {
    config: ViewerConfig,
    state: &'a mut ViewerState,
    current_content: Option<String>,
    file_tree: Option<FileSystemTree<'a>>,
    file_tree_state: TreeViewState,
}

impl<'a> Viewer<'a> {
    /// Create a new viewer
    pub fn new(state: &'a mut ViewerState) -> Self {
        Self::with_config(state, ViewerConfig::default())
    }

    /// Create with configuration
    pub fn with_config(state: &'a mut ViewerState, config: ViewerConfig) -> Self {
        let file_tree = FileSystemTree::new(state.file_tree_root.clone()).ok();

        Self {
            config,
            state,
            current_content: None,
            file_tree,
            file_tree_state: TreeViewState::default(),
        }
    }

    /// Set theme
    pub fn with_theme(mut self, theme: AppTheme) -> Self {
        self.config = self.config.with_theme(theme);
        self
    }

    /// Load a file
    pub fn load_file(&mut self, path: &Path) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(path)?;
        self.current_content = Some(content);
        self.state.select_file(path);
        Ok(())
    }

    /// Get current content
    pub fn current_content(&self) -> Option<&str> {
        self.current_content.as_deref()
    }

    /// Handle key event
    pub fn handle_key_event(&mut self, key: KeyEvent) -> ViewerEvent {
        // Global key: toggle file tree
        if key.code == self.config.hide_file_tree_key {
            self.state.toggle_file_tree();
            return ViewerEvent::FileTreeHidden {
                hidden: self.state.is_file_tree_hidden(),
            };
        }

        // Route to focused component
        if self.state.is_file_tree_focused() && !self.state.is_file_tree_hidden() {
            // File tree focused
            match key.code {
                KeyCode::Right | KeyCode::Char('l') => {
                    // Move to markdown
                    self.state.toggle_focus();
                    ViewerEvent::None
                }
                KeyCode::Enter => {
                    // Select file (placeholder)
                    ViewerEvent::None
                }
                _ => ViewerEvent::None,
            }
        } else {
            // Markdown focused
            match key.code {
                KeyCode::Left | KeyCode::Char('h') => {
                    // Move to file tree
                    if !self.state.is_file_tree_hidden() {
                        self.state.toggle_focus();
                    }
                    ViewerEvent::None
                }
                KeyCode::Char('/') => {
                    // Enter filter mode
                    ViewerEvent::None
                }
                _ => ViewerEvent::None,
            }
        }
    }

    /// Handle mouse event
    pub fn handle_mouse_event(&mut self, _event: MouseEvent) -> ViewerEvent {
        // Placeholder for mouse handling
        ViewerEvent::None
    }

    /// Render the viewer
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        if self.state.is_file_tree_hidden() {
            self.render_markdown_panel(area, buf);
        } else {
            self.render_split_view(area, buf);
        }
    }

    /// Render split view
    fn render_split_view(&mut self, area: Rect, buf: &mut Buffer) {
        let panes = ResizablePanes::new()
            .with_split_ratio(self.state.split_ratio())
            .with_min_width(self.config.min_pane_width)
            .with_max_width(self.config.max_pane_width.unwrap_or(area.width));

        let (left_area, right_area, divider_area) = panes.calculate_areas(area);

        // Render file tree (left)
        self.render_file_tree(left_area, buf);

        // Render divider
        DividerWidget::default().render(divider_area, buf);

        // Render markdown panel (right)
        self.render_markdown_panel(right_area, buf);
    }

    /// Render file tree
    fn render_file_tree(&mut self, area: Rect, buf: &mut Buffer) {
        if let Some(ref file_tree) = self.file_tree {
            let tree = file_tree.clone();
            tree.render(area, buf, &mut self.file_tree_state);
        } else {
            let block = Block::default()
                .title("File Tree")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));

            let text = Paragraph::new("Loading...")
                .block(block)
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Center);

            text.render(area, buf);
        }
    }

    /// Render markdown panel
    fn render_markdown_panel(&mut self, area: Rect, buf: &mut Buffer) {
        if let Some(content) = &self.current_content {
            use ratatui_toolkit::render_markdown;

            let text = render_markdown(content, Some(area.width as usize));

            let block = Block::default()
                .title("Markdown Viewer")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));

            let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });

            paragraph.render(area, buf);
        } else {
            let block = Block::default()
                .title("Markdown Viewer")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));

            let text = Paragraph::new("Select a markdown file to view")
                .block(block)
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Center);

            text.render(area, buf);
        }
    }

    /// Save state to disk
    pub fn save_state(&self) -> anyhow::Result<()> {
        let persistence = Persistence::new();
        let state = self.state.save_state();
        persistence.save_state(state)?;
        Ok(())
    }

    /// Load state from disk
    pub fn load_state(&mut self) -> anyhow::Result<()> {
        let persistence = Persistence::new();
        let state = persistence.load_state()?;
        self.state.load_state(state)?;

        // Load last open file if exists
        if let Some(ref path) = self.state.last_open_file.clone() {
            if path.exists() {
                let _ = self.load_file(path);
            }
        }

        Ok(())
    }

    /// Get state reference
    pub fn state(&self) -> &ViewerState {
        self.state
    }

    /// Get state mutable reference
    pub fn state_mut(&mut self) -> &mut ViewerState {
        self.state
    }
}

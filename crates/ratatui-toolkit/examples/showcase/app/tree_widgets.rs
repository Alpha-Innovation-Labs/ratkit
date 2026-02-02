//! Tree widget wrappers that own their state internally.
//!
//! These wrappers follow the pattern from CodeDiff where the widget owns
//! all its state internally and exposes simple handle_key() and handle_mouse()
//! methods without requiring external state parameters.

use crossterm::event::{KeyCode, MouseEvent};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
};
use ratatui_toolkit::primitives::tree_view::{
    TreeNavigator, TreeNode, TreeView as TreeViewBase, TreeViewRef, TreeViewState,
};
use ratatui_toolkit::FileSystemEntry;
use ratatui_toolkit::FileSystemTree;

const FILE_TREE_FILTER_UI_HEIGHT: u16 = 1;

pub struct FileSystemTreeWidget {
    pub tree: Option<FileSystemTree<'static>>,
    pub state: TreeViewState,
    pub navigator: TreeNavigator,
    pub area: Option<Rect>,
}

impl FileSystemTreeWidget {
    pub fn new(path: std::path::PathBuf) -> Self {
        let tree = FileSystemTree::new(path).ok();
        let mut state = TreeViewState::new();
        state.select(vec![0]);
        Self {
            tree,
            state,
            navigator: TreeNavigator::new(),
            area: None,
        }
    }

    pub fn with_theme(mut self, _theme: &ratatui_toolkit::AppTheme) -> Self {
        self
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        let Some(ref tree) = self.tree else {
            return;
        };

        if self.state.filter_mode {
            self.handle_filter_key(key);
        } else if key == KeyCode::Char('/') {
            self.state.enter_filter_mode();
        } else {
            self.navigator.handle_key(key, &tree.nodes, &mut self.state);
        }
    }

    fn handle_filter_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.state.clear_filter();
            }
            KeyCode::Enter => {
                self.state.exit_filter_mode();
            }
            KeyCode::Backspace => {
                self.state.backspace_filter();
            }
            KeyCode::Char(c) => {
                self.state.append_to_filter(c);
            }
            _ => {}
        }
    }

    pub fn handle_mouse(&mut self, event: MouseEvent) {
        let Some(area) = self.area else {
            return;
        };

        let Some(ref tree) = self.tree else {
            return;
        };

        let inner_area = match tree.block {
            Some(ref block) => block.inner(area),
            None => area,
        };

        if inner_area.height == 0 {
            return;
        }

        let y = event.row;
        if y < inner_area.y || y >= inner_area.y + inner_area.height {
            return;
        }

        let row = (y - inner_area.y + self.state.offset as u16) as usize;
        let paths = tree.get_visible_paths(&self.state);

        if let Some(path) = paths.get(row) {
            self.state.selected_path = Some(path.clone());
        }
    }

    pub fn render(
        &mut self,
        frame: &mut ratatui::Frame,
        area: Rect,
        block: ratatui::widgets::Block<'static>,
    ) {
        self.area = Some(area);

        let Some(ref mut tree) = self.tree else {
            return;
        };

        let filter_ui_height = if self.state.filter_mode {
            FILE_TREE_FILTER_UI_HEIGHT
        } else {
            0
        };

        let render_area = Rect {
            height: area.height.saturating_sub(filter_ui_height),
            ..area
        };

        let widget = tree.clone().block(block);

        frame.render_stateful_widget(widget, render_area, &mut self.state);

        if self.state.filter_mode {
            let filter_area = Rect {
                y: area.y + area.height - filter_ui_height,
                height: filter_ui_height,
                ..area
            };
            let filter_text = format!(
                " Filter: {} ",
                self.state.filter.clone().unwrap_or_default()
            );
            let filter_line = Line::from(filter_text).style(
                Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );
            frame.render_widget(ratatui::widgets::Paragraph::new(filter_line), filter_area);
        }
    }

    pub fn filter_mode(&self) -> bool {
        self.state.filter_mode
    }
}

pub struct TreeViewWidget {
    pub tree: TreeViewRef<'static>,
    pub state: TreeViewState,
    pub navigator: TreeNavigator,
    pub area: Option<Rect>,
}

impl TreeViewWidget {
    pub fn new(nodes: &[TreeNode<String>]) -> Self {
        let tree = TreeViewRef::new(nodes.to_vec())
            .highlight_style(Style::default().bg(ratatui::style::Color::DarkGray))
            .render_fn(move |data: &String, state| {
                let style = if state.is_selected {
                    Style::default()
                        .fg(ratatui::style::Color::LightBlue)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                Line::styled(data.clone(), style)
            })
            .with_filter_ui(false);

        let mut state = TreeViewState::new();
        state.select(vec![0]);

        Self {
            tree,
            state,
            navigator: TreeNavigator::new(),
            area: None,
        }
    }

    pub fn with_theme(mut self, theme: &ratatui_toolkit::AppTheme) -> Self {
        self.tree = self
            .tree
            .highlight_style(Style::default().bg(theme.background_panel));
        self
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        if self.state.filter_mode {
            self.handle_filter_key(key);
        } else if key == KeyCode::Char('/') {
            self.state.enter_filter_mode();
        } else {
            self.navigator
                .handle_key(key, &self.tree.nodes, &mut self.state);
        }
    }

    fn handle_filter_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.state.clear_filter();
            }
            KeyCode::Enter => {
                self.state.exit_filter_mode();
            }
            KeyCode::Backspace => {
                self.state.backspace_filter();
            }
            KeyCode::Char(c) => {
                self.state.append_to_filter(c);
            }
            _ => {}
        }
    }

    pub fn handle_mouse(&mut self, event: MouseEvent) {
        let Some(area) = self.area else {
            return;
        };

        let inner_area = match self.tree.block {
            Some(ref block) => block.inner(area),
            None => area,
        };

        if inner_area.height == 0 {
            return;
        }

        let y = event.row;
        if y < inner_area.y || y >= inner_area.y + inner_area.height {
            return;
        }

        let row = (y - inner_area.y + self.state.offset as u16) as usize;
        let paths = self.tree.get_visible_paths(&self.state);

        if let Some(path) = paths.get(row) {
            self.state.selected_path = Some(path.clone());
        }
    }

    pub fn render(&mut self, frame: &mut ratatui::Frame, area: Rect) {
        self.area = Some(area);
        frame.render_stateful_widget(self.tree.clone(), area, &mut self.state);
    }

    pub fn filter_mode(&self) -> bool {
        self.state.filter_mode
    }
}

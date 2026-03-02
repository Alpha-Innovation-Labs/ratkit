use std::fs;
use std::path::Path;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Widget};

use crate::widgets::file_system_tree::config::FileSystemTreeConfig;
use crate::widgets::file_system_tree::entry::FileSystemEntry;
use crate::widgets::file_system_tree::state::FileSystemTreeState;
use crate::widgets::file_system_tree::tree_node::FileSystemTreeNode;
use devicons::{icon_for_file, Theme as DevIconTheme};

fn parse_hex_color(color: &str) -> Option<Color> {
    let hex = color.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}

fn yazi_dir_icon_color() -> Color {
    parse_hex_color("#03a9f4").unwrap_or(Color::Blue)
}

#[derive(Clone)]
pub struct FileSystemTree<'a> {
    pub root_path: std::path::PathBuf,
    pub nodes: Vec<FileSystemTreeNode>,
    pub config: FileSystemTreeConfig,
    pub block: Option<Block<'a>>,
}

impl<'a> FileSystemTree<'a> {
    pub fn new(root_path: std::path::PathBuf) -> std::io::Result<Self> {
        let config = FileSystemTreeConfig::default();
        let root_entry = FileSystemEntry::new(root_path.clone())?;
        let root_children = if root_entry.is_dir {
            Self::load_directory(&root_path, &config)?
        } else {
            Vec::new()
        };
        let nodes = vec![FileSystemTreeNode {
            data: root_entry,
            children: root_children,
            expandable: root_path.is_dir(),
        }];

        Ok(Self {
            root_path,
            nodes,
            config,
            block: None,
        })
    }

    pub fn with_config(
        root_path: std::path::PathBuf,
        config: FileSystemTreeConfig,
    ) -> std::io::Result<Self> {
        let root_entry = FileSystemEntry::new(root_path.clone())?;
        let root_children = if root_entry.is_dir {
            Self::load_directory(&root_path, &config)?
        } else {
            Vec::new()
        };
        let nodes = vec![FileSystemTreeNode {
            data: root_entry,
            children: root_children,
            expandable: root_path.is_dir(),
        }];

        Ok(Self {
            root_path,
            nodes,
            config,
            block: None,
        })
    }

    fn load_directory(
        path: &Path,
        config: &FileSystemTreeConfig,
    ) -> std::io::Result<Vec<FileSystemTreeNode>> {
        let mut entries = Vec::new();

        let read_dir = fs::read_dir(path)?;

        for entry in read_dir {
            let entry = entry?;
            let path = entry.path();

            let fs_entry = FileSystemEntry::new(path.clone())?;

            if fs_entry.is_hidden && !config.show_hidden {
                continue;
            }

            let node = if fs_entry.is_dir {
                FileSystemTreeNode {
                    data: fs_entry,
                    children: Vec::new(),
                    expandable: true,
                }
            } else {
                FileSystemTreeNode::new(fs_entry)
            };

            entries.push(node);
        }

        entries.sort_by(|a, b| match (a.data.is_dir, b.data.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase()),
        });

        Ok(entries)
    }

    pub fn expand_directory(&mut self, path: &[usize]) -> std::io::Result<()> {
        fn find_and_expand(
            nodes: &mut [FileSystemTreeNode],
            path: &[usize],
            config: &FileSystemTreeConfig,
        ) -> std::io::Result<()> {
            if path.is_empty() {
                return Ok(());
            }

            if path.len() == 1 {
                if let Some(node) = nodes.get_mut(path[0]) {
                    if node.data.is_dir && node.children.is_empty() {
                        node.children = FileSystemTree::load_directory(&node.data.path, config)?;
                    }
                }
                return Ok(());
            }

            if let Some(node) = nodes.get_mut(path[0]) {
                find_and_expand(&mut node.children, &path[1..], config)?;
            }

            Ok(())
        }

        find_and_expand(&mut self.nodes, path, &self.config)
    }

    pub fn get_entry_at_path(&self, path: &[usize]) -> Option<&FileSystemEntry> {
        fn find_entry<'a>(
            nodes: &'a [FileSystemTreeNode],
            path: &[usize],
        ) -> Option<&'a FileSystemEntry> {
            if path.is_empty() {
                return None;
            }

            if let Some(node) = nodes.get(path[0]) {
                if path.len() == 1 {
                    return Some(&node.data);
                }
                return find_entry(&node.children, &path[1..]);
            }
            None
        }

        find_entry(&self.nodes, path)
    }

    pub fn get_selected_entry(&self, state: &FileSystemTreeState) -> Option<&FileSystemEntry> {
        state
            .selected_path
            .as_ref()
            .and_then(|path| self.get_entry_at_path(path))
    }

    pub fn get_visible_paths(&self, state: &FileSystemTreeState) -> Vec<Vec<usize>> {
        let mut paths = Vec::new();

        fn traverse(
            nodes: &[FileSystemTreeNode],
            current_path: Vec<usize>,
            state: &FileSystemTreeState,
            paths: &mut Vec<Vec<usize>>,
        ) {
            for (idx, node) in nodes.iter().enumerate() {
                let mut path = current_path.clone();
                path.push(idx);
                paths.push(path.clone());

                if state.is_expanded(&path) && !node.children.is_empty() {
                    traverse(&node.children, path, state, paths);
                }
            }
        }

        traverse(&self.nodes, Vec::new(), state, &mut paths);
        paths
    }

    pub fn select_next(&mut self, state: &mut FileSystemTreeState) {
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
            state.select(visible_paths[0].clone());
        }
    }

    pub fn select_previous(&mut self, state: &mut FileSystemTreeState) {
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
            state.select(visible_paths[0].clone());
        }
    }

    pub fn toggle_selected(&mut self, state: &mut FileSystemTreeState) -> std::io::Result<()> {
        if let Some(path) = state.selected_path.clone() {
            if let Some(entry) = self.get_entry_at_path(&path) {
                if entry.is_dir {
                    if !state.is_expanded(&path) {
                        self.expand_directory(&path)?;
                    }
                    state.toggle_expansion(path);
                }
            }
        }
        Ok(())
    }

    pub fn expand_selected(&mut self, state: &mut FileSystemTreeState) -> std::io::Result<bool> {
        let Some(path) = state.selected_path.clone() else {
            return Ok(false);
        };

        let Some(entry) = self.get_entry_at_path(&path) else {
            return Ok(false);
        };

        if !entry.is_dir {
            return Ok(false);
        }

        if !state.is_expanded(&path) {
            self.expand_directory(&path)?;
            state.expand(path);
            return Ok(true);
        }

        Ok(false)
    }

    pub fn collapse_selected(&mut self, state: &mut FileSystemTreeState) -> bool {
        let Some(path) = state.selected_path.clone() else {
            return false;
        };

        if state.is_expanded(&path) {
            state.collapse(path);
            return true;
        }

        if path.len() > 1 {
            let mut parent = path;
            parent.pop();
            state.select(parent);
            return true;
        }

        false
    }

    pub fn handle_navigation_key(
        &mut self,
        key: crossterm::event::KeyCode,
        state: &mut FileSystemTreeState,
    ) -> std::io::Result<bool> {
        match key {
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                self.select_next(state);
                Ok(true)
            }
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                self.select_previous(state);
                Ok(true)
            }
            crossterm::event::KeyCode::Enter => {
                self.toggle_selected(state)?;
                Ok(true)
            }
            crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('l') => {
                if self.expand_selected(state)? {
                    return Ok(true);
                }

                if let Some(path) = state.selected_path.clone() {
                    let mut first_child = path.clone();
                    first_child.push(0);
                    if self.get_entry_at_path(&first_child).is_some() {
                        state.select(first_child);
                        return Ok(true);
                    }
                }

                Ok(false)
            }
            crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('h') => {
                Ok(self.collapse_selected(state))
            }
            _ => Ok(false),
        }
    }

    pub fn enter_filter_mode(&self, state: &mut FileSystemTreeState) {
        state.enter_filter_mode();
    }

    pub fn is_filter_mode(&self, state: &FileSystemTreeState) -> bool {
        state.is_filter_mode()
    }

    pub fn filter_text<'s>(&self, state: &'s FileSystemTreeState) -> Option<&'s str> {
        state.filter_text()
    }

    pub fn clear_filter(&self, state: &mut FileSystemTreeState) {
        state.clear_filter();
    }

    pub fn handle_filter_key(
        &self,
        key: crossterm::event::KeyCode,
        state: &mut FileSystemTreeState,
    ) -> bool {
        match key {
            crossterm::event::KeyCode::Esc => {
                state.exit_filter_mode();
                true
            }
            crossterm::event::KeyCode::Enter => {
                state.exit_filter_mode();
                true
            }
            crossterm::event::KeyCode::Backspace => {
                state.pop_filter();
                true
            }
            crossterm::event::KeyCode::Char(c) => {
                state.push_filter(c);
                true
            }
            _ => false,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> ratatui::widgets::StatefulWidget for FileSystemTree<'a> {
    type State = FileSystemTreeState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.expanded.is_empty() && !self.nodes.is_empty() {
            state.expand(vec![0]);
        }

        let config = self.config;
        let block = self.block.take();
        let filter_mode = state.filter_mode;
        let has_filter = state.filter.as_ref().is_some_and(|f| !f.is_empty());
        let show_filter_line = filter_mode || has_filter;

        let tree_area = if show_filter_line && area.height > 1 {
            Rect {
                height: area.height - 1,
                ..area
            }
        } else {
            area
        };

        let visible_paths = self.get_visible_paths(state);
        let visible_count = visible_paths.len();
        let offset = state.offset.min(visible_count.saturating_sub(1));
        let visible_paths: Vec<_> = visible_paths.into_iter().skip(offset).collect();

        let tree_area = if let Some(block) = block {
            let inner = block.inner(tree_area);
            block.render(tree_area, buf);
            inner
        } else {
            tree_area
        };

        for (row, path) in visible_paths.iter().enumerate() {
            let y = tree_area.y + row as u16;
            if y >= tree_area.y + tree_area.height {
                break;
            }

            if let Some(entry) = self.get_entry_at_path(path) {
                let is_selected = state.selected_path.as_ref() == Some(path);

                let (icon_glyph, icon_color) = if entry.is_dir {
                    if state.is_expanded(path) {
                        ('\u{f115}', yazi_dir_icon_color())
                    } else {
                        ('\u{f114}', yazi_dir_icon_color())
                    }
                } else {
                    let theme = if config.use_dark_theme {
                        DevIconTheme::Dark
                    } else {
                        DevIconTheme::Light
                    };
                    let file_icon = icon_for_file(&entry.name, &Some(theme));
                    let icon_char = file_icon.icon;
                    let color = parse_hex_color(file_icon.color).unwrap_or(Color::White);
                    (icon_char, color)
                };

                let style = if is_selected {
                    config.selected_style
                } else if entry.is_dir {
                    config.dir_style
                } else {
                    config.file_style
                };

                let selected_bg = if entry.is_dir {
                    config.dir_style.fg.unwrap_or(Color::Blue)
                } else {
                    config.file_style.fg.unwrap_or(Color::White)
                };
                let selected_text_style = Style::default().fg(Color::Black).bg(selected_bg);

                let depth = path.len().saturating_sub(1);
                let indent = "  ".repeat(depth);

                let (line_x, line_width) = if tree_area.width > 2 {
                    let left_x = tree_area.x;
                    let right_x = tree_area.x + tree_area.width - 1;

                    if is_selected {
                        buf[(left_x, y)]
                            .set_symbol("")
                            .set_style(Style::default().fg(selected_bg));
                        buf[(right_x, y)]
                            .set_symbol("")
                            .set_style(Style::default().fg(selected_bg));

                        for x in (left_x + 1)..right_x {
                            buf[(x, y)].set_style(selected_text_style);
                        }
                    } else {
                        buf[(left_x, y)].set_symbol(" ").set_style(Style::default());
                        buf[(right_x, y)]
                            .set_symbol(" ")
                            .set_style(Style::default());
                    }

                    (left_x + 1, tree_area.width - 2)
                } else {
                    if is_selected {
                        for x in tree_area.x..(tree_area.x + tree_area.width) {
                            buf[(x, y)].set_style(selected_text_style);
                        }
                    }
                    (tree_area.x, tree_area.width)
                };

                let line = if is_selected {
                    Line::from(vec![
                        Span::styled(indent, selected_text_style),
                        Span::styled(format!("{} ", icon_glyph), selected_text_style),
                        Span::styled(entry.name.clone(), selected_text_style),
                    ])
                } else {
                    Line::from(vec![
                        Span::raw(indent),
                        Span::styled(format!("{} ", icon_glyph), Style::default().fg(icon_color)),
                        Span::styled(entry.name.clone(), style),
                    ])
                };

                buf.set_line(line_x, y, &line, line_width);
            }
        }

        if show_filter_line && area.height > 1 {
            let y = area.y + area.height - 1;
            let filter_str = state.filter_text().unwrap_or("");
            let cursor = if filter_mode { "_" } else { "" };

            let line = Line::from(vec![
                Span::styled(
                    "/ ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(filter_str, Style::default().fg(Color::White)),
                Span::styled(
                    cursor,
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]);

            let bg_style = Style::default();
            for x in area.x..(area.x + area.width) {
                buf[(x, y)].set_style(bg_style);
            }

            buf.set_line(area.x, y, &line, area.width);
        }
    }
}

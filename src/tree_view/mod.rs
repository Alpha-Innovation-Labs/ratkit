mod keybindings;

pub use keybindings::TreeKeyBindings;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, StatefulWidget, Widget},
};
use std::collections::HashSet;

/// A node in the tree
#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    /// Node data
    pub data: T,
    /// Child nodes
    pub children: Vec<TreeNode<T>>,
    /// Whether this node can be expanded (has children)
    pub expandable: bool,
}

impl<T> TreeNode<T> {
    /// Create a new tree node
    pub fn new(data: T) -> Self {
        Self {
            data,
            children: Vec::new(),
            expandable: false,
        }
    }

    /// Create a new tree node with children
    pub fn with_children(data: T, children: Vec<TreeNode<T>>) -> Self {
        let expandable = !children.is_empty();
        Self {
            data,
            children,
            expandable,
        }
    }
}

/// State information for rendering a node
#[derive(Debug, Clone)]
pub struct NodeState {
    /// Whether this node is selected
    pub is_selected: bool,
    /// Whether this node is expanded
    pub is_expanded: bool,
    /// Depth level in the tree (0 = root)
    pub level: usize,
    /// Whether this node has children
    pub has_children: bool,
    /// Path to this node (indices from root)
    pub path: Vec<usize>,
}

/// Type alias for node render function to reduce complexity
pub type NodeRenderFn<'a, T> = Box<dyn Fn(&T, &NodeState) -> Line<'a> + 'a>;

/// Tree view state (for StatefulWidget pattern)
#[derive(Debug, Clone, Default)]
pub struct TreeViewState {
    /// Currently selected node path (indices from root)
    pub selected_path: Option<Vec<usize>>,
    /// Set of expanded node paths
    pub expanded: HashSet<Vec<usize>>,
    /// Vertical scroll offset
    pub offset: usize,
}

impl TreeViewState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the selected node path
    pub fn select(&mut self, path: Vec<usize>) {
        self.selected_path = Some(path);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected_path = None;
    }

    /// Toggle expansion of a node at the given path
    pub fn toggle_expansion(&mut self, path: Vec<usize>) {
        if self.expanded.contains(&path) {
            self.expanded.remove(&path);
        } else {
            self.expanded.insert(path);
        }
    }

    /// Check if a node is expanded
    pub fn is_expanded(&self, path: &[usize]) -> bool {
        self.expanded.contains(path)
    }

    /// Expand a node
    pub fn expand(&mut self, path: Vec<usize>) {
        self.expanded.insert(path);
    }

    /// Collapse a node
    pub fn collapse(&mut self, path: Vec<usize>) {
        self.expanded.remove(&path);
    }

    /// Expand all nodes
    pub fn expand_all<T>(&mut self, nodes: &[TreeNode<T>]) {
        fn collect_paths<T>(
            nodes: &[TreeNode<T>],
            current_path: Vec<usize>,
            expanded: &mut HashSet<Vec<usize>>,
        ) {
            for (idx, node) in nodes.iter().enumerate() {
                let mut path = current_path.clone();
                path.push(idx);

                if node.expandable {
                    expanded.insert(path.clone());
                }

                if !node.children.is_empty() {
                    collect_paths(&node.children, path, expanded);
                }
            }
        }

        collect_paths(nodes, Vec::new(), &mut self.expanded);
    }

    /// Collapse all nodes
    pub fn collapse_all(&mut self) {
        self.expanded.clear();
    }
}

/// Tree view widget
pub struct TreeView<'a, T> {
    /// Root nodes of the tree
    nodes: Vec<TreeNode<T>>,
    /// Block to wrap the tree
    block: Option<Block<'a>>,
    /// Render callback for custom node display
    render_fn: NodeRenderFn<'a, T>,
    /// Default expand icon
    expand_icon: &'a str,
    /// Default collapse icon
    collapse_icon: &'a str,
    /// Style for selected row background (full-width highlight)
    highlight_style: Option<Style>,
}

impl<'a, T> TreeView<'a, T> {
    /// Create a new tree view with nodes
    pub fn new(nodes: Vec<TreeNode<T>>) -> Self {
        Self {
            nodes,
            block: None,
            render_fn: Box::new(|_data, _state| Line::from("Node")),
            expand_icon: "▶",
            collapse_icon: "▼",
            highlight_style: None,
        }
    }

    /// Set the block to wrap the tree
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Set custom expand/collapse icons
    pub fn icons(mut self, expand: &'a str, collapse: &'a str) -> Self {
        self.expand_icon = expand;
        self.collapse_icon = collapse;
        self
    }

    /// Set the render function for nodes
    pub fn render_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&T, &NodeState) -> Line<'a> + 'a,
    {
        self.render_fn = Box::new(f);
        self
    }

    /// Set the highlight style for selected rows (full-width background)
    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = Some(style);
        self
    }

    /// Flatten the tree into a list of visible items
    fn flatten_tree(&self, state: &TreeViewState) -> Vec<(Line<'a>, Vec<usize>)> {
        let mut items = Vec::new();

        /// Context for tree traversal to reduce function parameters
        struct TraverseContext<'a, 'b, T> {
            state: &'b TreeViewState,
            render_fn: &'b dyn Fn(&T, &NodeState) -> Line<'a>,
            expand_icon: &'b str,
            collapse_icon: &'b str,
        }

        fn traverse<'a, T>(
            nodes: &[TreeNode<T>],
            current_path: Vec<usize>,
            level: usize,
            ctx: &TraverseContext<'a, '_, T>,
            items: &mut Vec<(Line<'a>, Vec<usize>)>,
        ) {
            for (idx, node) in nodes.iter().enumerate() {
                let mut path = current_path.clone();
                path.push(idx);

                let is_expanded = ctx.state.is_expanded(&path);
                let is_selected = ctx.state.selected_path.as_ref() == Some(&path);

                let node_state = NodeState {
                    is_selected,
                    is_expanded,
                    level,
                    has_children: !node.children.is_empty(),
                    path: path.clone(),
                };

                // Render the node with indent and expand/collapse icon
                let indent = "  ".repeat(level);
                let expansion_icon = if node.expandable {
                    if is_expanded {
                        ctx.collapse_icon
                    } else {
                        ctx.expand_icon
                    }
                } else {
                    " "
                };

                // Get the custom rendered line
                let custom_line = (ctx.render_fn)(&node.data, &node_state);

                // Prepend indent and expansion icon
                let mut spans = vec![
                    Span::raw(indent),
                    Span::styled(
                        format!("{} ", expansion_icon),
                        Style::default().fg(Color::DarkGray),
                    ),
                ];
                spans.extend(custom_line.spans);

                items.push((Line::from(spans), path.clone()));

                // Recursively render children if expanded
                if is_expanded && !node.children.is_empty() {
                    traverse(&node.children, path, level + 1, ctx, items);
                }
            }
        }

        let ctx = TraverseContext {
            state,
            render_fn: &self.render_fn,
            expand_icon: self.expand_icon,
            collapse_icon: self.collapse_icon,
        };

        traverse(&self.nodes, Vec::new(), 0, &ctx, &mut items);

        items
    }

    /// Get the node at a specific row (considering scroll offset)
    pub fn node_at_row(&self, state: &TreeViewState, row: usize) -> Option<Vec<usize>> {
        let items = self.flatten_tree(state);
        items.get(row + state.offset).map(|(_, path)| path.clone())
    }

    /// Get total visible item count
    pub fn visible_item_count(&self, state: &TreeViewState) -> usize {
        self.flatten_tree(state).len()
    }
}

impl<'a, T> StatefulWidget for TreeView<'a, T> {
    type State = TreeViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = match self.block {
            Some(ref b) => {
                let inner = b.inner(area);
                b.clone().render(area, buf);
                inner
            }
            None => area,
        };

        if area.height == 0 {
            return;
        }

        let items = self.flatten_tree(state);
        let visible_height = area.height as usize;

        // Adjust scroll offset to ensure selected item is visible
        if let Some(ref selected) = state.selected_path {
            if let Some(selected_idx) = items.iter().position(|(_, path)| path == selected) {
                if selected_idx < state.offset {
                    state.offset = selected_idx;
                } else if selected_idx >= state.offset + visible_height {
                    state.offset = selected_idx.saturating_sub(visible_height - 1);
                }
            }
        }

        // Render visible items
        for (i, (line, path)) in items
            .iter()
            .skip(state.offset)
            .take(visible_height)
            .enumerate()
        {
            let y = area.y + i as u16;

            // Fill background for selected row (full-width highlight like Yazi)
            let is_selected = state.selected_path.as_ref() == Some(path);
            if is_selected && self.highlight_style.is_some() {
                let style = self.highlight_style.unwrap();
                for x in area.x..(area.x + area.width) {
                    buf[(x, y)].set_style(style);
                }
            }

            buf.set_line(area.x, y, line, area.width);
        }
    }
}

// Also implement Widget for &TreeView with immutable state
impl<'a, T> Widget for &TreeView<'a, T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let state = TreeViewState::default();

        let area = match &self.block {
            Some(ref b) => {
                let inner = b.inner(area);
                b.clone().render(area, buf);
                inner
            }
            None => area,
        };

        if area.height == 0 {
            return;
        }

        let items = self.flatten_tree(&state);
        let visible_height = area.height as usize;

        // Render visible items
        for (i, (line, _)) in items
            .iter()
            .skip(state.offset)
            .take(visible_height)
            .enumerate()
        {
            let y = area.y + i as u16;
            buf.set_line(area.x, y, line, area.width);
        }
    }
}

/// Get all visible paths (flattened tree with expansion state)
pub fn get_visible_paths<T>(nodes: &[TreeNode<T>], state: &TreeViewState) -> Vec<Vec<usize>> {
    let mut paths = Vec::new();

    fn traverse<T>(
        nodes: &[TreeNode<T>],
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

    traverse(nodes, Vec::new(), state, &mut paths);
    paths
}

/// Tree navigator with configurable keybindings
#[derive(Clone)]
pub struct TreeNavigator {
    pub keybindings: TreeKeyBindings,
}

impl Default for TreeNavigator {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeNavigator {
    /// Create a new tree navigator with default keybindings
    pub fn new() -> Self {
        Self {
            keybindings: TreeKeyBindings::default(),
        }
    }

    /// Create a tree navigator with custom keybindings
    pub fn with_keybindings(keybindings: TreeKeyBindings) -> Self {
        Self { keybindings }
    }

    /// Get hotkey items for display in HotkeyFooter
    /// Returns a vec of (key_display, description) pairs
    pub fn get_hotkey_items(&self) -> Vec<(String, &'static str)> {
        let mut items = Vec::new();

        // Helper to format multiple keys
        let format_keys = |keys: &[KeyCode]| -> String {
            keys.iter()
                .map(|k| match k {
                    KeyCode::Char(c) => c.to_string(),
                    KeyCode::Up => "↑".to_string(),
                    KeyCode::Down => "↓".to_string(),
                    KeyCode::Left => "←".to_string(),
                    KeyCode::Right => "→".to_string(),
                    KeyCode::Enter => "Enter".to_string(),
                    _ => format!("{:?}", k),
                })
                .collect::<Vec<_>>()
                .join("/")
        };

        items.push((format_keys(&self.keybindings.next), "Next"));
        items.push((format_keys(&self.keybindings.previous), "Previous"));
        items.push((format_keys(&self.keybindings.expand), "Expand"));
        items.push((format_keys(&self.keybindings.collapse), "Collapse"));
        items.push((format_keys(&self.keybindings.toggle), "Toggle"));
        items.push((format_keys(&self.keybindings.goto_top), "Top"));
        items.push((format_keys(&self.keybindings.goto_bottom), "Bottom"));

        items
    }

    /// Handle a key event and update tree state
    /// Returns true if the key was handled
    pub fn handle_key<T>(
        &self,
        key: KeyEvent,
        nodes: &[TreeNode<T>],
        state: &mut TreeViewState,
    ) -> bool {
        // Only handle key press events, not release
        if key.kind != crossterm::event::KeyEventKind::Press {
            return false;
        }

        let code = key.code;

        if self.keybindings.next.contains(&code) {
            self.select_next(nodes, state);
            true
        } else if self.keybindings.previous.contains(&code) {
            self.select_previous(nodes, state);
            true
        } else if self.keybindings.expand.contains(&code) {
            self.expand_selected(nodes, state);
            true
        } else if self.keybindings.collapse.contains(&code) {
            self.collapse_selected(nodes, state);
            true
        } else if self.keybindings.toggle.contains(&code) {
            self.toggle_selected(nodes, state);
            true
        } else if self.keybindings.goto_top.contains(&code) {
            self.goto_top(nodes, state);
            true
        } else if self.keybindings.goto_bottom.contains(&code) {
            self.goto_bottom(nodes, state);
            true
        } else {
            false
        }
    }

    /// Select next visible item
    pub fn select_next<T>(&self, nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        let visible_paths = get_visible_paths(nodes, state);
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

    /// Select previous visible item
    pub fn select_previous<T>(&self, nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        let visible_paths = get_visible_paths(nodes, state);
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

    /// Go to first visible item
    pub fn goto_top<T>(&self, nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        let visible_paths = get_visible_paths(nodes, state);
        if !visible_paths.is_empty() {
            state.select(visible_paths[0].clone());
        }
    }

    /// Go to last visible item
    pub fn goto_bottom<T>(&self, nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        let visible_paths = get_visible_paths(nodes, state);
        if !visible_paths.is_empty() {
            state.select(visible_paths[visible_paths.len() - 1].clone());
        }
    }

    /// Expand selected node
    pub fn expand_selected<T>(&self, nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        if let Some(path) = state.selected_path.clone() {
            // Check if node has children
            if let Some(node) = self.get_node_at_path(nodes, &path) {
                if !node.children.is_empty() {
                    state.expand(path);
                }
            }
        }
    }

    /// Collapse selected node or move to parent
    pub fn collapse_selected<T>(&self, _nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        if let Some(path) = state.selected_path.clone() {
            if state.is_expanded(&path) {
                // Collapse current
                state.collapse(path);
            } else if path.len() > 1 {
                // Move to parent
                let parent = path[..path.len() - 1].to_vec();
                state.select(parent);
            }
        }
    }

    /// Toggle expansion of selected node (expand if collapsed, collapse if expanded)
    pub fn toggle_selected<T>(&self, nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        if let Some(path) = state.selected_path.clone() {
            // Check if node has children
            if let Some(node) = self.get_node_at_path(nodes, &path) {
                if !node.children.is_empty() {
                    state.toggle_expansion(path);
                }
            }
        }
    }

    /// Helper to get node at a specific path
    fn get_node_at_path<'a, T>(
        &self,
        nodes: &'a [TreeNode<T>],
        path: &[usize],
    ) -> Option<&'a TreeNode<T>> {
        if path.is_empty() {
            return None;
        }

        let mut current_nodes = nodes;
        let mut node = None;

        for &idx in path {
            node = current_nodes.get(idx);
            if let Some(n) = node {
                current_nodes = &n.children;
            } else {
                return None;
            }
        }

        node
    }
}

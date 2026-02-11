//! TreeView::flatten_tree method.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::primitives::tree_view::node_state::NodeState;
use crate::primitives::tree_view::tree_node::TreeNode;
use crate::primitives::tree_view::tree_view_state::TreeViewState;
use crate::primitives::tree_view::widget::TreeView;

impl<'a, T> TreeView<'a, T> {
    /// Flattens the tree into a list of visible items.
    ///
    /// # Arguments
    ///
    /// * `state` - The tree view state (for expansion info).
    ///
    /// # Returns
    ///
    /// A vector of tuples containing the rendered line and the path to each visible node.
    pub(crate) fn flatten_tree(&self, state: &TreeViewState) -> Vec<(Line<'a>, Vec<usize>)> {
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
}

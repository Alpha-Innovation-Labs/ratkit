//! TreeViewRef::render_fn method.

use ratatui::text::Line;

use crate::tree_view::node_state::NodeState;
use crate::tree_view::tree_view_ref::TreeViewRef;

impl<'a, 'b, T> TreeViewRef<'a, 'b, T> {
    /// Sets the render function for nodes.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes node data and state, returns a Line.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::text::Line;
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeViewRef};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeViewRef::new(&nodes)
    ///     .render_fn(|data, state| {
    ///         if state.is_selected {
    ///             Line::from(format!("> {}", data))
    ///         } else {
    ///             Line::from(*data)
    ///         }
    ///     });
    /// ```
    pub fn render_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&T, &NodeState) -> Line<'a> + 'a,
    {
        self.render_fn = Box::new(f);
        self
    }
}

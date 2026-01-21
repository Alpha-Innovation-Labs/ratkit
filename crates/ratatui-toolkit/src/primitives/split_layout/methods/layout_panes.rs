use ratatui::layout::Rect;

use crate::primitives::split_layout::enums::layout_node::LayoutNode;
use crate::primitives::split_layout::pane_layout::PaneLayout;
use crate::primitives::split_layout::SplitAxis;
use crate::primitives::split_layout::SplitLayout;

impl SplitLayout {
    /// Calculates pane rectangles for the current split tree.
    ///
    /// # Arguments
    /// - `area`: The available rectangle to divide among panes.
    ///
    /// # Returns
    /// A vector of `PaneLayout` values for each leaf pane.
    ///
    /// # Errors
    /// - None.
    ///
    /// # Panics
    /// - Does not panic.
    ///
    /// # Safety
    /// - No safety requirements.
    ///
    /// # Performance
    /// - O(n) where n is the number of nodes.
    ///
    /// # Example
    /// ```rust
    /// use ratatui::layout::Rect;
    /// use ratatui_toolkit::primitives::split_layout::SplitLayout;
    ///
    /// let layout = SplitLayout::new(0);
    /// let panes = layout.layout_panes(Rect::new(0, 0, 120, 40));
    /// ```
    pub fn layout_panes(&self, area: Rect) -> Vec<PaneLayout> {
        let mut layouts = Vec::new();
        let mut stack = vec![(self.root_index, area)];

        while let Some((node_index, node_area)) = stack.pop() {
            let Some(node) = self.nodes.get(node_index) else {
                continue;
            };

            match node {
                LayoutNode::Pane { id } => {
                    layouts.push(PaneLayout::new(*id, node_area));
                }
                LayoutNode::Split {
                    axis,
                    ratio,
                    first,
                    second,
                } => match axis {
                    SplitAxis::Vertical => {
                        let first_width = ((node_area.width as u32 * *ratio as u32) / 100) as u16;
                        let second_width = node_area.width.saturating_sub(first_width);
                        let first_area = Rect {
                            x: node_area.x,
                            y: node_area.y,
                            width: first_width,
                            height: node_area.height,
                        };
                        let second_area = Rect {
                            x: node_area.x.saturating_add(first_width),
                            y: node_area.y,
                            width: second_width,
                            height: node_area.height,
                        };
                        stack.push((*second, second_area));
                        stack.push((*first, first_area));
                    }
                    SplitAxis::Horizontal => {
                        let first_height = ((node_area.height as u32 * *ratio as u32) / 100) as u16;
                        let second_height = node_area.height.saturating_sub(first_height);
                        let first_area = Rect {
                            x: node_area.x,
                            y: node_area.y,
                            width: node_area.width,
                            height: first_height,
                        };
                        let second_area = Rect {
                            x: node_area.x,
                            y: node_area.y.saturating_add(first_height),
                            width: node_area.width,
                            height: second_height,
                        };
                        stack.push((*second, second_area));
                        stack.push((*first, first_area));
                    }
                },
            }
        }

        layouts
    }
}

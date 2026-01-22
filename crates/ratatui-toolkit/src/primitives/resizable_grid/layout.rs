//! Layout calculations for resizable grids.
//!
//! Provides methods for calculating pane and divider layouts from a resizable grid.

use crate::primitives::resizable_grid::types::{
    LayoutNode, PaneId, ResizableGrid, SplitAxis, SplitDividerLayout,
};
use ratatui::layout::Rect;

/// A computed rectangle for a pane.
///
/// Combines a pane identifier with the rectangle allocated by a split layout.
///
/// # Example
/// ```rust
/// use ratatui::layout::Rect;
/// use ratatui_toolkit::primitives::resizable_grid::ResizableGrid;
///
/// let grid = ResizableGrid::new(1);
/// let panes = grid.layout_panes(Rect::new(0, 0, 10, 5));
/// let _ = panes[0].pane_id();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaneLayout {
    pane_id: PaneId,
    area: Rect,
}

impl PaneLayout {
    /// Creates a new pane layout from an ID and rectangle.
    ///
    /// # Arguments
    /// - `pane_id`: Identifier for the pane.
    /// - `area`: The rectangle allocated to the pane.
    ///
    /// # Returns
    /// A `PaneLayout` containing the provided values.
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
    /// - O(1).
    ///
    /// # Example
    /// ```rust
    /// use ratatui::layout::Rect;
    /// use ratatui_toolkit::primitives::resizable_grid::ResizableGrid;
    ///
    /// let grid = ResizableGrid::new(0);
    /// let panes = grid.layout_panes(Rect::new(0, 0, 10, 5));
    /// let _ = panes.first();
    /// ```
    pub(crate) fn new(pane_id: PaneId, area: Rect) -> Self {
        Self { pane_id, area }
    }

    /// Returns the rectangle allocated to this pane.
    ///
    /// # Arguments
    /// - None.
    ///
    /// # Returns
    /// The allocated `Rect`.
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
    /// - O(1).
    ///
    /// # Example
    /// ```rust
    /// use ratatui::layout::Rect;
    /// use ratatui_toolkit::primitives::resizable_grid::ResizableGrid;
    ///
    /// let grid = ResizableGrid::new(0);
    /// let panes = grid.layout_panes(Rect::new(0, 0, 10, 5));
    /// let _ = panes[0].area();
    /// ```
    pub fn area(&self) -> Rect {
        self.area
    }

    /// Returns the identifier for this pane.
    ///
    /// # Arguments
    /// - None.
    ///
    /// # Returns
    /// The pane identifier.
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
    /// - O(1).
    ///
    /// # Example
    /// ```rust
    /// use ratatui::layout::Rect;
    /// use ratatui_toolkit::primitives::resizable_grid::ResizableGrid;
    ///
    /// let grid = ResizableGrid::new(0);
    /// let panes = grid.layout_panes(Rect::new(0, 0, 10, 5));
    /// let _ = panes[0].pane_id();
    /// ```
    pub fn pane_id(&self) -> PaneId {
        self.pane_id
    }
}

impl ResizableGrid {
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
    /// use ratatui_toolkit::primitives::resizable_grid::ResizableGrid;
    ///
    /// let grid = ResizableGrid::new(0);
    /// let panes = grid.layout_panes(Rect::new(0, 0, 120, 40));
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

    /// Calculates divider metadata for the current split tree.
    ///
    /// # Arguments
    /// - `area`: The available rectangle to divide among panes.
    ///
    /// # Returns
    /// A vector of `SplitDividerLayout` values for each split node.
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
    /// - O(n) where n is the number of split nodes.
    ///
    /// # Example
    /// ```rust
    /// use ratatui::layout::Rect;
    /// use ratatui_toolkit::primitives::resizable_grid::ResizableGrid;
    ///
    /// let grid = ResizableGrid::new(0);
    /// let dividers = grid.layout_dividers(Rect::new(0, 0, 120, 40));
    /// ```
    pub fn layout_dividers(&self, area: Rect) -> Vec<SplitDividerLayout> {
        let mut dividers = Vec::new();
        let mut stack = vec![(self.root_index, area)];

        while let Some((node_index, node_area)) = stack.pop() {
            let Some(node) = self.nodes.get(node_index) else {
                continue;
            };

            if let LayoutNode::Split {
                axis,
                ratio,
                first,
                second,
            } = node
            {
                dividers.push(SplitDividerLayout {
                    split_index: node_index,
                    axis: *axis,
                    area: node_area,
                    ratio: *ratio,
                });

                match axis {
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
                }
            }
        }

        dividers
    }
}

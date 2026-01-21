use crate::primitives::split_layout::enums::layout_node::LayoutNode;
use crate::primitives::split_layout::SplitAxis;
use crate::primitives::split_layout::SplitDividerLayout;
use crate::primitives::split_layout::SplitLayout;
use ratatui::layout::Rect;

impl SplitLayout {
    /// Calculates divider metadata for the current split tree.
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

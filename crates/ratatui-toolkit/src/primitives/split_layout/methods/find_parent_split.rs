use crate::primitives::split_layout::enums::layout_node::LayoutNode;
use crate::primitives::split_layout::PaneId;
use crate::primitives::split_layout::SplitLayout;

impl SplitLayout {
    pub(super) fn find_parent_split(&self, pane_id: PaneId) -> Option<(usize, bool)> {
        let mut stack = vec![(self.root_index, None)];

        while let Some((node_index, parent_index)) = stack.pop() {
            let Some(node) = self.nodes.get(node_index) else {
                continue;
            };

            match node {
                LayoutNode::Pane { id } => {
                    if *id == pane_id {
                        let Some(parent_index) = parent_index else {
                            return None;
                        };
                        let Some(LayoutNode::Split { first, .. }) = self.nodes.get(parent_index)
                        else {
                            return None;
                        };
                        let is_first = *first == node_index;
                        return Some((parent_index, is_first));
                    }
                }
                LayoutNode::Split { first, second, .. } => {
                    stack.push((*second, Some(node_index)));
                    stack.push((*first, Some(node_index)));
                }
            }
        }

        None
    }
}

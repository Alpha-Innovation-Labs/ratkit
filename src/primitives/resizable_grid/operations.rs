use crate::primitives::resizable_grid::types::{
    LayoutNode, PaneId, ResizableGrid, SplitAxis, SplitDividerLayout, DEFAULT_SPLIT_PERCENT,
    MAX_SPLIT_PERCENT, MIN_SPLIT_PERCENT,
};
use ratatui::layout::Rect;

impl ResizableGrid {
    pub(super) fn split_pane(&mut self, pane_id: PaneId, axis: SplitAxis) -> Option<PaneId> {
        let pane_index = self.find_pane_node_index(pane_id)?;
        let new_pane_id = self.allocate_pane_id();
        let first_index = self.nodes.len();
        let second_index = self.nodes.len().saturating_add(1);

        self.nodes.push(LayoutNode::Pane { id: pane_id });
        self.nodes.push(LayoutNode::Pane { id: new_pane_id });
        self.nodes[pane_index] = LayoutNode::Split {
            axis,
            ratio: DEFAULT_SPLIT_PERCENT,
            first: first_index,
            second: second_index,
        };

        Some(new_pane_id)
    }

    pub fn split_pane_horizontally(&mut self, pane_id: PaneId) -> Option<PaneId> {
        self.split_pane(pane_id, SplitAxis::Horizontal)
    }

    pub fn split_pane_vertically(&mut self, pane_id: PaneId) -> Option<PaneId> {
        self.split_pane(pane_id, SplitAxis::Vertical)
    }

    pub fn resize_split(&mut self, split_index: usize, percent: u16) -> bool {
        let Some(LayoutNode::Split { ratio, .. }) = self.nodes.get_mut(split_index) else {
            return false;
        };

        *ratio = percent.clamp(MIN_SPLIT_PERCENT, MAX_SPLIT_PERCENT);
        true
    }

    pub fn resize_divider(&mut self, pane_id: PaneId, percent: u16) -> bool {
        let Some((parent_index, is_first)) = self.find_parent_split(pane_id) else {
            return false;
        };
        let Some(LayoutNode::Split { ratio, .. }) = self.nodes.get_mut(parent_index) else {
            return false;
        };

        let new_ratio = if is_first {
            percent
        } else {
            100_u16.saturating_sub(percent)
        };

        *ratio = new_ratio.clamp(MIN_SPLIT_PERCENT, MAX_SPLIT_PERCENT);
        true
    }

    pub(super) fn find_parent_split(&self, pane_id: PaneId) -> Option<(usize, bool)> {
        let mut stack = vec![(self.root_index, None)];

        while let Some((node_index, parent_index)) = stack.pop() {
            let Some(node) = self.nodes.get(node_index) else {
                continue;
            };

            match node {
                LayoutNode::Pane { id } => {
                    if *id == pane_id {
                        let parent_index = parent_index?;
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

    pub(super) fn find_pane_node_index(&self, pane_id: PaneId) -> Option<usize> {
        let mut stack = vec![self.root_index];

        while let Some(node_index) = stack.pop() {
            let Some(node) = self.nodes.get(node_index) else {
                continue;
            };

            match node {
                LayoutNode::Pane { id } => {
                    if *id == pane_id {
                        return Some(node_index);
                    }
                }
                LayoutNode::Split { first, second, .. } => {
                    stack.push(*second);
                    stack.push(*first);
                }
            }
        }

        None
    }

    pub fn move_pane(&mut self, pane_id: PaneId, target_pane_id: PaneId) -> bool {
        let Some(source_index) = self.find_pane_node_index(pane_id) else {
            return false;
        };
        let Some(target_index) = self.find_pane_node_index(target_pane_id) else {
            return false;
        };

        if source_index == target_index {
            return true;
        }

        let source_id = match self.nodes.get(source_index) {
            Some(LayoutNode::Pane { id }) => *id,
            _ => return false,
        };
        let target_id = match self.nodes.get(target_index) {
            Some(LayoutNode::Pane { id }) => *id,
            _ => return false,
        };

        self.nodes[source_index] = LayoutNode::Pane { id: target_id };
        self.nodes[target_index] = LayoutNode::Pane { id: source_id };

        true
    }

    pub fn remove_pane(&mut self, pane_id: PaneId) -> bool {
        let Some((parent_index, is_first)) = self.find_parent_split(pane_id) else {
            return false;
        };
        let Some(LayoutNode::Split { first, second, .. }) = self.nodes.get(parent_index) else {
            return false;
        };
        let sibling_index = if is_first { *second } else { *first };
        let Some(sibling_node) = self.nodes.get(sibling_index).cloned() else {
            return false;
        };

        self.nodes[parent_index] = sibling_node;
        true
    }

    pub(super) fn allocate_pane_id(&mut self) -> PaneId {
        let pane_id = self.next_pane_id;
        self.next_pane_id = self.next_pane_id.saturating_add(1);
        pane_id
    }

    pub fn get_split_ratio(&self, split_index: usize) -> Option<u16> {
        let Some(LayoutNode::Split { ratio, .. }) = self.nodes.get(split_index) else {
            return None;
        };
        Some(*ratio)
    }

    pub fn split_percent(&self) -> u16 {
        self.get_split_ratio(0).unwrap_or(DEFAULT_SPLIT_PERCENT)
    }

    pub fn min_percent(&self) -> u16 {
        MIN_SPLIT_PERCENT
    }

    pub fn max_percent(&self) -> u16 {
        MAX_SPLIT_PERCENT
    }

    pub fn set_split_percent(&mut self, percent: u16) {
        let _ = self.resize_split(0, percent);
    }

    pub fn is_hovering(&self) -> bool {
        self.hovered_split.is_some()
    }

    pub fn is_dragging(&self) -> bool {
        self.dragging_split.is_some()
    }

    pub fn start_drag(&mut self) {
        self.dragging_split = self.hovered_split;
    }

    pub fn stop_drag(&mut self) {
        self.dragging_split = None;
    }

    pub fn update_divider_position(&mut self, _area: Rect) {}

    pub fn is_on_divider(&self, mouse_column: u16, mouse_row: u16, area: Rect) -> bool {
        self.find_divider_at(mouse_column, mouse_row, area)
            .is_some()
    }

    pub fn find_divider_at(&self, column: u16, row: u16, area: Rect) -> Option<usize> {
        let layouts = self.layout_dividers_internal(area);
        let threshold = self.hit_threshold;

        for divider in &layouts {
            let rect = divider.area();
            match divider.axis() {
                SplitAxis::Vertical => {
                    let divider_x = rect.x.saturating_add(
                        ((rect.width as u32 * divider.ratio() as u32) / 100) as u16,
                    );
                    let distance = divider_x.abs_diff(column);
                    if distance <= threshold
                        && column <= divider_x.saturating_add(threshold)
                        && row >= rect.y
                        && row <= rect.y.saturating_add(rect.height)
                    {
                        return Some(divider.split_index());
                    }
                }
                SplitAxis::Horizontal => {
                    let divider_y = rect.y.saturating_add(
                        ((rect.height as u32 * divider.ratio() as u32) / 100) as u16,
                    );
                    let distance = divider_y.abs_diff(row);
                    if distance <= threshold
                        && row <= divider_y.saturating_add(threshold)
                        && column >= rect.x
                        && column <= rect.x.saturating_add(rect.width)
                    {
                        return Some(divider.split_index());
                    }
                }
            }
        }
        None
    }

    pub fn update_from_mouse(&mut self, mouse_column: u16, mouse_row: u16, area: Rect) {
        if self.dragging_split.is_none() {
            return;
        }

        let Some(split_index) = self.dragging_split else {
            return;
        };

        let layouts = self.layout_dividers_internal(area);
        let divider_layout = layouts
            .iter()
            .find(|divider| divider.split_index() == split_index);

        if let Some(divider) = divider_layout {
            let rect = divider.area();
            match divider.axis() {
                SplitAxis::Vertical => {
                    let content_width = rect.width;
                    if content_width > 0 {
                        let relative_x = mouse_column.saturating_sub(rect.x);
                        let percent = ((relative_x as u32 * 100) / content_width as u32) as u16;
                        let _ = self.resize_split(split_index, percent);
                    }
                }
                SplitAxis::Horizontal => {
                    let content_height = rect.height;
                    if content_height > 0 {
                        let relative_y = mouse_row.saturating_sub(rect.y);
                        let percent = ((relative_y as u32 * 100) / content_height as u32) as u16;
                        let _ = self.resize_split(split_index, percent);
                    }
                }
            }
        }
    }

    fn layout_dividers_internal(&self, area: Rect) -> Vec<SplitDividerLayout> {
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

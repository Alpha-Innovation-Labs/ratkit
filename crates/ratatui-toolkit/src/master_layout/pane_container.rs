//! Pane container with selection and focus management

use super::{InteractionMode, Pane, PaneId, PaneLayout};
use crate::resizable_split::{ResizableSplit, SplitDirection};
use ratatui::layout::Rect;

/// Container for managing panes within a tab
pub struct PaneContainer {
    pub(crate) panes: Vec<Pane>,
    layout: PaneLayout,
    /// Resizable splits for horizontal/vertical layouts (one per divider)
    /// For 2 panes there's 1 divider, for 3 panes there's 2 dividers, etc.
    resizable_splits: Vec<ResizableSplit>,
    /// The area where panes are rendered (updated each frame)
    container_area: Rect,
}

impl PaneContainer {
    /// Create a new pane container with the given layout
    pub fn new(layout: PaneLayout) -> Self {
        Self {
            panes: Vec::new(),
            layout,
            resizable_splits: Vec::new(),
            container_area: Rect::default(),
        }
    }

    /// Initialize resizable splits based on current layout and pane count
    fn init_resizable_splits(&mut self) {
        self.resizable_splits.clear();

        match &self.layout {
            PaneLayout::Horizontal(percentages) => {
                // For N panes, we need N-1 dividers
                let divider_count = self.panes.len().saturating_sub(1);

                for i in 0..divider_count {
                    // Calculate the cumulative percentage up to this divider
                    let split_percent = if i < percentages.len() {
                        // Sum up percentages up to and including this pane
                        percentages.iter().take(i + 1).copied().sum::<u16>()
                    } else {
                        // Equal distribution
                        (((i + 1) * 100) / self.panes.len()) as u16
                    };

                    self.resizable_splits
                        .push(ResizableSplit::new_with_direction(
                            split_percent,
                            SplitDirection::Vertical,
                        ));
                }
            }
            PaneLayout::Vertical(percentages) => {
                let divider_count = self.panes.len().saturating_sub(1);

                for i in 0..divider_count {
                    let split_percent = if i < percentages.len() {
                        percentages.iter().take(i + 1).copied().sum::<u16>()
                    } else {
                        (((i + 1) * 100) / self.panes.len()) as u16
                    };

                    self.resizable_splits
                        .push(ResizableSplit::new_with_direction(
                            split_percent,
                            SplitDirection::Horizontal,
                        ));
                }
            }
            _ => {
                // Grid and Custom layouts don't support resizing (for now)
            }
        }
    }

    /// Add a pane to the container
    pub fn add_pane(&mut self, pane: Pane) {
        self.panes.push(pane);
        // Reinitialize splits when panes change
        self.init_resizable_splits();
    }

    /// Get number of panes
    pub fn pane_count(&self) -> usize {
        self.panes.len()
    }

    /// Get pane by ID
    pub fn get_pane(&self, id: PaneId) -> Option<&Pane> {
        self.panes.iter().find(|p| p.id() == id)
    }

    /// Get mutable pane by ID
    pub fn get_pane_mut(&mut self, id: PaneId) -> Option<&mut Pane> {
        self.panes.iter_mut().find(|p| p.id() == id)
    }

    /// Get pane by index
    pub fn get_pane_by_index(&self, index: usize) -> Option<&Pane> {
        self.panes.get(index)
    }

    /// Get mutable pane by index
    pub fn get_pane_by_index_mut(&mut self, index: usize) -> Option<&mut Pane> {
        self.panes.get_mut(index)
    }

    /// Find pane at the given coordinates
    pub fn find_pane_at(&self, x: u16, y: u16) -> Option<PaneId> {
        self.panes
            .iter()
            .find(|pane| pane.contains_point(x, y))
            .map(|pane| pane.id())
    }

    /// Calculate and update pane areas based on layout
    pub fn update_layout(&mut self, available_area: Rect) {
        self.container_area = available_area;

        // Update divider positions
        for split in &mut self.resizable_splits {
            split.update_divider_position(available_area);
        }

        let areas = if self.resizable_splits.is_empty() {
            // No resizable splits - use standard layout calculation
            self.layout
                .calculate_areas(available_area, self.panes.len())
        } else {
            // Calculate areas from resizable splits
            self.calculate_areas_from_splits(available_area)
        };

        for (pane, area) in self.panes.iter_mut().zip(areas.iter()) {
            pane.set_area(*area);
        }
    }

    /// Calculate pane areas from resizable splits
    fn calculate_areas_from_splits(&self, available_area: Rect) -> Vec<Rect> {
        if self.panes.is_empty() {
            return Vec::new();
        }

        let mut areas = Vec::new();

        match &self.layout {
            PaneLayout::Horizontal(_) => {
                // Calculate areas for horizontal layout with vertical dividers
                let mut last_x = available_area.x;

                for (i, _pane_idx) in (0..self.panes.len()).enumerate() {
                    let next_x = if i < self.resizable_splits.len() {
                        // Use the split's divider position
                        self.resizable_splits[i].divider_pos
                    } else {
                        // Last pane - extend to the end
                        available_area.x + available_area.width
                    };

                    let width = next_x.saturating_sub(last_x);
                    areas.push(Rect::new(
                        last_x,
                        available_area.y,
                        width,
                        available_area.height,
                    ));

                    last_x = next_x;
                }
            }
            PaneLayout::Vertical(_) => {
                // Calculate areas for vertical layout with horizontal dividers
                let mut last_y = available_area.y;

                for (i, _pane_idx) in (0..self.panes.len()).enumerate() {
                    let next_y = if i < self.resizable_splits.len() {
                        // Use the split's divider position
                        self.resizable_splits[i].divider_pos
                    } else {
                        // Last pane - extend to the end
                        available_area.y + available_area.height
                    };

                    let height = next_y.saturating_sub(last_y);
                    areas.push(Rect::new(
                        available_area.x,
                        last_y,
                        available_area.width,
                        height,
                    ));

                    last_y = next_y;
                }
            }
            _ => {
                // Fallback to standard calculation
                return self
                    .layout
                    .calculate_areas(available_area, self.panes.len());
            }
        }

        areas
    }

    /// Select next pane (cycle forward)
    pub fn select_next(&self, current: Option<PaneId>) -> Option<PaneId> {
        if self.panes.is_empty() {
            return None;
        }

        let focusable_panes: Vec<_> = self.panes.iter().filter(|p| p.is_focusable()).collect();

        if focusable_panes.is_empty() {
            return None;
        }

        match current {
            None => Some(focusable_panes[0].id()),
            Some(current_id) => {
                let current_idx = focusable_panes.iter().position(|p| p.id() == current_id);

                match current_idx {
                    Some(idx) => {
                        let next_idx = (idx + 1) % focusable_panes.len();
                        Some(focusable_panes[next_idx].id())
                    }
                    None => Some(focusable_panes[0].id()),
                }
            }
        }
    }

    /// Select previous pane (cycle backward)
    pub fn select_prev(&self, current: Option<PaneId>) -> Option<PaneId> {
        if self.panes.is_empty() {
            return None;
        }

        let focusable_panes: Vec<_> = self.panes.iter().filter(|p| p.is_focusable()).collect();

        if focusable_panes.is_empty() {
            return None;
        }

        match current {
            None => Some(focusable_panes[focusable_panes.len() - 1].id()),
            Some(current_id) => {
                let current_idx = focusable_panes.iter().position(|p| p.id() == current_id);

                match current_idx {
                    Some(idx) => {
                        let prev_idx = if idx == 0 {
                            focusable_panes.len() - 1
                        } else {
                            idx - 1
                        };
                        Some(focusable_panes[prev_idx].id())
                    }
                    None => Some(focusable_panes[0].id()),
                }
            }
        }
    }

    /// Select pane to the left (h key)
    pub fn select_left(&self, current: PaneId) -> Option<PaneId> {
        self.select_directional(current, Direction::Left)
    }

    /// Select pane to the right (l key)
    pub fn select_right(&self, current: PaneId) -> Option<PaneId> {
        self.select_directional(current, Direction::Right)
    }

    /// Select pane above (k key)
    pub fn select_up(&self, current: PaneId) -> Option<PaneId> {
        self.select_directional(current, Direction::Up)
    }

    /// Select pane below (j key)
    pub fn select_down(&self, current: PaneId) -> Option<PaneId> {
        self.select_directional(current, Direction::Down)
    }

    /// Spatial navigation in a direction
    fn select_directional(&self, current: PaneId, direction: Direction) -> Option<PaneId> {
        let current_pane = self.get_pane(current)?;
        let current_area = current_pane.area();
        let current_center = center_point(current_area);

        // Find the closest focusable pane in the given direction
        let mut best_pane: Option<PaneId> = None;
        let mut best_distance: f64 = f64::MAX;

        for pane in &self.panes {
            if !pane.is_focusable() || pane.id() == current {
                continue;
            }

            let area = pane.area();
            let center = center_point(area);

            // Check if pane is in the correct direction
            let in_direction = match direction {
                Direction::Left => center.0 < current_center.0,
                Direction::Right => center.0 > current_center.0,
                Direction::Up => center.1 < current_center.1,
                Direction::Down => center.1 > current_center.1,
            };

            if !in_direction {
                continue;
            }

            // Calculate distance
            let distance = distance_between(current_center, center);

            if distance < best_distance {
                best_distance = distance;
                best_pane = Some(pane.id());
            }
        }

        best_pane
    }

    /// Render all panes
    pub fn render(&mut self, frame: &mut ratatui::Frame, mode: &InteractionMode) {
        let selected_id = mode.selected_pane();
        let focused_id = mode.focused_pane();

        for pane in &mut self.panes {
            let is_selected = selected_id == Some(pane.id());
            let is_focused = focused_id == Some(pane.id());

            // Notify pane content about focus state
            pane.set_focused(is_focused);

            pane.render(frame, is_selected, is_focused);
        }
    }

    /// Check if mouse is on any divider and return the split index
    pub fn find_divider_at(&self, mouse_x: u16, mouse_y: u16) -> Option<usize> {
        for (i, split) in self.resizable_splits.iter().enumerate() {
            if split.is_on_divider(mouse_x, mouse_y, self.container_area) {
                return Some(i);
            }
        }
        None
    }

    /// Start dragging a divider
    pub fn start_drag(&mut self, divider_index: usize) {
        if let Some(split) = self.resizable_splits.get_mut(divider_index) {
            split.start_drag();
        }
    }

    /// Update drag position
    pub fn update_drag(&mut self, mouse_x: u16, mouse_y: u16) {
        for split in &mut self.resizable_splits {
            if split.is_dragging {
                split.update_from_mouse(mouse_x, mouse_y, self.container_area);
            }
        }
    }

    /// Stop dragging all dividers
    pub fn stop_drag(&mut self) {
        for split in &mut self.resizable_splits {
            split.stop_drag();
        }
    }

    /// Check if any divider is currently being dragged
    pub fn is_dragging(&self) -> bool {
        self.resizable_splits.iter().any(|s| s.is_dragging)
    }

    /// Update hover state for dividers
    pub fn update_hover(&mut self, mouse_x: u16, mouse_y: u16) {
        for split in &mut self.resizable_splits {
            split.is_hovering = split.is_on_divider(mouse_x, mouse_y, self.container_area);
        }
    }

    /// Clear hover state for all dividers
    pub fn clear_hover(&mut self) {
        for split in &mut self.resizable_splits {
            split.is_hovering = false;
        }
    }
}

impl Default for PaneContainer {
    fn default() -> Self {
        Self::new(PaneLayout::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// Get center point of a rectangle
fn center_point(rect: Rect) -> (u16, u16) {
    (rect.x + rect.width / 2, rect.y + rect.height / 2)
}

/// Calculate Euclidean distance between two points
fn distance_between(p1: (u16, u16), p2: (u16, u16)) -> f64 {
    let dx = (p2.0 as f64) - (p1.0 as f64);
    let dy = (p2.1 as f64) - (p1.1 as f64);
    (dx * dx + dy * dy).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEvent, MouseEvent};
    use ratatui::{buffer::Buffer, widgets::Widget};

    // Mock PaneContent for testing
    struct MockContent {
        title: String,
        focusable: bool,
    }

    impl MockContent {
        fn new(title: &str) -> Self {
            Self {
                title: title.to_string(),
                focusable: true,
            }
        }

        fn non_focusable(title: &str) -> Self {
            Self {
                title: title.to_string(),
                focusable: false,
            }
        }
    }

    impl Widget for MockContent {
        fn render(self, _area: Rect, _buf: &mut Buffer) {}
    }

    impl super::super::pane::PaneContent for MockContent {
        fn handle_key(&mut self, _key: KeyEvent) -> bool {
            true
        }
        fn handle_mouse(&mut self, _mouse: MouseEvent) -> bool {
            true
        }
        fn title(&self) -> String {
            self.title.clone()
        }
        fn render_content(&mut self, _area: Rect, _frame: &mut ratatui::Frame) {
            // Mock implementation - do nothing
        }
        fn is_focusable(&self) -> bool {
            self.focusable
        }
    }

    #[test]
    fn test_pane_container_creation() {
        let container = PaneContainer::new(PaneLayout::default());
        assert_eq!(container.pane_count(), 0);
    }

    #[test]
    fn test_add_pane() {
        let mut container = PaneContainer::default();
        let pane = Pane::new(PaneId::new("test"), Box::new(MockContent::new("Test")));

        container.add_pane(pane);
        assert_eq!(container.pane_count(), 1);
    }

    #[test]
    fn test_get_pane_by_id() {
        let mut container = PaneContainer::default();
        let pane_id = PaneId::new("test");
        let pane = Pane::new(pane_id, Box::new(MockContent::new("Test")));

        container.add_pane(pane);

        assert!(container.get_pane(pane_id).is_some());
    }

    #[test]
    fn test_select_next_empty() {
        let container = PaneContainer::default();
        assert_eq!(container.select_next(None), None);
    }

    #[test]
    fn test_select_next_single_pane() {
        let mut container = PaneContainer::default();
        let id1 = PaneId::new("pane1");
        container.add_pane(Pane::new(id1, Box::new(MockContent::new("Pane 1"))));

        let next = container.select_next(None);
        assert_eq!(next, Some(id1));

        // Cycling from id1 should return to id1
        let next = container.select_next(Some(id1));
        assert_eq!(next, Some(id1));
    }

    #[test]
    fn test_select_next_multiple_panes() {
        let mut container = PaneContainer::default();
        let id1 = PaneId::new("pane1");
        let id2 = PaneId::new("pane2");
        let id3 = PaneId::new("pane3");

        container.add_pane(Pane::new(id1, Box::new(MockContent::new("Pane 1"))));
        container.add_pane(Pane::new(id2, Box::new(MockContent::new("Pane 2"))));
        container.add_pane(Pane::new(id3, Box::new(MockContent::new("Pane 3"))));

        let next = container.select_next(None);
        assert_eq!(next, Some(id1));

        let next = container.select_next(Some(id1));
        assert_eq!(next, Some(id2));

        let next = container.select_next(Some(id2));
        assert_eq!(next, Some(id3));

        // Cycle back to first
        let next = container.select_next(Some(id3));
        assert_eq!(next, Some(id1));
    }

    #[test]
    fn test_select_prev_multiple_panes() {
        let mut container = PaneContainer::default();
        let id1 = PaneId::new("pane1");
        let id2 = PaneId::new("pane2");
        let id3 = PaneId::new("pane3");

        container.add_pane(Pane::new(id1, Box::new(MockContent::new("Pane 1"))));
        container.add_pane(Pane::new(id2, Box::new(MockContent::new("Pane 2"))));
        container.add_pane(Pane::new(id3, Box::new(MockContent::new("Pane 3"))));

        let prev = container.select_prev(None);
        assert_eq!(prev, Some(id3)); // Start from last

        let prev = container.select_prev(Some(id3));
        assert_eq!(prev, Some(id2));

        let prev = container.select_prev(Some(id2));
        assert_eq!(prev, Some(id1));

        // Cycle back to last
        let prev = container.select_prev(Some(id1));
        assert_eq!(prev, Some(id3));
    }

    #[test]
    fn test_skip_non_focusable_panes() {
        let mut container = PaneContainer::default();
        let id1 = PaneId::new("pane1");
        let id2 = PaneId::new("pane2");
        let id3 = PaneId::new("pane3");

        container.add_pane(Pane::new(id1, Box::new(MockContent::new("Pane 1"))));
        container.add_pane(Pane::new(
            id2,
            Box::new(MockContent::non_focusable("Status")),
        ));
        container.add_pane(Pane::new(id3, Box::new(MockContent::new("Pane 3"))));

        // Should skip id2 (non-focusable)
        let next = container.select_next(Some(id1));
        assert_eq!(next, Some(id3));

        let prev = container.select_prev(Some(id3));
        assert_eq!(prev, Some(id1));
    }

    #[test]
    fn test_find_pane_at() {
        let mut container = PaneContainer::default();
        let id1 = PaneId::new("pane1");
        let id2 = PaneId::new("pane2");

        let mut pane1 = Pane::new(id1, Box::new(MockContent::new("Pane 1")));
        pane1.set_area(Rect::new(0, 0, 40, 20));

        let mut pane2 = Pane::new(id2, Box::new(MockContent::new("Pane 2")));
        pane2.set_area(Rect::new(40, 0, 40, 20));

        container.add_pane(pane1);
        container.add_pane(pane2);

        assert_eq!(container.find_pane_at(20, 10), Some(id1));
        assert_eq!(container.find_pane_at(60, 10), Some(id2));
        assert_eq!(container.find_pane_at(100, 10), None);
    }

    #[test]
    fn test_directional_navigation_horizontal() {
        let mut container = PaneContainer::default();
        let id1 = PaneId::new("left");
        let id2 = PaneId::new("right");

        let mut pane1 = Pane::new(id1, Box::new(MockContent::new("Left")));
        pane1.set_area(Rect::new(0, 0, 40, 20));

        let mut pane2 = Pane::new(id2, Box::new(MockContent::new("Right")));
        pane2.set_area(Rect::new(40, 0, 40, 20));

        container.add_pane(pane1);
        container.add_pane(pane2);

        // From left, go right
        assert_eq!(container.select_right(id1), Some(id2));
        // From right, go left
        assert_eq!(container.select_left(id2), Some(id1));
        // Can't go further right
        assert_eq!(container.select_right(id2), None);
        // Can't go further left
        assert_eq!(container.select_left(id1), None);
    }

    #[test]
    fn test_update_layout() {
        let mut container = PaneContainer::new(PaneLayout::Horizontal(vec![50, 50]));
        let id1 = PaneId::new("pane1");
        let id2 = PaneId::new("pane2");

        container.add_pane(Pane::new(id1, Box::new(MockContent::new("Pane 1"))));
        container.add_pane(Pane::new(id2, Box::new(MockContent::new("Pane 2"))));

        container.update_layout(Rect::new(0, 0, 100, 50));

        let pane1 = container.get_pane(id1).unwrap();
        let pane2 = container.get_pane(id2).unwrap();

        assert_ne!(pane1.area(), Rect::default());
        assert_ne!(pane2.area(), Rect::default());
    }

    #[test]
    fn test_directional_navigation_vertical() {
        let mut container = PaneContainer::default();
        let id1 = PaneId::new("top");
        let id2 = PaneId::new("bottom");

        let mut pane1 = Pane::new(id1, Box::new(MockContent::new("Top")));
        pane1.set_area(Rect::new(0, 0, 40, 20));

        let mut pane2 = Pane::new(id2, Box::new(MockContent::new("Bottom")));
        pane2.set_area(Rect::new(0, 20, 40, 20));

        container.add_pane(pane1);
        container.add_pane(pane2);

        // From top, go down
        assert_eq!(container.select_down(id1), Some(id2));
        // From bottom, go up
        assert_eq!(container.select_up(id2), Some(id1));
        // Can't go further down
        assert_eq!(container.select_down(id2), None);
        // Can't go further up
        assert_eq!(container.select_up(id1), None);
    }

    #[test]
    fn test_get_pane_by_index() {
        let mut container = PaneContainer::default();
        let id1 = PaneId::new("pane1");
        let id2 = PaneId::new("pane2");

        container.add_pane(Pane::new(id1, Box::new(MockContent::new("Pane 1"))));
        container.add_pane(Pane::new(id2, Box::new(MockContent::new("Pane 2"))));

        assert_eq!(container.get_pane_by_index(0).unwrap().id(), id1);
        assert_eq!(container.get_pane_by_index(1).unwrap().id(), id2);
        assert!(container.get_pane_by_index(2).is_none());
    }

    #[test]
    fn test_get_pane_mut() {
        let mut container = PaneContainer::default();
        let id1 = PaneId::new("pane1");

        container.add_pane(Pane::new(id1, Box::new(MockContent::new("Pane 1"))));

        let pane = container.get_pane_mut(id1);
        assert!(pane.is_some());

        let non_existent = PaneId::new("nonexistent");
        assert!(container.get_pane_mut(non_existent).is_none());
    }

    #[test]
    fn test_resizable_splits_initialized() {
        let mut container = PaneContainer::new(PaneLayout::Horizontal(vec![50, 50]));
        let id1 = PaneId::new("pane1");
        let id2 = PaneId::new("pane2");

        container.add_pane(Pane::new(id1, Box::new(MockContent::new("Pane 1"))));
        container.add_pane(Pane::new(id2, Box::new(MockContent::new("Pane 2"))));

        // 2 panes should have 1 divider
        assert_eq!(container.resizable_splits.len(), 1);
    }

    #[test]
    fn test_find_divider_at() {
        let mut container = PaneContainer::new(PaneLayout::Horizontal(vec![50, 50]));
        container.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("P1")),
        ));
        container.add_pane(Pane::new(
            PaneId::new("p2"),
            Box::new(MockContent::new("P2")),
        ));

        // Update layout to calculate divider positions
        container.update_layout(Rect::new(0, 0, 100, 50));

        // Divider should be at column 50 (50% of 100)
        // With 3-column hit area: 49, 50, 51
        assert!(container.find_divider_at(50, 25).is_some());
        assert!(container.find_divider_at(49, 25).is_some());
        assert!(container.find_divider_at(51, 25).is_some());

        // Far from divider
        assert!(container.find_divider_at(10, 25).is_none());
        assert!(container.find_divider_at(90, 25).is_none());
    }

    #[test]
    fn test_drag_start_stop() {
        let mut container = PaneContainer::new(PaneLayout::Horizontal(vec![50, 50]));
        container.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("P1")),
        ));
        container.add_pane(Pane::new(
            PaneId::new("p2"),
            Box::new(MockContent::new("P2")),
        ));

        assert!(!container.is_dragging());

        container.start_drag(0);
        assert!(container.is_dragging());

        container.stop_drag();
        assert!(!container.is_dragging());
    }

    #[test]
    fn test_drag_updates_split() {
        let mut container = PaneContainer::new(PaneLayout::Horizontal(vec![50, 50]));
        container.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("P1")),
        ));
        container.add_pane(Pane::new(
            PaneId::new("p2"),
            Box::new(MockContent::new("P2")),
        ));

        let area = Rect::new(0, 0, 100, 50);
        container.update_layout(area);

        // Start dragging
        container.start_drag(0);

        // Drag to 70% position (column 70)
        container.update_drag(70, 25);

        // Update layout to reflect new position
        container.update_layout(area);

        // Check that split percent changed
        assert!(container.resizable_splits[0].split_percent > 50);
        assert!(container.resizable_splits[0].split_percent <= 70);
    }

    #[test]
    fn test_vertical_layout_resizable() {
        let mut container = PaneContainer::new(PaneLayout::Vertical(vec![50, 50]));
        container.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("P1")),
        ));
        container.add_pane(Pane::new(
            PaneId::new("p2"),
            Box::new(MockContent::new("P2")),
        ));

        // 2 panes should have 1 divider
        assert_eq!(container.resizable_splits.len(), 1);

        // Update layout
        container.update_layout(Rect::new(0, 0, 100, 100));

        // Divider should be at row 50 (50% of 100)
        assert!(container.find_divider_at(50, 50).is_some());
    }

    #[test]
    fn test_three_panes_two_dividers() {
        let mut container = PaneContainer::new(PaneLayout::Horizontal(vec![33, 33, 34]));
        container.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("P1")),
        ));
        container.add_pane(Pane::new(
            PaneId::new("p2"),
            Box::new(MockContent::new("P2")),
        ));
        container.add_pane(Pane::new(
            PaneId::new("p3"),
            Box::new(MockContent::new("P3")),
        ));

        // 3 panes should have 2 dividers
        assert_eq!(container.resizable_splits.len(), 2);
    }

    #[test]
    fn test_grid_layout_no_resizing() {
        let mut container = PaneContainer::new(PaneLayout::Grid { rows: 2, cols: 2 });
        container.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("P1")),
        ));
        container.add_pane(Pane::new(
            PaneId::new("p2"),
            Box::new(MockContent::new("P2")),
        ));

        // Grid layout doesn't support resizing - no dividers
        assert_eq!(container.resizable_splits.len(), 0);
    }
}

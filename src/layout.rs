//! Layout management with three-region geometry.

use ratatui::layout::Rect;
use std::time::{Duration, Instant};
use tracing::{debug, trace};

use crate::error::{LayoutError, LayoutResult};
use crate::registry::ElementRegistry;
use crate::types::{ElementId, LayoutState, Region, ResizeDebounceState};

const MIN_TERMINAL_WIDTH: u16 = 10;
const MIN_TERMINAL_HEIGHT: u16 = 5;
const DEFAULT_RESIZE_DEBOUNCE: Duration = Duration::from_millis(16);

/// Layout manager for computing element geometry.
#[derive(Debug)]
pub struct LayoutManager {
    registry: ElementRegistry,
    state: LayoutState,
    dirty: bool,
    resize_debounce: Duration,
    debounce_state: ResizeDebounceState,
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutManager {
    pub fn new() -> Self {
        Self {
            registry: ElementRegistry::new(),
            state: LayoutState::new(Rect::new(0, 0, 0, 0)),
            dirty: true,
            resize_debounce: DEFAULT_RESIZE_DEBOUNCE,
            debounce_state: ResizeDebounceState::Idle,
        }
    }

    pub fn with_resize_debounce(mut self, debounce: Duration) -> Self {
        self.resize_debounce = debounce;
        self
    }

    pub fn resize_debounce(&self) -> Duration {
        self.resize_debounce
    }

    pub fn set_resize_debounce(&mut self, debounce: Duration) {
        self.resize_debounce = debounce;
    }

    pub fn registry(&self) -> &ElementRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut ElementRegistry {
        &mut self.registry
    }

    pub fn state(&self) -> &LayoutState {
        &self.state
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Update layout for a terminal resize.
    pub fn on_resize(&mut self, width: u16, height: u16) -> LayoutResult<()> {
        if width < MIN_TERMINAL_WIDTH || height < MIN_TERMINAL_HEIGHT {
            return Err(LayoutError::terminal_too_small(
                MIN_TERMINAL_WIDTH,
                MIN_TERMINAL_HEIGHT,
                width,
                height,
            ));
        }

        match self.debounce_state {
            ResizeDebounceState::Pending {
                pending_width,
                pending_height,
                scheduled_at,
            } => {
                if scheduled_at.elapsed() >= self.resize_debounce {
                    self.state.terminal_area = Rect::new(0, 0, width, height);
                    self.mark_dirty();
                    self.debounce_state = ResizeDebounceState::Idle;
                    debug!("Layout resize (debounced): {}x{}", width, height);
                    self.recompute()
                } else {
                    self.debounce_state = ResizeDebounceState::Pending {
                        pending_width: width,
                        pending_height: height,
                        scheduled_at,
                    };
                    debug!("Layout resize debounced: {}x{}", width, height);
                    Ok(())
                }
            }
            ResizeDebounceState::Idle => {
                self.state.terminal_area = Rect::new(0, 0, width, height);
                self.mark_dirty();
                debug!(
                    "Layout resize: {}x{} (area: {:?})",
                    width, height, self.state.terminal_area
                );
                self.recompute()
            }
        }
    }

    /// Schedule a resize with debouncing.
    pub fn schedule_resize(&mut self, width: u16, height: u16) {
        self.debounce_state = ResizeDebounceState::Pending {
            pending_width: width,
            pending_height: height,
            scheduled_at: Instant::now(),
        };
        debug!(
            "Resize scheduled: {}x{} (debounce: {:?})",
            width, height, self.resize_debounce
        );
    }

    /// Process any pending resize.
    pub fn process_pending_resize(&mut self) -> LayoutResult<()> {
        match self.debounce_state {
            ResizeDebounceState::Pending {
                pending_width,
                pending_height,
                ..
            } => {
                if pending_width < MIN_TERMINAL_WIDTH || pending_height < MIN_TERMINAL_HEIGHT {
                    self.debounce_state = ResizeDebounceState::Idle;
                    return Err(LayoutError::terminal_too_small(
                        MIN_TERMINAL_WIDTH,
                        MIN_TERMINAL_HEIGHT,
                        pending_width,
                        pending_height,
                    ));
                }

                self.state.terminal_area = Rect::new(0, 0, pending_width, pending_height);
                self.mark_dirty();
                self.debounce_state = ResizeDebounceState::Idle;
                debug!(
                    "Pending resize processed: {}x{}",
                    pending_width, pending_height
                );
                self.recompute()
            }
            ResizeDebounceState::Idle => Ok(()),
        }
    }

    /// Check if there's a pending resize.
    pub fn has_pending_resize(&self) -> bool {
        matches!(self.debounce_state, ResizeDebounceState::Pending { .. })
    }

    /// Get the pending resize dimensions.
    pub fn pending_resize(&self) -> Option<(u16, u16)> {
        match self.debounce_state {
            ResizeDebounceState::Pending {
                pending_width,
                pending_height,
                ..
            } => Some((pending_width, pending_height)),
            ResizeDebounceState::Idle => None,
        }
    }

    /// Recompute layout geometry based on registered elements.
    pub fn recompute(&mut self) -> LayoutResult<()> {
        if !self.dirty {
            return Ok(());
        }

        let area = self.state.terminal_area;
        let mut top_height: u16 = 0;
        let mut bottom_height: u16 = 0;

        for id in self.registry.all_ids() {
            let metadata = self.registry.get_metadata(id)?;

            if !metadata.is_visible() {
                continue;
            }

            if let Some(fixed) = metadata.fixed_height {
                match metadata.region {
                    Region::Top => {
                        top_height = top_height.max(fixed);
                    }
                    Region::Bottom => {
                        bottom_height = bottom_height.max(fixed);
                    }
                    Region::Center => {}
                }
            }
        }

        let remaining_height = area
            .height
            .saturating_sub(top_height)
            .saturating_sub(bottom_height);

        if remaining_height < 1 {
            return Err(LayoutError::layout_computation(
                "Insufficient height for center region",
            ));
        }

        let top_area = if top_height > 0 {
            Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: top_height,
            }
        } else {
            Rect::default()
        };

        let center_area = Rect {
            x: area.x,
            y: area.y + top_height,
            width: area.width,
            height: remaining_height,
        };

        let bottom_area = if bottom_height > 0 {
            Rect {
                x: area.x,
                y: area.y + top_height + remaining_height,
                width: area.width,
                height: bottom_height,
            }
        } else {
            Rect::default()
        };

        self.state.top_area = top_area;
        self.state.center_area = center_area;
        self.state.bottom_area = bottom_area;
        self.state.top_height = top_height;
        self.state.bottom_height = bottom_height;

        self.assign_element_rects(top_area, center_area, bottom_area)?;

        self.dirty = false;

        debug!(
            "Layout recomputed: top={:?} center={:?} bottom={:?}",
            top_area, center_area, bottom_area
        );

        Ok(())
    }

    fn assign_element_rects(
        &mut self,
        top_area: Rect,
        center_area: Rect,
        bottom_area: Rect,
    ) -> LayoutResult<()> {
        for id in self.registry.all_ids() {
            let metadata = self.registry.get_metadata_mut(id)?;

            if !metadata.is_visible() {
                metadata.rect = Rect::default();
                continue;
            }

            metadata.rect = match metadata.region {
                Region::Top => top_area,
                Region::Center => center_area,
                Region::Bottom => bottom_area,
            };

            trace!("Assigned rect to element {}: {:?}", id, metadata.rect);
        }

        Ok(())
    }

    /// Get the area for a specific region.
    pub fn get_region_area(&self, region: Region) -> Rect {
        match region {
            Region::Top => self.state.top_area,
            Region::Center => self.state.center_area,
            Region::Bottom => self.state.bottom_area,
        }
    }

    /// Get elements at a position (x, y) in z-order (top-most first).
    pub fn hit_test(&self, x: u16, y: u16) -> Vec<(ElementId, Rect)> {
        let mut hits = Vec::new();

        for id in self.registry.all_ids() {
            let metadata = match self.registry.get_metadata(id) {
                Ok(m) => m,
                Err(_) => continue,
            };

            if !metadata.is_visible() {
                continue;
            }

            let rect = metadata.rect;
            if x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height {
                hits.push((id, rect));
            }
        }

        hits.sort_by(|a, b| {
            let meta_a = self.registry.get_metadata(a.0).ok();
            let meta_b = self.registry.get_metadata(b.0).ok();
            match (meta_a, meta_b) {
                (Some(a_meta), Some(b_meta)) => b_meta.z_order.cmp(&a_meta.z_order),
                _ => std::cmp::Ordering::Equal,
            }
        });

        hits
    }

    /// Get top-most element at position (x, y).
    pub fn hit_test_top(&self, x: u16, y: u16) -> Option<ElementId> {
        self.hit_test(x, y).first().map(|(id, _)| *id)
    }

    /// Validate current layout state.
    pub fn validate(&self) -> LayoutResult<()> {
        if self.state.terminal_area.width < MIN_TERMINAL_WIDTH
            || self.state.terminal_area.height < MIN_TERMINAL_HEIGHT
        {
            return Err(LayoutError::terminal_too_small(
                MIN_TERMINAL_WIDTH,
                MIN_TERMINAL_HEIGHT,
                self.state.terminal_area.width,
                self.state.terminal_area.height,
            ));
        }

        let total_height =
            self.state.top_height + self.state.center_area.height + self.state.bottom_height;

        if total_height != self.state.terminal_area.height {
            return Err(LayoutError::layout_computation(format!(
                "Height mismatch: {} != {}",
                total_height, self.state.terminal_area.height
            )));
        }

        Ok(())
    }

    /// Get the rect for a specific element.
    pub fn get_element_rect(&self, id: ElementId) -> Option<Rect> {
        self.registry.get_metadata(id).ok().map(|m| m.rect)
    }

    /// Get all elements sorted by z-order (highest first).
    pub fn all_hits_sorted_by_z_order(&self) -> Vec<(ElementId, Rect)> {
        let mut hits: Vec<(ElementId, Rect)> = self
            .registry
            .all_ids()
            .into_iter()
            .filter_map(|id| {
                self.registry
                    .get_metadata(id)
                    .ok()
                    .filter(|m| m.is_visible())
                    .map(|m| (id, m.rect))
            })
            .collect();

        hits.sort_by(|a, b| {
            let meta_a = self.registry.get_metadata(a.0).ok();
            let meta_b = self.registry.get_metadata(b.0).ok();
            match (meta_a, meta_b) {
                (Some(a_meta), Some(b_meta)) => b_meta.z_order.cmp(&a_meta.z_order),
                _ => std::cmp::Ordering::Equal,
            }
        });

        hits
    }

    /// Get all element IDs sorted by z-order (highest first).
    pub fn all_ids_sorted_by_z_order(&self) -> Vec<ElementId> {
        self.all_hits_sorted_by_z_order()
            .into_iter()
            .map(|(id, _)| id)
            .collect()
    }

    /// Get the z-order for a specific element.
    pub fn get_element_z_order(&self, id: ElementId) -> Option<u32> {
        self.registry.get_metadata(id).ok().map(|m| m.z_order)
    }

    /// Get terminal dimensions.
    pub fn terminal_size(&self) -> (u16, u16) {
        (
            self.state.terminal_area.width,
            self.state.terminal_area.height,
        )
    }

    /// Get layout statistics for diagnostics.
    pub fn get_layout_stats(&self) -> LayoutStats {
        let all_ids = self.registry.all_ids();
        let visible_count = all_ids
            .iter()
            .filter(|&id| {
                self.registry
                    .get_metadata(*id)
                    .map(|m| m.is_visible())
                    .unwrap_or(false)
            })
            .count();

        LayoutStats {
            total_elements: all_ids.len(),
            visible_elements: visible_count,
            terminal_width: self.state.terminal_area.width,
            terminal_height: self.state.terminal_area.height,
            top_height: self.state.top_height,
            center_height: self.state.center_area.height,
            bottom_height: self.state.bottom_height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayoutStats {
    pub total_elements: usize,
    pub visible_elements: usize,
    pub terminal_width: u16,
    pub terminal_height: u16,
    pub top_height: u16,
    pub center_height: u16,
    pub bottom_height: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ElementMetadata;
    use std::sync::Arc;

    fn create_test_manager() -> LayoutManager {
        LayoutManager::new()
    }

    #[test]
    fn test_layout_manager_init() {
        let manager = create_test_manager();
        assert!(manager.dirty);
        assert_eq!(manager.state().terminal_area, Rect::default());
    }

    #[test]
    fn test_layout_resize() {
        let mut manager = create_test_manager();
        let result = manager.on_resize(80, 24);

        assert!(result.is_ok());
        assert_eq!(manager.state().terminal_area, Rect::new(0, 0, 80, 24));
    }

    #[test]
    fn test_layout_too_small() {
        let mut manager = create_test_manager();
        let result = manager.on_resize(5, 3);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LayoutError::TerminalTooSmall(_, _, _, _)
        ));
    }

    #[test]
    fn test_recompute_empty() {
        let mut manager = create_test_manager();
        manager.on_resize(80, 24).unwrap();
        let result = manager.recompute();

        assert!(result.is_ok());
        assert!(!manager.is_dirty());
    }

    #[test]
    fn test_recompute_with_fixed_heights() {
        let mut manager = create_test_manager();

        let id1 = ElementId::new();
        let metadata1 = ElementMetadata::new(id1, Region::Top).with_fixed_height(3);

        let id2 = ElementId::new();
        let metadata2 = ElementMetadata::new(id2, Region::Bottom).with_fixed_height(2);

        let _ = manager
            .registry_mut()
            .register(metadata1, Arc::new(DummyElement::new(id1)));
        let _ = manager
            .registry_mut()
            .register(metadata2, Arc::new(DummyElement::new(id2)));

        manager.on_resize(80, 24).unwrap();
        manager.recompute().unwrap();

        assert_eq!(manager.state().top_height, 3);
        assert_eq!(manager.state().bottom_height, 2);
        assert_eq!(manager.state().center_area.height, 19);
    }

    #[test]
    fn test_hit_test() {
        let mut manager = create_test_manager();

        let id = ElementId::new();
        let metadata = ElementMetadata::new(id, Region::Center).with_z_order(10);

        let _ = manager
            .registry_mut()
            .register(metadata, Arc::new(DummyElement::new(id)));

        manager.on_resize(80, 24).unwrap();
        manager.recompute().unwrap();

        let hits = manager.hit_test(10, 5);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].0, id);

        let top = manager.hit_test_top(10, 5);
        assert_eq!(top, Some(id));
    }

    #[test]
    fn test_validate() {
        let mut manager = create_test_manager();
        manager.on_resize(80, 24).unwrap();
        manager.recompute().unwrap();

        assert!(manager.validate().is_ok());
    }

    #[test]
    fn test_region_areas() {
        let mut manager = create_test_manager();

        let id1 = ElementId::new();
        let metadata1 = ElementMetadata::new(id1, Region::Top).with_fixed_height(3);

        let id2 = ElementId::new();
        let metadata2 = ElementMetadata::new(id2, Region::Bottom).with_fixed_height(2);

        let _ = manager
            .registry_mut()
            .register(metadata1, Arc::new(DummyElement::new(id1)));
        let _ = manager
            .registry_mut()
            .register(metadata2, Arc::new(DummyElement::new(id2)));

        manager.on_resize(80, 24).unwrap();
        manager.recompute().unwrap();

        let top_area = manager.get_region_area(Region::Top);
        assert_eq!(top_area.height, 3);

        let center_area = manager.get_region_area(Region::Center);
        assert_eq!(center_area.height, 19);

        let bottom_area = manager.get_region_area(Region::Bottom);
        assert_eq!(bottom_area.height, 2);
    }

    #[test]
    fn test_get_element_rect() {
        let mut manager = create_test_manager();
        let id = ElementId::new();
        let metadata = ElementMetadata::new(id, Region::Center);

        let _ = manager
            .registry_mut()
            .register(metadata, Arc::new(DummyElement::new(id)));
        manager.on_resize(80, 24).unwrap();
        manager.recompute().unwrap();

        let rect = manager.get_element_rect(id);
        assert!(rect.is_some());
        assert_eq!(rect.unwrap(), manager.state().center_area);
    }

    #[test]
    fn test_all_hits_sorted_by_z_order() {
        let mut manager = create_test_manager();

        let id1 = ElementId::new();
        let metadata1 = ElementMetadata::new(id1, Region::Center).with_z_order(5);

        let id2 = ElementId::new();
        let metadata2 = ElementMetadata::new(id2, Region::Center).with_z_order(10);

        let _ = manager
            .registry_mut()
            .register(metadata1, Arc::new(DummyElement::new(id1)));
        let _ = manager
            .registry_mut()
            .register(metadata2, Arc::new(DummyElement::new(id2)));

        manager.on_resize(80, 24).unwrap();
        manager.recompute().unwrap();

        let hits = manager.all_hits_sorted_by_z_order();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].0, id2);
        assert_eq!(hits[1].0, id1);
    }

    #[test]
    fn test_get_element_z_order() {
        let mut manager = create_test_manager();
        let id = ElementId::new();
        let metadata = ElementMetadata::new(id, Region::Center).with_z_order(42);

        let _ = manager
            .registry_mut()
            .register(metadata, Arc::new(DummyElement::new(id)));

        let z_order = manager.get_element_z_order(id);
        assert_eq!(z_order, Some(42));
    }

    #[test]
    fn test_terminal_size() {
        let mut manager = create_test_manager();
        manager.on_resize(80, 24).unwrap();

        let (width, height) = manager.terminal_size();
        assert_eq!(width, 80);
        assert_eq!(height, 24);
    }

    #[test]
    fn test_get_layout_stats() {
        let mut manager = create_test_manager();
        manager.on_resize(80, 24).unwrap();

        let stats = manager.get_layout_stats();
        assert_eq!(stats.total_elements, 0);
        assert_eq!(stats.terminal_width, 80);
        assert_eq!(stats.terminal_height, 24);
    }

    #[test]
    fn test_resize_debounce() {
        let mut manager = LayoutManager::new().with_resize_debounce(Duration::from_millis(50));

        manager.schedule_resize(100, 30);
        assert!(manager.has_pending_resize());
        assert_eq!(manager.pending_resize(), Some((100, 30)));
    }

    struct DummyElement {
        id: ElementId,
    }

    impl DummyElement {
        fn new(id: ElementId) -> Self {
            Self { id }
        }
    }

    impl crate::registry::Element for DummyElement {
        fn id(&self) -> ElementId {
            self.id
        }

        fn on_metadata_update(&self, _metadata: &ElementMetadata) {}

        fn on_render(&self) {}

        fn on_keyboard(&self, _event: &crate::events::KeyboardEvent) -> bool {
            false
        }

        fn on_mouse(&self, _event: &crate::events::MouseEvent) -> bool {
            false
        }

        fn on_focus_gain(&self) {}

        fn on_focus_loss(&self) {}

        fn on_tick(&self) {}
    }
}

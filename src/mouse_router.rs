//! Mouse routing with z-order snapshot and capture semantics.

use std::time::{Duration, Instant};
use tracing::{debug, trace};

use crate::error::LayoutResult;
use crate::layout::LayoutManager;
use crate::types::{ElementId, MouseCaptureState, MouseSnapshot};

const DEFAULT_CAPTURE_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_SNAPSHOT_MAX_AGE: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, Copy)]
pub struct MouseRouterConfig {
    pub capture_timeout: Duration,
    pub snapshot_max_age: Duration,
    pub auto_release_on_click_outside: bool,
}

impl Default for MouseRouterConfig {
    fn default() -> Self {
        Self {
            capture_timeout: DEFAULT_CAPTURE_TIMEOUT,
            snapshot_max_age: DEFAULT_SNAPSHOT_MAX_AGE,
            auto_release_on_click_outside: true,
        }
    }
}

#[derive(Debug)]
pub struct MouseRouter {
    config: MouseRouterConfig,
    capture_state: MouseCaptureState,
    last_snapshot: Option<MouseSnapshot>,
    last_update: Instant,
}

impl Default for MouseRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseRouter {
    pub fn new() -> Self {
        Self {
            config: MouseRouterConfig::default(),
            capture_state: MouseCaptureState::None,
            last_snapshot: None,
            last_update: Instant::now(),
        }
    }

    pub fn with_config(mut self, config: MouseRouterConfig) -> Self {
        self.config = config;
        self
    }

    pub fn config(&self) -> MouseRouterConfig {
        self.config
    }

    pub fn capture(&mut self, element_id: ElementId) -> LayoutResult<()> {
        self.capture_state = MouseCaptureState::Captured {
            element_id,
            captured_at: Instant::now(),
            timeout: Some(self.config.capture_timeout),
        };
        debug!("Mouse capture started for element: {}", element_id);
        self.last_update = Instant::now();
        Ok(())
    }

    pub fn release_capture(&mut self) {
        if let Some(id) = self.capture_state.element_id() {
            debug!("Mouse capture released for element: {}", id);
        }
        self.capture_state = MouseCaptureState::None;
        self.last_snapshot = None;
        self.last_update = Instant::now();
    }

    pub fn is_captured(&self) -> bool {
        self.capture_state.is_captured()
    }

    pub fn captured_element(&self) -> Option<ElementId> {
        self.capture_state.element_id()
    }

    pub fn capture_state(&self) -> MouseCaptureState {
        self.capture_state
    }

    pub fn check_capture_expired(&mut self) -> bool {
        if self.capture_state.is_expired() {
            if let Some(id) = self.capture_state.element_id() {
                debug!("Mouse capture expired for element: {}", id);
            }
            self.capture_state = MouseCaptureState::None;
            true
        } else {
            false
        }
    }

    pub fn remaining_capture_time(&self) -> Option<Duration> {
        self.capture_state.remaining_time()
    }

    pub fn take_snapshot(&mut self, layout: &LayoutManager) -> MouseSnapshot {
        let captured_element = self.capture_state.element_id();
        let z_order_hits = layout.all_hits_sorted_by_z_order();
        let hits_len = z_order_hits.len();

        let snapshot = MouseSnapshot::new(captured_element, z_order_hits);
        self.last_snapshot = Some(snapshot.clone());
        self.last_update = Instant::now();

        trace!("Mouse snapshot taken: {} hits", hits_len);
        snapshot
    }

    pub fn snapshot(&self) -> Option<&MouseSnapshot> {
        self.last_snapshot.as_ref()
    }

    pub fn is_snapshot_stale(&self) -> bool {
        if let Some(snapshot) = &self.last_snapshot {
            snapshot.is_stale(self.config.snapshot_max_age)
        } else {
            true
        }
    }

    pub fn should_reroute_mouse(&self, x: u16, y: u16, layout: &LayoutManager) -> bool {
        if self.is_captured() && !self.capture_state.is_expired() {
            if let Some(captured_id) = self.capture_state.element_id() {
                if let Some(hit_rect) = layout.get_element_rect(captured_id) {
                    let is_inside = x >= hit_rect.x
                        && x < hit_rect.x + hit_rect.width
                        && y >= hit_rect.y
                        && y < hit_rect.y + hit_rect.height;
                    return is_inside;
                }
            }
        }
        false
    }

    pub fn route_mouse_event(
        &mut self,
        x: u16,
        y: u16,
        layout: &LayoutManager,
    ) -> Vec<(ElementId, Rect)> {
        self.take_snapshot(layout);

        if self.is_captured() && !self.capture_state.is_expired() {
            if let Some(captured_id) = self.capture_state.element_id() {
                if let Some(rect) = layout.get_element_rect(captured_id) {
                    if x >= rect.x
                        && x < rect.x + rect.width
                        && y >= rect.y
                        && y < rect.y + rect.height
                    {
                        return vec![(captured_id, rect)];
                    }
                }
                return vec![];
            }
        }

        layout.hit_test(x, y)
    }

    pub fn last_update(&self) -> Instant {
        self.last_update
    }

    pub fn validate_capture(&mut self, element_id: ElementId) -> bool {
        self.capture_state.element_id() == Some(element_id)
    }

    pub fn handle_click_outside(&mut self, x: u16, y: u16, layout: &LayoutManager) -> bool {
        if self.config.auto_release_on_click_outside && self.is_captured() {
            if let Some(captured_id) = self.capture_state.element_id() {
                if let Some(rect) = layout.get_element_rect(captured_id) {
                    let is_outside = x < rect.x
                        || x >= rect.x + rect.width
                        || y < rect.y
                        || y >= rect.y + rect.height;

                    if is_outside {
                        self.release_capture();
                        debug!("Auto-released mouse capture due to click outside");
                        return true;
                    }
                }
            }
        }
        false
    }
}

use ratatui::layout::Rect;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutManager;
    use crate::types::{ElementMetadata, Region};
    use std::sync::Arc;

    fn create_test_router() -> MouseRouter {
        MouseRouter::new()
    }

    fn create_test_layout() -> LayoutManager {
        let mut layout = LayoutManager::new();
        layout.on_resize(80, 24).unwrap();
        layout
    }

    #[test]
    fn test_mouse_router_init() {
        let router = create_test_router();
        assert!(!router.is_captured());
        assert!(router.captured_element().is_none());
    }

    #[test]
    fn test_mouse_capture() {
        let mut router = create_test_router();
        let id = ElementId::new();

        router.capture(id).unwrap();
        assert!(router.is_captured());
        assert_eq!(router.captured_element(), Some(id));
    }

    #[test]
    fn test_mouse_capture_release() {
        let mut router = create_test_router();
        let id = ElementId::new();

        router.capture(id).unwrap();
        router.release_capture();

        assert!(!router.is_captured());
        assert!(router.captured_element().is_none());
    }

    #[test]
    fn test_mouse_capture_with_timeout() {
        let mut router = create_test_router();
        let id = ElementId::new();

        router.capture(id).unwrap();
        assert!(!router.check_capture_expired());

        router.release_capture();
        assert!(!router.is_captured());
    }

    #[test]
    fn test_snapshot_taken() {
        let mut router = create_test_router();
        let layout = create_test_layout();

        let snapshot = router.take_snapshot(&layout);
        assert!(router.snapshot().is_some());
        assert!(router.last_update() <= Instant::now());
    }

    #[test]
    fn test_route_mouse_event_captured() {
        let mut router = create_test_router();
        let mut layout = create_test_layout();

        let id = ElementId::new();
        let metadata = ElementMetadata::new(id, Region::Center);
        let _ = layout
            .registry_mut()
            .register(metadata, Arc::new(DummyElement::new(id)));

        layout.mark_dirty();
        layout.recompute().unwrap();

        let rect = layout.get_element_rect(id);
        assert!(rect.is_some(), "Element rect should be assigned");
        let rect = rect.unwrap();
        assert!(
            rect.width > 0 && rect.height > 0,
            "Element rect should have size"
        );

        router.capture(id).unwrap();

        let hits = router.route_mouse_event(10, 5, &layout);
        assert_eq!(hits.len(), 1, "Should hit captured element at (10, 5)");
        assert_eq!(hits[0].0, id);
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

        fn on_metadata_update(&self, _metadata: &crate::types::ElementMetadata) {}

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

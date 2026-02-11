//! Coordinator for integrating LayoutManager, FocusManager, and MouseRouter.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, info};

use crate::error::LayoutResult;
use crate::events::{KeyboardEvent, MouseEvent, ResizeEvent};
use crate::focus::{FocusManager, FocusRequest};
use crate::layout::LayoutManager;
use crate::mouse_router::{MouseRouter, MouseRouterConfig};
use crate::registry::Element;
use crate::types::{DiagnosticInfo, DirtyFlags, ElementId, ElementMetadata, Region, Visibility};

#[derive(Debug, Clone, Copy)]
pub struct CoordinatorConfig {
    pub layout_debounce: Duration,
    pub mouse_router_config: MouseRouterConfig,
    pub tick_rate: Duration,
}

impl Default for CoordinatorConfig {
    fn default() -> Self {
        Self {
            layout_debounce: Duration::from_millis(16),
            mouse_router_config: MouseRouterConfig::default(),
            tick_rate: Duration::from_millis(50),
        }
    }
}

pub trait CoordinatorApp {
    fn on_event(&mut self, event: CoordinatorEvent) -> LayoutResult<CoordinatorAction>;
    fn on_draw(&mut self, frame: &mut ratatui::Frame);
    fn on_layout_changed(&mut self) {}
}

#[derive(Clone)]
pub enum CoordinatorEvent {
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
    Tick(u64),
    Resize(ResizeEvent),
    Focus(FocusRequest),
    Register(ElementMetadata, Arc<dyn Element>),
    Unregister(ElementId),
    SetVisibility(ElementId, Visibility),
    RequestDiagnosticInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinatorAction {
    Continue,
    Redraw,
    Quit,
}

#[derive(Debug)]
pub struct LayoutCoordinator<A: CoordinatorApp> {
    app: A,
    layout: LayoutManager,
    focus: FocusManager,
    mouse: MouseRouter,
    config: CoordinatorConfig,
    dirty: DirtyFlags,
    last_layout_invalidation: Option<Instant>,
    tick_count: u64,
    pending_resize: Option<(u16, u16)>,
}

impl<A: CoordinatorApp> LayoutCoordinator<A> {
    pub fn new(app: A) -> Self {
        Self {
            app,
            layout: LayoutManager::new(),
            focus: FocusManager::new(),
            mouse: MouseRouter::new(),
            config: CoordinatorConfig::default(),
            dirty: DirtyFlags::all_dirty(),
            last_layout_invalidation: None,
            tick_count: 0,
            pending_resize: None,
        }
    }

    pub fn with_config(mut self, config: CoordinatorConfig) -> Self {
        self.config = config;
        self.mouse = MouseRouter::new().with_config(config.mouse_router_config);
        self
    }

    pub fn app(&self) -> &A {
        &self.app
    }

    pub fn app_mut(&mut self) -> &mut A {
        &mut self.app
    }

    pub fn layout(&self) -> &LayoutManager {
        &self.layout
    }

    pub fn layout_mut(&mut self) -> &mut LayoutManager {
        &mut self.layout
    }

    pub fn focus(&self) -> &FocusManager {
        &self.focus
    }

    pub fn focus_mut(&mut self) -> &mut FocusManager {
        &mut self.focus
    }

    pub fn mouse(&self) -> &MouseRouter {
        &self.mouse
    }

    pub fn mouse_mut(&mut self) -> &mut MouseRouter {
        &mut self.mouse
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.needs_redraw()
    }

    pub fn invalidate_layout(&mut self) {
        if let Some(last) = self.last_layout_invalidation {
            if last.elapsed() < self.config.layout_debounce {
                debug!("Layout invalidation debounced");
                return;
            }
        }

        self.dirty.set_layout_dirty();
        self.last_layout_invalidation = Some(Instant::now());
        debug!("Layout marked dirty");
    }

    pub fn invalidate_elements(&mut self) {
        self.dirty.set_elements_dirty();
    }

    pub fn set_dirty(&mut self) {
        self.dirty = DirtyFlags::all_dirty();
    }

    pub fn clear_dirty(&mut self) {
        self.dirty.clear();
    }

    pub fn handle_event(&mut self, event: CoordinatorEvent) -> LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => self.handle_keyboard(keyboard),
            CoordinatorEvent::Mouse(mouse) => self.handle_mouse(mouse),
            CoordinatorEvent::Tick(count) => self.handle_tick(count),
            CoordinatorEvent::Resize(resize) => self.handle_resize(resize),
            CoordinatorEvent::Focus(request) => self.handle_focus(request),
            CoordinatorEvent::Register(metadata, element) => {
                self.handle_register(metadata, element)
            }
            CoordinatorEvent::Unregister(id) => self.handle_unregister(id),
            CoordinatorEvent::SetVisibility(id, visibility) => {
                self.handle_set_visibility(id, visibility)
            }
            CoordinatorEvent::RequestDiagnosticInfo => self.handle_diagnostic_request(),
        }
    }

    fn handle_keyboard(&mut self, keyboard: KeyboardEvent) -> LayoutResult<CoordinatorAction> {
        if let Some(focused_id) = self.focus.focused() {
            if let Ok(element) = self.layout.registry().get_strong_ref(focused_id) {
                if element.on_keyboard(&keyboard) {
                    self.invalidate_elements();
                    return Ok(CoordinatorAction::Redraw);
                }
            }
        }

        let action = self.app.on_event(CoordinatorEvent::Keyboard(keyboard))?;
        Ok(action)
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) -> LayoutResult<CoordinatorAction> {
        let (x, y) = mouse.position();

        self.mouse.check_capture_expired();

        let hits = self.mouse.route_mouse_event(x, y, &self.layout);

        for (id, _rect) in hits {
            if let Ok(element) = self.layout.registry().get_strong_ref(id) {
                if element.on_mouse(&mouse) {
                    self.invalidate_elements();
                    return Ok(CoordinatorAction::Redraw);
                }
            }
        }

        self.mouse.handle_click_outside(x, y, &self.layout);
        let action = self.app.on_event(CoordinatorEvent::Mouse(mouse))?;
        Ok(action)
    }

    fn handle_tick(&mut self, count: u64) -> LayoutResult<CoordinatorAction> {
        self.tick_count = count;

        for id in self.layout.registry().all_ids() {
            if let Ok(element) = self.layout.registry().get_strong_ref(id) {
                element.on_tick();
            }
        }

        self.mouse.check_capture_expired();

        let action = self.app.on_event(CoordinatorEvent::Tick(count))?;
        Ok(action)
    }

    fn handle_resize(&mut self, resize: ResizeEvent) -> LayoutResult<CoordinatorAction> {
        self.pending_resize = Some((resize.width, resize.height));

        if let Some(last) = self.last_layout_invalidation {
            if last.elapsed() < self.config.layout_debounce {
                debug!("Resize debounced");
                return Ok(CoordinatorAction::Continue);
            }
        }

        self.process_resize()?;
        Ok(CoordinatorAction::Redraw)
    }

    fn process_resize(&mut self) -> LayoutResult<()> {
        if let Some((width, height)) = self.pending_resize.take() {
            self.layout.on_resize(width, height)?;
            self.app.on_layout_changed();
            self.dirty.set_layout_dirty();
            self.last_layout_invalidation = Some(Instant::now());
            debug!("Resize processed: {}x{}", width, height);
        }
        Ok(())
    }

    fn handle_focus(&mut self, request: FocusRequest) -> LayoutResult<CoordinatorAction> {
        let previous = self.focus.focused();
        let result = self.focus.handle_request(request)?;

        if previous != result {
            if let Some(prev_id) = previous {
                if let Ok(element) = self.layout.registry().get_strong_ref(prev_id) {
                    element.on_focus_loss();
                }
            }
            if let Some(curr_id) = result {
                if let Ok(element) = self.layout.registry().get_strong_ref(curr_id) {
                    element.on_focus_gain();
                }
            }
            self.invalidate_elements();
            return Ok(CoordinatorAction::Redraw);
        }

        Ok(CoordinatorAction::Continue)
    }

    fn handle_register(
        &mut self,
        metadata: ElementMetadata,
        element: Arc<dyn Element>,
    ) -> LayoutResult<CoordinatorAction> {
        let id = metadata.id;

        self.layout
            .registry_mut()
            .register(metadata.clone(), element.clone())?;
        self.focus.registry_mut().register(metadata, element)?;

        self.invalidate_layout();

        debug!("Element registered: {}", id);
        Ok(CoordinatorAction::Continue)
    }

    fn handle_unregister(&mut self, id: ElementId) -> LayoutResult<CoordinatorAction> {
        self.layout.registry_mut().unregister(id)?;
        self.focus.remove_element(id)?;

        if self.focus.focused() == Some(id) {
            self.focus.handle_request(FocusRequest::First)?;
        }

        self.invalidate_layout();

        debug!("Element unregistered: {}", id);
        Ok(CoordinatorAction::Continue)
    }

    fn handle_set_visibility(
        &mut self,
        id: ElementId,
        visibility: Visibility,
    ) -> LayoutResult<CoordinatorAction> {
        self.layout.registry_mut().set_visibility(id, visibility)?;

        if visibility == Visibility::Hidden && self.focus.focused() == Some(id) {
            self.focus.handle_request(FocusRequest::First)?;
        }

        self.invalidate_layout();

        debug!("Element visibility changed: {} -> {:?}", id, visibility);
        Ok(CoordinatorAction::Continue)
    }

    fn handle_diagnostic_request(&mut self) -> LayoutResult<CoordinatorAction> {
        let diagnostic = self.get_diagnostic_info();
        info!("Diagnostic info: {:?}", diagnostic);
        Ok(CoordinatorAction::Continue)
    }

    pub fn get_diagnostic_info(&self) -> DiagnosticInfo {
        let registry = self.layout.registry();
        let focusable = registry.focusable_elements();

        DiagnosticInfo {
            total_elements: registry.len(),
            visible_elements: registry
                .all_ids()
                .iter()
                .filter(|&id| {
                    registry
                        .get_metadata(*id)
                        .map(|m| m.is_visible())
                        .unwrap_or(false)
                })
                .count(),
            focusable_elements: focusable.len(),
            focused_element: self.focus.focused(),
            captured_element: self.mouse.captured_element(),
            terminal_size: (
                self.layout.state().terminal_area.width,
                self.layout.state().terminal_area.height,
            ),
            region_areas: vec![
                (Region::Top, self.layout.get_region_area(Region::Top)),
                (Region::Center, self.layout.get_region_area(Region::Center)),
                (Region::Bottom, self.layout.get_region_area(Region::Bottom)),
            ],
            z_order_top: self
                .layout
                .registry()
                .all_ids()
                .into_iter()
                .filter_map(|id| {
                    registry
                        .get_metadata(id)
                        .ok()
                        .map(|m| (id, m.region, m.z_order))
                })
                .take(10)
                .collect(),
            dirty_flags: self.dirty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::Element;

    struct TestApp;

    impl CoordinatorApp for TestApp {
        fn on_event(&mut self, _event: CoordinatorEvent) -> LayoutResult<CoordinatorAction> {
            Ok(CoordinatorAction::Continue)
        }

        fn on_draw(&mut self, _frame: &mut ratatui::Frame) {}
    }

    struct DummyElement {
        id: ElementId,
    }

    impl DummyElement {
        fn new(id: ElementId) -> Self {
            Self { id }
        }
    }

    impl Element for DummyElement {
        fn id(&self) -> ElementId {
            self.id
        }

        fn on_metadata_update(&self, _metadata: &crate::types::ElementMetadata) {}

        fn on_render(&self) {}

        fn on_keyboard(&self, _event: &KeyboardEvent) -> bool {
            false
        }

        fn on_mouse(&self, _event: &MouseEvent) -> bool {
            false
        }

        fn on_focus_gain(&self) {}

        fn on_focus_loss(&self) {}

        fn on_tick(&self) {}
    }

    #[test]
    fn test_coordinator_init() {
        let app = TestApp;
        let coordinator = LayoutCoordinator::new(app);
        assert!(coordinator.is_dirty());
    }

    #[test]
    fn test_coordinator_register() {
        let app = TestApp;
        let mut coordinator = LayoutCoordinator::new(app);

        let id = ElementId::new();
        let metadata = ElementMetadata::new(id, Region::Center).with_focusable(true);
        let element = Arc::new(DummyElement::new(id));

        let action = coordinator
            .handle_event(CoordinatorEvent::Register(metadata, element))
            .unwrap();

        assert_eq!(action, CoordinatorAction::Continue);
        assert!(coordinator.layout.registry().len() == 1);
    }

    #[test]
    fn test_coordinator_resize() {
        let app = TestApp;
        let mut coordinator = LayoutCoordinator::new(app);

        let action = coordinator
            .handle_event(CoordinatorEvent::Resize(ResizeEvent::new(80, 24)))
            .unwrap();

        assert_eq!(action, CoordinatorAction::Redraw);
        assert_eq!(coordinator.layout.state().terminal_area.width, 80);
        assert_eq!(coordinator.layout.state().terminal_area.height, 24);
    }

    #[test]
    fn test_coordinator_diagnostic() {
        let app = TestApp;
        let coordinator = LayoutCoordinator::new(app);

        let diagnostic = coordinator.get_diagnostic_info();
        assert_eq!(diagnostic.total_elements, 0);
        assert!(diagnostic.focused_element.is_none());
    }
}

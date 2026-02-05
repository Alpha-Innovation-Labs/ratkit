//! Core runner coordinating layout and event dispatch.

use std::sync::Arc;
use std::time::Duration;

use crate::coordinator::{
    CoordinatorAction, CoordinatorApp, CoordinatorConfig, CoordinatorEvent, LayoutCoordinator,
};
use crate::error::LayoutResult;
use crate::events::{RunnerEvent as LayoutRunnerEvent, TickEvent};
use crate::focus::FocusRequest;
use crate::mouse_router::MouseRouterConfig;
use crate::registry::Element;
use crate::types::{ElementId, ElementMetadata, Visibility};
use ratatui::Frame;

/// Runner events routed to the core runtime.
pub type RunnerEvent = LayoutRunnerEvent;

/// Runner configuration for event cadence and routing.
#[derive(Debug, Clone, Copy)]
pub struct RunnerConfig {
    /// Duration between tick events.
    pub tick_rate: Duration,
    /// Debounce duration for layout invalidations.
    pub layout_debounce: Duration,
    /// Mouse routing configuration.
    pub mouse_router_config: MouseRouterConfig,
}

impl Default for RunnerConfig {
    fn default() -> Self {
        let base = CoordinatorConfig::default();
        Self {
            tick_rate: base.tick_rate,
            layout_debounce: base.layout_debounce,
            mouse_router_config: base.mouse_router_config,
        }
    }
}

impl RunnerConfig {
    fn coordinator_config(&self) -> CoordinatorConfig {
        CoordinatorConfig {
            layout_debounce: self.layout_debounce,
            mouse_router_config: self.mouse_router_config,
            tick_rate: self.tick_rate,
        }
    }
}

/// Action requested by the runner after handling an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunnerAction {
    /// Continue without redrawing.
    Continue,
    /// Redraw the frame.
    Redraw,
    /// Exit the loop.
    Quit,
}

/// Core runtime runner coordinating event dispatch and render passes.
#[derive(Debug)]
pub struct Runner<A: CoordinatorApp> {
    coordinator: LayoutCoordinator<A>,
    config: RunnerConfig,
    tick_count: u64,
}

impl<A: CoordinatorApp> Runner<A> {
    /// Create a new runner with default configuration.
    pub fn new(app: A) -> Self {
        let config = RunnerConfig::default();
        let coordinator = LayoutCoordinator::new(app).with_config(config.coordinator_config());
        Self {
            coordinator,
            config,
            tick_count: 0,
        }
    }

    /// Apply a custom runner configuration.
    pub fn with_config(mut self, config: RunnerConfig) -> Self {
        self.coordinator = self.coordinator.with_config(config.coordinator_config());
        self.config = config;
        self
    }

    /// Access the underlying coordinator.
    pub fn coordinator(&self) -> &LayoutCoordinator<A> {
        &self.coordinator
    }

    /// Mutable access to the underlying coordinator.
    pub fn coordinator_mut(&mut self) -> &mut LayoutCoordinator<A> {
        &mut self.coordinator
    }

    /// Runner configuration.
    pub fn config(&self) -> RunnerConfig {
        self.config
    }

    /// Current tick count.
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Handle a runner event and return the desired action.
    pub fn handle_event(&mut self, event: RunnerEvent) -> LayoutResult<RunnerAction> {
        let action = match event {
            RunnerEvent::Keyboard(keyboard) => {
                self.handle_coordinator_event(CoordinatorEvent::Keyboard(keyboard))?
            }
            RunnerEvent::Mouse(mouse) => {
                self.handle_coordinator_event(CoordinatorEvent::Mouse(mouse))?
            }
            RunnerEvent::Tick(tick) => self.handle_tick(tick)?,
            RunnerEvent::Resize(resize) => {
                self.handle_coordinator_event(CoordinatorEvent::Resize(resize))?
            }
        };

        Ok(action)
    }

    /// Handle coordinator events that are outside the standard input stream.
    pub fn handle_coordinator_event(
        &mut self,
        event: CoordinatorEvent,
    ) -> LayoutResult<RunnerAction> {
        let action = self.coordinator.handle_event(event)?;
        Ok(self.normalize_action(action))
    }

    /// Handle a tick event and update the tick counter.
    pub fn handle_tick(&mut self, tick: TickEvent) -> LayoutResult<RunnerAction> {
        self.tick_count = tick.count;
        self.handle_coordinator_event(CoordinatorEvent::Tick(tick.count))
    }

    /// Register a new element with the runtime.
    pub fn register_element(
        &mut self,
        metadata: ElementMetadata,
        element: Arc<dyn Element>,
    ) -> LayoutResult<RunnerAction> {
        self.handle_coordinator_event(CoordinatorEvent::Register(metadata, element))
    }

    /// Remove an element from the runtime.
    pub fn unregister_element(&mut self, id: ElementId) -> LayoutResult<RunnerAction> {
        self.handle_coordinator_event(CoordinatorEvent::Unregister(id))
    }

    /// Update element visibility.
    pub fn set_visibility(
        &mut self,
        id: ElementId,
        visibility: Visibility,
    ) -> LayoutResult<RunnerAction> {
        self.handle_coordinator_event(CoordinatorEvent::SetVisibility(id, visibility))
    }

    /// Apply a focus change request.
    pub fn request_focus(&mut self, request: FocusRequest) -> LayoutResult<RunnerAction> {
        self.handle_coordinator_event(CoordinatorEvent::Focus(request))
    }

    /// Whether the runner should redraw based on dirty state.
    pub fn needs_redraw(&self) -> bool {
        self.coordinator.is_dirty() || self.coordinator.layout().is_dirty()
    }

    /// Render all visible elements and clear layout dirty flags.
    pub fn render(&mut self, frame: &mut Frame) -> LayoutResult<()> {
        self.render_visible_elements();
        self.coordinator.app_mut().on_draw(frame);
        self.coordinator.clear_dirty();
        Ok(())
    }

    fn render_visible_elements(&self) {
        let layout = self.coordinator.layout();
        let registry = layout.registry();

        for id in registry.all_ids() {
            let metadata = match registry.get_metadata(id) {
                Ok(metadata) => metadata,
                Err(_) => continue,
            };

            if !metadata.is_visible() {
                continue;
            }

            if let Ok(element) = registry.get_strong_ref(id) {
                element.on_render();
            }
        }
    }

    fn normalize_action(&self, action: CoordinatorAction) -> RunnerAction {
        match action {
            CoordinatorAction::Quit => RunnerAction::Quit,
            CoordinatorAction::Redraw => RunnerAction::Redraw,
            CoordinatorAction::Continue => {
                if self.needs_redraw() {
                    RunnerAction::Redraw
                } else {
                    RunnerAction::Continue
                }
            }
        }
    }
}

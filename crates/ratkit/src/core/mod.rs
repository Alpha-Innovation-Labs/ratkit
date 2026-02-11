//! Core runtime types and runner.

pub mod runner;

pub use crate::{
    coordinator::{
        CoordinatorAction, CoordinatorApp, CoordinatorConfig, CoordinatorEvent, LayoutCoordinator,
    },
    error::{LayoutError, LayoutResult},
    events::{KeyboardEvent, MouseEvent, ResizeEvent, TickEvent},
    focus::FocusRequest,
    mouse_router::MouseRouterConfig,
    redraw_signal::RedrawSignal,
    registry::{Element, ElementHandle},
    types::{ElementId, ElementMetadata, Visibility},
};
pub use runner::{Runner, RunnerAction, RunnerConfig, RunnerEvent};

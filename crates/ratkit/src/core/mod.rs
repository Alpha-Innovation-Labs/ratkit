//! Core runtime types and runner.

pub mod runner;

pub use crate::{
    coordinator::{
        CoordinatorAction, CoordinatorApp, CoordinatorConfig, CoordinatorEvent, LayoutCoordinator,
    },
    error::{LayoutError, LayoutResult},
    events::{KeyboardEvent, MouseEvent, ResizeEvent, TickEvent},
    focus::{FocusManager, FocusRequest},
    layout::{LayoutManager, LayoutStats},
    mouse_router::{MouseRouter, MouseRouterConfig},
    registry::{Element, ElementHandle, ElementRegistry},
    types::{ElementId, ElementMetadata, Region, Visibility},
};
pub use runner::{Runner, RunnerAction, RunnerConfig, RunnerEvent};

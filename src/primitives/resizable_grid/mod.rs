//! ResizableGrid primitive - unified split pane layout with mouse interaction.
//!
//! This module consolidates the former `split_layout`, `resizable_split`,
//! and `widgets::split_layout` modules into a single feature.
//!
//! # Example
//!
//! ```rust
//! use ratatui_toolkit::primitives::resizable_grid::ResizableGrid;
//!
//! let grid = ResizableGrid::new(0);
//! ```

pub mod builders;
pub mod layout;
pub mod operations;
pub mod types;
pub mod widget;

pub use layout::PaneLayout;
pub use types::{PaneId, PaneInfo, ResizableGrid, SplitAreas, SplitAxis, SplitDividerLayout};

pub use widget::{ResizableGridWidget, ResizableGridWidgetState};

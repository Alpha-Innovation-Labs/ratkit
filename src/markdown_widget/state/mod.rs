//! State management for markdown widget.
//!
//! Contains all state modules for the markdown widget including:
//! - `MarkdownScrollManager` - Scroll state and caching
//! - `TocState` - Table of Contents state
//! - `SelectionState` - Text selection state
//! - `DoubleClickState` - Double-click detection state

pub mod double_click_state;
pub mod scroll_manager;
pub mod selection_state;
pub mod toc_state;

pub use double_click_state::DoubleClickState;
pub use scroll_manager::{ExpandableState, MarkdownScrollManager, ParsedCache, RenderCache};
pub use selection_state::SelectionState;
pub use toc_state::{TocEntry, TocState};

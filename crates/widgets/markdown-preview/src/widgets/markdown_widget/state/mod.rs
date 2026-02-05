//! State management for markdown widget.
//!
//! Contains all state modules for the markdown widget including:
//! - `MarkdownState` - Unified state (bundles all below)
//! - `ScrollState` - Core scroll position and viewport
//! - `SourceState` - Content source management
//! - `CacheState` - Parsed and render caching
//! - `DisplaySettings` - Display configuration
//! - `CollapseState` - Section collapse tracking
//! - `ExpandableState` - Expandable content state
//! - `GitStatsState` - Git integration
//! - `VimState` - Vim keybinding state
//! - `TocState` - Table of Contents state
//! - `SelectionState` - Text selection state
//! - `DoubleClickState` - Double-click detection state

// Focused state modules
pub mod cache_state;
pub mod collapse_state;
pub mod display_settings;
pub mod double_click_state;
pub mod expandable_state;
pub mod git_stats_state;
pub mod markdown_state;
pub mod scroll_state;
pub mod selection_state;
pub mod source_state;
pub mod toc_state;
pub mod vim_state;

// State exports
pub use cache_state::{CacheState, ParsedCache, RenderCache};
pub use collapse_state::CollapseState;
pub use display_settings::DisplaySettings;
pub use double_click_state::DoubleClickState;
pub use expandable_state::{ExpandableEntry, ExpandableState};
pub use git_stats_state::GitStatsState;
pub use markdown_state::MarkdownState;
pub use scroll_state::ScrollState;
pub use selection_state::SelectionState;
pub use source_state::SourceState;
pub use toc_state::{TocEntry, TocState};
pub use vim_state::VimState;

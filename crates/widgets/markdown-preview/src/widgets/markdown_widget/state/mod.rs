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
pub mod cache;
pub mod collapse;
pub mod display_settings;
pub mod double_click;
pub mod expandable;
pub mod git_stats;
pub mod markdown;
pub mod scroll;
pub mod selection;
pub mod source;
pub mod toc;
pub mod vim;

// State exports
pub use cache::{CacheState, ParsedCache, RenderCache};
pub use collapse::CollapseState;
pub use display_settings::DisplaySettings;
pub use double_click::DoubleClickState;
pub use expandable::{ExpandableEntry, ExpandableState};
pub use git_stats::GitStatsState;
pub use markdown::MarkdownState;
pub use scroll::ScrollState;
pub use selection::SelectionState;
pub use source::SourceState;
pub use toc::{TocEntry, TocState};
pub use vim::VimState;

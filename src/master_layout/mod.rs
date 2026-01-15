//! Master Layout Framework
//!
//! A comprehensive TUI framework providing vim-like modal interaction for building
//! terminal applications with tabs, panes, and keyboard/mouse navigation.
//!
//! # Architecture
//!
//! - **MasterLayout**: Top-level container managing tabs, navigation bar, and modes
//! - **Tab**: Container for panes within a tab
//! - **PaneContainer**: Manages panes, handles selection and focus
//! - **Pane**: Individual content area implementing PaneContent trait
//!
//! # Modes
//!
//! - **Layout Mode** (Command Mode): Navigate panes with hjkl, select with Enter
//! - **Focus Mode** (Insert Mode): Interact with focused pane, exit with Ctrl-A

mod footer;
mod interaction_mode;
mod layout;
mod layout_manager;
mod navigation_bar;
mod pane;
mod pane_container;
mod pane_id;
mod tab;

pub use footer::{Footer, FooterItem};
pub use interaction_mode::InteractionMode;
pub use layout::PaneLayout;
pub use layout_manager::{EventResult, MasterLayout};
pub use navigation_bar::{NavigationBar, TabButton};
pub use pane::{Pane, PaneContent};
pub use pane_container::PaneContainer;
pub use pane_id::PaneId;
pub use tab::Tab;

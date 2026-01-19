//! Table of Contents widget for markdown document navigation.
//!
//! Provides a compact or expanded view of document headings for quick navigation.
//! In compact mode, shows heading indicators as horizontal lines.
//! In expanded mode, shows full heading text with indentation.
//!
//! # Features
//!
//! - Compact mode: horizontal lines indicating heading positions and levels
//! - Expanded mode: full heading text with hierarchy indentation
//! - Current heading highlight (blue in expanded, bright in compact)
//! - Hover highlight for items
//! - Click-to-scroll navigation
//!
//! # Mouse Capture Requirement
//!
//! For TOC click navigation and hover interactions to work, you must enable
//! mouse capture with crossterm:
//!
//! ```rust,ignore
//! use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
//! execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//! ```
//!
//! Without `EnableMouseCapture`, click events will not be received.
//!
//! # Architecture
//!
//! The Toc extension is a UI widget only - it receives `&TocState` as a parameter
//! and ONLY handles rendering. State mutations happen through TocState methods.

pub mod constructors;
pub mod enums;
mod methods;
mod traits;

pub use constructors::*;
pub use enums::TocConfig;
pub use enums::*;

use crate::markdown_widget::state::toc_state::TocState;

/// Table of Contents widget for markdown navigation.
///
/// Shows document headings in either compact (lines) or expanded (text) mode.
/// Supports hover interactions and click-to-scroll navigation.
///
/// This is a UI-only widget that receives `&TocState` for state access.
/// State mutations happen through `TocState` methods, not here.
///
/// # Example
///
/// ```rust,no_run
/// use ratatui_toolkit::markdown_widget::extensions::toc::{Toc, TocConfig};
/// use ratatui_toolkit::markdown_widget::state::toc_state::TocState;
///
/// let mut toc_state = TocState::new();
/// let toc = Toc::new(&toc_state)
///     .config(TocConfig::default())
///     .expanded(true);
/// ```
#[derive(Debug)]
pub struct Toc<'a> {
    /// Reference to the TOC state (entries, scroll, hover).
    pub(crate) toc_state: &'a TocState,
    /// Configuration for appearance.
    pub(crate) config: TocConfig,
    /// Whether the TOC is in expanded mode.
    pub(crate) expanded: bool,
}

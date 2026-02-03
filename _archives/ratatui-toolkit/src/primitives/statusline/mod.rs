//! A status-line widget that can stack up indicators
//! on the left and right end.
//!
//! If you use the constants SLANT_TL_BR and SLANT_BL_TR as
//! separator you can do neo-vim/neovim style statusline.
//!
//! This is adapted from rat-widget's StatusLineStacked.
//!
//! # Example
//!
//! ```rust,no_run
//! use ratatui::style::{Color, Style};
//! use ratatui::text::Span;
//! use ratatui_toolkit::statusline_stacked::{StatusLineStacked, SLANT_BL_TR, SLANT_TL_BR};
//!
//! StatusLineStacked::new()
//!     .start(
//!         Span::from(" STATUS ").style(Style::new().fg(Color::Black).bg(Color::DarkGray)),
//!         Span::from(SLANT_TL_BR).style(Style::new().fg(Color::DarkGray).bg(Color::Green)),
//!     )
//!     .start(
//!         Span::from(" OPERATIONAL ").style(Style::new().fg(Color::Black).bg(Color::Green)),
//!         Span::from(SLANT_TL_BR).style(Style::new().fg(Color::Green)),
//!     )
//!     .center("Some status message...")
//!     .end(
//!         Span::from(" INFO ").style(Style::new().fg(Color::Black).bg(Color::Cyan)),
//!         Span::from(SLANT_BL_TR).style(Style::new().fg(Color::Cyan)),
//!     );
//! ```

pub mod constructors;
pub mod methods;
pub mod traits;

use ratatui::style::Style;
use ratatui::text::Line;
use std::marker::PhantomData;

/// PowerLine block cut at the diagonal (top-left to bottom-right).
/// Requires a Nerd Font or PowerLine font.
pub const SLANT_TL_BR: &str = "\u{e0b8}";

/// PowerLine block cut at the diagonal (bottom-left to top-right).
/// Requires a Nerd Font or PowerLine font.
pub const SLANT_BL_TR: &str = "\u{e0ba}";

/// Statusline with stacked indicators on the left and right side.
///
/// This widget creates a statusline with a "stacked" appearance using
/// PowerLine-style diagonal separators. It has three sections:
/// - Left: Stack indicators from left to right
/// - Center: Centered status message
/// - Right: Stack indicators from right to left
#[derive(Debug, Clone)]
pub struct StatusLineStacked<'a> {
    style: Style,
    left: Vec<(Line<'a>, Line<'a>)>,
    center_margin: u16,
    center: Line<'a>,
    right: Vec<(Line<'a>, Line<'a>)>,
    phantom: PhantomData<&'a ()>,
}

/// Operational mode for styled statusline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OperationalMode {
    /// Normal operation (green)
    #[default]
    Operational,
    /// Warning state (yellow)
    Dire,
    /// Critical state (red)
    Evacuate,
}

/// Builder for creating a styled statusline that looks like rat-salsa's menu_status2.
///
/// This provides a pre-configured statusline with the "Westinghouse" reactor-control aesthetic.
pub struct StyledStatusLine<'a> {
    mode: OperationalMode,
    title: &'a str,
    center_text: String,
    render_count: usize,
    event_count: usize,
    render_time_us: u64,
    event_time_us: u64,
    message_count: u32,
    use_slants: bool,
}

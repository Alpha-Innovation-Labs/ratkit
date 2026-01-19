//! Vim keybinding state for markdown widget.
//!
//! Tracks state for vim-style keyboard navigation.

pub mod constructors;
pub mod methods;
pub mod traits;

pub use constructors::*;
pub use methods::*;
pub use traits::*;

use std::time::Instant;

/// Vim keybinding state.
///
/// Tracks pending keypresses for vim-style multi-key commands.
#[derive(Debug, Clone)]
pub struct VimState {
    /// Pending 'g' keypress time for vim-style gg (go to top).
    pending_g_time: Option<Instant>,
}

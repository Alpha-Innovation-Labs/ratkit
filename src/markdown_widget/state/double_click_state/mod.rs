//! State for tracking double-click detection with deferred single-click handling.

pub mod constructors;
pub mod methods;
pub mod traits;

use std::time::Instant;

/// State for tracking double-click detection with deferred single-click handling.
#[derive(Debug, Clone)]
pub struct DoubleClickState {
    /// Time of the last click.
    pub(crate) last_click_time: Option<Instant>,
    /// Position of the last click.
    pub(crate) last_click_pos: Option<(u16, u16)>,
    /// Pending single-click that hasn't been processed yet.
    /// Stores: (x, y, timestamp, scroll_offset_at_click_time)
    pub(crate) pending_single_click: Option<(u16, u16, Instant, usize)>,
}

impl DoubleClickState {
    /// Double-click time threshold in milliseconds.
    pub(crate) const DOUBLE_CLICK_THRESHOLD_MS: u64 = 150;
}

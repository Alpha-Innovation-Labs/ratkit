//! Cross-thread redraw invalidation primitive.
//!
//! Use this when your app receives updates from background work (PTY output,
//! network I/O, file watchers) and you want to avoid redrawing every tick.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// A cloneable signal for requesting a redraw from any thread.
#[derive(Debug, Clone, Default)]
pub struct RedrawSignal {
    dirty: Arc<AtomicBool>,
}

impl RedrawSignal {
    /// Create a new signal.
    ///
    /// Starts in the clean state (no redraw requested).
    pub fn new() -> Self {
        Self::default()
    }

    /// Request a redraw.
    pub fn request_redraw(&self) {
        self.dirty.store(true, Ordering::Release);
    }

    /// Returns true if a redraw is currently requested.
    pub fn is_redraw_requested(&self) -> bool {
        self.dirty.load(Ordering::Acquire)
    }

    /// Consume and clear the redraw request flag.
    ///
    /// Returns true if a redraw had been requested since the last consume.
    pub fn take_redraw_request(&self) -> bool {
        self.dirty.swap(false, Ordering::AcqRel)
    }
}

#[cfg(test)]
mod tests {
    use super::RedrawSignal;

    #[test]
    fn redraw_signal_is_clean_by_default() {
        let signal = RedrawSignal::new();
        assert!(!signal.is_redraw_requested());
    }

    #[test]
    fn redraw_signal_can_be_requested_and_taken() {
        let signal = RedrawSignal::new();
        signal.request_redraw();
        assert!(signal.is_redraw_requested());
        assert!(signal.take_redraw_request());
        assert!(!signal.is_redraw_requested());
        assert!(!signal.take_redraw_request());
    }
}

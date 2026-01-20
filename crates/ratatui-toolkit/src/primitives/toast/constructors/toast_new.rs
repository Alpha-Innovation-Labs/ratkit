use crate::primitives::toast::{Toast, ToastLevel, DEFAULT_TOAST_DURATION};
use std::time::{Duration, Instant};

impl Toast {
    /// Create a new toast with an optional duration.
    ///
    /// If `duration` is `None`, uses the default duration (5 seconds).
    ///
    /// # Arguments
    ///
    /// * `message` - The toast message
    /// * `level` - The toast level (Success, Error, Info, Warning)
    /// * `duration` - Optional duration, defaults to 5 seconds if None
    pub fn new(message: impl Into<String>, level: ToastLevel, duration: Option<Duration>) -> Self {
        Self {
            message: message.into(),
            level,
            created_at: Instant::now(),
            duration: duration.unwrap_or(DEFAULT_TOAST_DURATION),
        }
    }
}

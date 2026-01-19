use crate::toast::{Toast, ToastLevel};
use std::time::Duration;

impl Toast {
    pub fn with_duration(
        message: impl Into<String>,
        level: ToastLevel,
        duration: Duration,
    ) -> Self {
        Self {
            message: message.into(),
            level,
            created_at: std::time::Instant::now(),
            duration,
        }
    }
}

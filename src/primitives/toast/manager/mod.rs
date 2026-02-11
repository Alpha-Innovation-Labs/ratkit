use crate::primitives::toast::{Toast, ToastLevel, ToastManager};
use ratatui::layout::Rect;
use std::time::Duration;

impl Toast {
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }

    pub fn new(message: impl Into<String>, level: ToastLevel, duration: Option<Duration>) -> Self {
        Self {
            message: message.into(),
            level,
            created_at: std::time::Instant::now(),
            duration: duration.unwrap_or(super::DEFAULT_TOAST_DURATION),
        }
    }

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

    pub fn lifetime_percent(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_secs_f32();
        let total = self.duration.as_secs_f32();
        (total - elapsed) / total
    }
}

impl ToastManager {
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            max_toasts: 5,
        }
    }

    pub fn add(&mut self, toast: Toast) {
        self.remove_expired();

        self.toasts.push(toast);

        if self.toasts.len() > self.max_toasts {
            self.toasts.drain(0..self.toasts.len() - self.max_toasts);
        }

        tracing::debug!("Toast added, total toasts: {}", self.toasts.len());
    }

    pub fn clear(&mut self) {
        self.toasts.clear();
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Error, None));
    }

    pub fn get_active(&self) -> &[Toast] {
        &self.toasts
    }

    pub fn handle_click(&mut self, x: u16, y: u16, frame_area: Rect) -> bool {
        const TOAST_WIDTH: u16 = 40;
        const TOAST_HEIGHT: u16 = 3;
        const TOAST_MARGIN: u16 = 2;
        const TOAST_SPACING: u16 = 1;

        let active_count = self.toasts.iter().filter(|t| !t.is_expired()).count();
        if active_count == 0 {
            return false;
        }

        let mut y_offset = frame_area.height.saturating_sub(TOAST_MARGIN);

        for i in (0..self.toasts.len()).rev() {
            if self.toasts[i].is_expired() {
                continue;
            }

            let toast_y = y_offset.saturating_sub(TOAST_HEIGHT);
            let toast_x = frame_area.width.saturating_sub(TOAST_WIDTH + TOAST_MARGIN);

            if x >= toast_x
                && x < toast_x + TOAST_WIDTH
                && y >= toast_y
                && y < toast_y + TOAST_HEIGHT
            {
                self.toasts.remove(i);
                return true;
            }

            y_offset = toast_y.saturating_sub(TOAST_SPACING);

            if toast_y == 0 || toast_x == 0 {
                break;
            }
        }

        false
    }

    pub fn has_toasts(&self) -> bool {
        !self.toasts.is_empty()
    }

    pub fn info(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Info, None));
    }

    pub fn remove_expired(&mut self) {
        let before = self.toasts.len();
        self.toasts.retain(|toast| !toast.is_expired());
        let removed = before - self.toasts.len();
        if removed > 0 {
            tracing::debug!("Removed {} expired toasts", removed);
        }
    }

    pub fn success(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Success, None));
    }

    pub fn warning(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Warning, None));
    }
}

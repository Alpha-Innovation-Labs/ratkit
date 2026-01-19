use crate::toast::{Toast, ToastManager};

impl ToastManager {
    pub fn add(&mut self, toast: Toast) {
        self.remove_expired();

        self.toasts.push(toast);

        if self.toasts.len() > self.max_toasts {
            self.toasts.drain(0..self.toasts.len() - self.max_toasts);
        }

        tracing::debug!("Toast added, total toasts: {}", self.toasts.len());
    }
}

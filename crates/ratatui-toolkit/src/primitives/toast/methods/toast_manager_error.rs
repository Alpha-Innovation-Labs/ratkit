use crate::primitives::toast::{Toast, ToastLevel, ToastManager};

impl ToastManager {
    pub fn error(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Error, None));
    }
}

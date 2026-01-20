use crate::primitives::toast::{Toast, ToastLevel, ToastManager};

impl ToastManager {
    pub fn success(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Success, None));
    }
}

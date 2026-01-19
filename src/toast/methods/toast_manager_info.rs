use crate::toast::{Toast, ToastLevel, ToastManager};

impl ToastManager {
    pub fn info(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Info, None));
    }
}

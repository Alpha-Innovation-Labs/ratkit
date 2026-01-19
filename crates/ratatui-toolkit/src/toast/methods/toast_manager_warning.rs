use crate::toast::{Toast, ToastLevel, ToastManager};

impl ToastManager {
    pub fn warning(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Warning, None));
    }
}

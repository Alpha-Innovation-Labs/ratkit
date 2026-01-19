use crate::toast::ToastManager;

impl ToastManager {
    pub fn clear(&mut self) {
        self.toasts.clear();
    }
}

use crate::toast::ToastManager;

impl ToastManager {
    pub fn get_active(&self) -> &[crate::toast::Toast] {
        &self.toasts
    }
}

use crate::primitives::toast::ToastManager;

impl ToastManager {
    pub fn get_active(&self) -> &[crate::primitives::toast::Toast] {
        &self.toasts
    }
}

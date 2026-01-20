use crate::primitives::toast::ToastManager;

impl ToastManager {
    pub fn has_toasts(&self) -> bool {
        !self.toasts.is_empty()
    }
}

use crate::toast::ToastManager;

impl ToastManager {
    pub fn remove_expired(&mut self) {
        let before = self.toasts.len();
        self.toasts.retain(|toast| !toast.is_expired());
        let removed = before - self.toasts.len();
        if removed > 0 {
            tracing::debug!("Removed {} expired toasts", removed);
        }
    }
}

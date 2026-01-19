use crate::toast::Toast;

impl Toast {
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }
}

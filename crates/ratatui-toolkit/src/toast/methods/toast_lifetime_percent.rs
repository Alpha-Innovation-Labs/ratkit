use crate::toast::Toast;

impl Toast {
    pub fn lifetime_percent(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_secs_f32();
        let total = self.duration.as_secs_f32();
        (total - elapsed) / total
    }
}

#[derive(Clone, Debug)]
pub struct HotkeyItem {
    pub key: String,
    pub description: String,
}

impl HotkeyItem {
    pub fn new(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
}

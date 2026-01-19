use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn with_loading_message(mut self, message: impl Into<String>) -> Self {
        self.loading_message = message.into();
        self
    }
}

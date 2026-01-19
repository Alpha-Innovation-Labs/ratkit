use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn with_size(mut self, percent_width: u16, percent_height: u16) -> Self {
        self.size_percent = (percent_width, percent_height);
        self
    }
}

use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn is_running(&self) -> bool {
        self.terminal.is_some()
    }
}

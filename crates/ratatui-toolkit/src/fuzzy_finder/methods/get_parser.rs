use crate::fuzzy_finder::FuzzyFinder;
use crate::primitives::termtui::Parser;
use std::sync::{Arc, Mutex};

impl FuzzyFinder {
    pub fn get_parser(&self) -> Option<Arc<Mutex<Parser>>> {
        self.terminal.as_ref().map(|t| Arc::clone(&t.parser))
    }
}

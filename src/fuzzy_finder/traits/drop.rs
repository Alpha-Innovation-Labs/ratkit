use crate::fuzzy_finder::FuzzyFinder;

impl Drop for FuzzyFinder {
    fn drop(&mut self) {
        if let Some(ref terminal) = self.terminal {
            let mut child = terminal.child.lock().unwrap();
            let _ = child.kill();
        }
    }
}

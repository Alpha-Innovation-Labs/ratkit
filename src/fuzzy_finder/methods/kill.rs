use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn kill(&mut self) -> std::io::Result<()> {
        if let Some(ref terminal) = self.terminal {
            let mut child = terminal.child.lock().unwrap();
            child.kill()?;
        }
        Ok(())
    }
}

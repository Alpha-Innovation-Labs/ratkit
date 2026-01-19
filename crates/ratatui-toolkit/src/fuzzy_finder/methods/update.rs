use anyhow::Result;
use std::io::Read;

use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn update(&mut self) -> Result<()> {
        if let Some(ref terminal) = self.terminal {
            let mut reader = terminal.reader.lock().unwrap();
            let mut parser = terminal.parser.lock().unwrap();

            let mut buf = [0u8; 1024];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        parser.process(&buf[..n]);
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(_) => break,
                }
            }
        }
        Ok(())
    }
}

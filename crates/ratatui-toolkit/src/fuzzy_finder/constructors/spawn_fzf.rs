use crate::primitives::termtui::Parser;
use anyhow::{Context, Result};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::sync::{Arc, Mutex};

use crate::fuzzy_finder::{FuzzyFinder, FuzzyFinderTerminal};

impl FuzzyFinder {
    pub fn spawn_fzf(
        &mut self,
        items: Vec<String>,
        rows: u16,
        cols: u16,
        prompt: Option<&str>,
    ) -> Result<()> {
        if items.is_empty() {
            return Err(anyhow::anyhow!("Cannot spawn fzf with empty item list"));
        }

        if rows == 0 || cols == 0 {
            return Err(anyhow::anyhow!("Invalid terminal size: {}x{}", rows, cols));
        }

        let pty_system = native_pty_system();

        let pty_size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };

        tracing::debug!("Opening PTY with size {}x{}", rows, cols);
        let pair = pty_system
            .openpty(pty_size)
            .context("Failed to allocate PTY")?;

        let mut cmd = CommandBuilder::new("fzf");
        cmd.arg("--prompt");
        cmd.arg(prompt.unwrap_or("Select: "));
        cmd.arg("--height");
        cmd.arg("100%");
        cmd.arg("--layout");
        cmd.arg("reverse");
        cmd.arg("--info");
        cmd.arg("inline");
        cmd.arg("--ansi");

        tracing::debug!("Spawning fzf command");
        let child = pair
            .slave
            .spawn_command(cmd)
            .context("Failed to spawn fzf process")?;
        tracing::debug!("fzf process spawned successfully");

        #[cfg(unix)]
        {
            if let Some(fd) = pair.master.as_raw_fd() {
                unsafe {
                    let flags = libc::fcntl(fd, libc::F_GETFL, 0);
                    libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
                }
            }
        }

        let reader = pair.master.try_clone_reader()?;
        let mut writer = pair.master.take_writer()?;

        let items_str = items.join("\n") + "\n";
        writer.write_all(items_str.as_bytes())?;
        writer.flush()?;

        let parser = Arc::new(Mutex::new(Parser::new(rows as usize, cols as usize, 0)));

        self.terminal = Some(FuzzyFinderTerminal {
            parser,
            _master: Arc::new(Mutex::new(pair.master)),
            child: Arc::new(Mutex::new(child)),
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
        });

        Ok(())
    }
}

use std::io::{Read, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};

#[test]
fn button_demo_exits_on_q_portable_pty() -> Result<()> {
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let mut cmd = CommandBuilder::new("cargo");
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_dir
        .join("../../../")
        .canonicalize()
        .expect("workspace root");
    cmd.cwd(workspace_root);
    cmd.arg("run");
    cmd.arg("-p");
    cmd.arg("ratkit-button");
    cmd.arg("--example");
    cmd.arg("button_demo");

    let mut child = pair.slave.spawn_command(cmd)?;

    let mut reader = pair.master.try_clone_reader()?;
    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(_) => {}
                Err(_) => break,
            }
        }
    });

    let mut writer = pair.master.take_writer()?;

    thread::sleep(Duration::from_millis(500));
    writer.write_all(b"q")?;
    writer.flush()?;

    for _ in 0..40 {
        if child.try_wait()?.is_some() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }

    child.kill()?;
    child.wait()?;

    Err(anyhow!("button demo did not terminate after 'q'"))
}

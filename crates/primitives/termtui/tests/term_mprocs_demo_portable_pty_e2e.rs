use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};

fn wait_for_output(buffer: &Arc<Mutex<Vec<u8>>>, timeout: Duration) {
    let start = Instant::now();
    loop {
        if let Ok(data) = buffer.lock() {
            if !data.is_empty() {
                break;
            }
        }
        if start.elapsed() > timeout {
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }
}

fn screen_text(buffer: &Arc<Mutex<Vec<u8>>>) -> Result<String> {
    let data = buffer.lock().unwrap();
    let stripped = strip_ansi_escapes::strip(&*data);
    Ok(String::from_utf8_lossy(&stripped).to_string())
}

fn wait_for_any_entry(
    buffer: &Arc<Mutex<Vec<u8>>>,
    entries: &[String],
    timeout: Duration,
) -> Result<()> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        let screen = screen_text(buffer)?;
        if entries
            .iter()
            .any(|entry| !entry.is_empty() && screen.contains(entry))
        {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }
    let screen = screen_text(buffer)?;
    Err(anyhow!(
        "no yazi entries found in terminal output: {screen}"
    ))
}

#[test]
fn term_mprocs_demo_renders_yazi_portable_pty() -> Result<()> {
    let pty_system = native_pty_system();
    let pty_size = PtySize {
        rows: 40,
        cols: 120,
        pixel_width: 0,
        pixel_height: 0,
    };
    let pair = pty_system.openpty(pty_size)?;

    let mut cmd = CommandBuilder::new("cargo");
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_dir
        .join("../../../")
        .canonicalize()
        .expect("workspace root");
    cmd.cwd(workspace_root);
    cmd.arg("run");
    cmd.arg("-p");
    cmd.arg("ratkit-termtui");
    cmd.arg("--example");
    cmd.arg("term_mprocs_demo");

    let mut child = pair.slave.spawn_command(cmd)?;

    let buffer = Arc::new(Mutex::new(Vec::new()));
    let buffer_reader = Arc::clone(&buffer);
    let mut reader = pair.master.try_clone_reader()?;
    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if let Ok(mut data) = buffer_reader.lock() {
                        data.extend_from_slice(&buf[..n]);
                    }
                }
                Err(_) => break,
            }
        }
    });

    let mut writer = pair.master.take_writer()?;

    wait_for_output(&buffer, Duration::from_secs(2));

    writer.write_all(b"yazi\n")?;
    writer.flush()?;
    thread::sleep(Duration::from_millis(800));

    let expected = Command::new("bash").arg("-lc").arg("ls -1").output()?;
    let expected_stdout = String::from_utf8_lossy(&expected.stdout);
    let entries: Vec<String> = expected_stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    wait_for_any_entry(&buffer, &entries, Duration::from_secs(3))?;

    writer.write_all(b"q")?;
    writer.flush()?;
    thread::sleep(Duration::from_millis(500));

    writer.write_all(b"\x18")?;
    writer.flush()?;
    thread::sleep(Duration::from_millis(200));
    writer.write_all(b"q")?;
    writer.flush()?;

    for _ in 0..60 {
        if child.try_wait()?.is_some() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }

    child.kill()?;
    child.wait()?;

    Err(anyhow!(
        "termtui demo did not terminate after Ctrl+X then q"
    ))
}

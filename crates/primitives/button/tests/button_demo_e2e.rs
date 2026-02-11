use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use portable_pty::CommandBuilder;
use ratatui_testlib::{KeyCode, Modifiers, TuiTestHarness};

#[test]
fn button_demo_does_not_exit_on_q() -> ratatui_testlib::Result<()> {
    let mut harness = TuiTestHarness::builder()
        .with_size(80, 24)
        .with_timeout(Duration::from_secs(2))
        .with_poll_interval(Duration::from_millis(50))
        .build()?;

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

    harness.spawn(cmd)?;

    thread::sleep(Duration::from_millis(300));

    harness.send_key(KeyCode::Char('q'))?;

    for _ in 0..30 {
        if !harness.is_running() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }

    harness.send_key_with_modifiers(KeyCode::Char('c'), Modifiers::CTRL)?;
    panic!("Button demo did not terminate after 'q'");
}

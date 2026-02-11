use std::time::Duration;

use termwright::prelude::*;

#[tokio::test]
async fn button_demo_does_not_exit_on_q_termwright() -> termwright::Result<()> {
    let term = Terminal::builder()
        .size(80, 24)
        .spawn(
            "cargo",
            &["run", "-p", "ratkit-button", "--example", "button_demo"],
        )
        .await?;

    tokio::time::sleep(Duration::from_millis(300)).await;

    term.send_key(Key::Char('q')).await?;

    let exit_result = tokio::time::timeout(Duration::from_secs(2), term.wait_exit()).await;
    if let Ok(Ok(_status)) = exit_result {
        return Ok(());
    }

    term.send_key(Key::Ctrl('c')).await?;
    term.wait_exit().await?;
    panic!("Button demo did not terminate after 'q'");

    Ok(())
}

//! Standalone Code Diff Widget Demo
//!
//! Run with: cargo run --example codediff_demo --features full

use std::io;

use crossterm::{
    cursor::Show,
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, layout::Rect, widgets::Widget, Terminal};
use ratatui_toolkit::{
    services::theme::loader::load_builtin_theme, AppTheme, CodeDiff, ThemeVariant,
};

struct AppState {
    diff: CodeDiff,
    theme: AppTheme,
}

const SAMPLE_DIFF: &str = r#"--- a/src/main.rs
+++ b/src/main.rs
@@ -1,10 +1,15 @@
+use std::io;
+
 fn main() {
-    println!("Hello, World!");
+    println!("Hello, Ratatui!");
+    println!("Welcome to the code diff demo!");
 }
+
+fn new_function() {
+    println!("This is a new function!");
+}
"#;

fn main() -> io::Result<()> {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        let _ = execute!(io::stdout(), Show);
        original_hook(panic_info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut diff = CodeDiff::from_unified_diff(SAMPLE_DIFF);

    let app = AppState {
        diff,
        theme: load_builtin_theme("ayu", ThemeVariant::Dark).unwrap_or_default(),
    };

    let result = run_demo(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_demo(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: AppState,
) -> io::Result<()> {
    let mut render_area = Rect::default();

    loop {
        terminal.draw(|frame| {
            render_area = frame.area();
            (&app.diff).render(render_area, frame.buffer_mut());
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                    app.diff.handle_key(key.code);
                }
                Event::Mouse(mouse) => {
                    app.diff.handle_mouse(mouse, render_area);
                }
                _ => {}
            }
        }
    }
}

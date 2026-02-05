use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{widgets::Block, Frame};
use ratkit_code_diff::CodeDiff;
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};

struct CodeDiffDemo {
    diff: CodeDiff,
}

impl CodeDiffDemo {
    fn new() -> Self {
        let diff = CodeDiff::from_unified_diff("@@ -1,3 +1,3 @@\n-old line\n+new line\n unchanged")
            .with_file_path("src/lib.rs");
        Self { diff }
    }
}

impl App for CodeDiffDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Crossterm(Event::Key(key))
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') =>
            {
                Ok(RunnerAction::Quit)
            }
            _ => Ok(RunnerAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::default().title(" Code Diff ");
        let inner = block.inner(area);
        frame.render_widget(block, area);
        frame.render_widget(self.diff.clone(), inner);
    }
}

fn main() -> io::Result<()> {
    let mut app = CodeDiffDemo::new();
    run(&mut app, RunConfig::default())
}

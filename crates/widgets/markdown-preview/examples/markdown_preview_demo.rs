use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{widgets::Block, Frame};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_markdown_preview::{MarkdownState, MarkdownWidget};

struct MarkdownPreviewDemo {
    state: MarkdownState,
}

impl MarkdownPreviewDemo {
    fn new() -> Self {
        let mut state = MarkdownState::default();
        if let Err(err) = state
            .source
            .set_source_file("crates/widgets/markdown-preview/examples/markdown_demo_full.md")
        {
            eprintln!("Failed to load markdown demo file: {err}");
        }

        Self { state }
    }
}

impl App for MarkdownPreviewDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Tick => {
                if self.state.reload_source_if_changed()? {
                    Ok(RunnerAction::Redraw)
                } else {
                    Ok(RunnerAction::Continue)
                }
            }
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
        let block = Block::default().title(" Markdown Preview ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let widget = MarkdownWidget::from_state(&self.state)
            .show_toc(true)
            .show_statusline(true)
            .show_scrollbar(true);
        frame.render_widget(widget, inner);
    }
}

fn main() -> io::Result<()> {
    let mut app = MarkdownPreviewDemo::new();
    run(&mut app, RunConfig::default())
}

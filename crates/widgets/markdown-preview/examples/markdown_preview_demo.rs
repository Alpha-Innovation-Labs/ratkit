use std::io;

use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{widgets::Block, Frame};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, LayoutResult, RunnerConfig,
};
use ratkit_markdown_preview::{MarkdownEvent, MarkdownState, MarkdownWidget};

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

impl CoordinatorApp for MarkdownPreviewDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Tick(_count) => match self.state.reload_source_if_changed() {
                Ok(changed) => {
                    if changed {
                        Ok(CoordinatorAction::Redraw)
                    } else {
                        Ok(CoordinatorAction::Continue)
                    }
                }
                Err(err) => Err(ratkit::LayoutError::event_routing(format!(
                    "markdown reload failed: {err}"
                ))),
            },
            CoordinatorEvent::Resize(resize) => {
                self.state.set_inner_area(resize.area());
                self.state.scroll.viewport_height = resize.height.saturating_sub(2) as usize;
                self.state.scroll.total_lines = self.state.source.line_count();
                Ok(CoordinatorAction::Redraw)
            }
            CoordinatorEvent::Keyboard(event) => {
                if event.is_key_up() {
                    return Ok(CoordinatorAction::Continue);
                }

                if event.is_char('q') {
                    return Ok(CoordinatorAction::Quit);
                }

                let key = KeyEvent::new(event.key_code, event.modifiers);
                let mut widget = MarkdownWidget::from_state(&self.state)
                    .show_toc(true)
                    .show_statusline(true)
                    .show_scrollbar(true);
                let markdown_event = widget.handle_key(key);
                let sync_state = widget.get_state_sync();
                sync_state.apply_to(&mut self.state);

                if matches!(markdown_event, MarkdownEvent::None) {
                    Ok(CoordinatorAction::Continue)
                } else {
                    Ok(CoordinatorAction::Redraw)
                }
            }
            CoordinatorEvent::Mouse(event) => {
                let crossterm_event = MouseEvent {
                    kind: event.kind,
                    column: event.column,
                    row: event.row,
                    modifiers: event.modifiers,
                };

                let area = self.state.inner_area();
                let mut widget = MarkdownWidget::from_state(&self.state)
                    .show_toc(true)
                    .show_statusline(true)
                    .show_scrollbar(true);
                let markdown_event = widget.handle_mouse(crossterm_event, area);
                let sync_state = widget.get_state_sync();
                sync_state.apply_to(&mut self.state);

                if matches!(markdown_event, MarkdownEvent::None) {
                    Ok(CoordinatorAction::Continue)
                } else {
                    Ok(CoordinatorAction::Redraw)
                }
            }
            _ => Ok(CoordinatorAction::Continue),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::default().title(" Markdown Preview ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        self.state.set_inner_area(inner);
        self.state.scroll.viewport_height = inner.height.saturating_sub(1) as usize;
        self.state.scroll.total_lines = self.state.source.line_count();
        let widget = MarkdownWidget::from_state(&self.state)
            .show_toc(true)
            .show_statusline(true)
            .show_scrollbar(true);
        frame.render_widget(widget, inner);
    }
}

fn main() -> io::Result<()> {
    let demo = MarkdownPreviewDemo::new();
    run_with_diagnostics(demo, RunnerConfig::default())
}

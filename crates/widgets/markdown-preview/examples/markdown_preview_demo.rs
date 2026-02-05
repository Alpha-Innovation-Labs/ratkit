use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{widgets::Block, Frame};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
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
                if key.kind != KeyEventKind::Release && key.code == KeyCode::Char('q') =>
            {
                Ok(RunnerAction::Quit)
            }
            RunnerEvent::Resize { .. } => Ok(RunnerAction::Redraw),
            RunnerEvent::Crossterm(Event::Key(key)) if key.kind != KeyEventKind::Release => {
                let (event, state_changed) = self.handle_widget_key(key);
                if state_changed || !matches!(event, MarkdownEvent::None) {
                    Ok(RunnerAction::Redraw)
                } else {
                    Ok(RunnerAction::Continue)
                }
            }
            RunnerEvent::Crossterm(Event::Mouse(mouse)) => {
                if let Some(changed) = self.handle_mouse_scroll(mouse) {
                    return Ok(if changed {
                        RunnerAction::Redraw
                    } else {
                        RunnerAction::Continue
                    });
                }

                let (event, state_changed) = self.handle_widget_mouse(mouse);
                if state_changed || !matches!(event, MarkdownEvent::None) {
                    Ok(RunnerAction::Redraw)
                } else {
                    Ok(RunnerAction::Continue)
                }
            }
            _ => Ok(RunnerAction::Continue),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::default().title(" Markdown Preview ");
        let inner = block.inner(area);
        frame.render_widget(block, area);
        self.state.set_inner_area(inner);
        self.state.scroll.viewport_height = inner.height as usize;
        self.state.scroll.total_lines = self.state.source.line_count();

        let widget = MarkdownWidget::from_state(&self.state)
            .show_toc(true)
            .show_statusline(true)
            .show_scrollbar(true);
        frame.render_widget(widget, inner);
    }
}

impl MarkdownPreviewDemo {
    fn handle_mouse_scroll(&mut self, mouse: crossterm::event::MouseEvent) -> Option<bool> {
        use crossterm::event::MouseEventKind;

        let amount = self.state.display.scroll_multiplier().max(1);
        let before = self.state.scroll.scroll_offset;
        match mouse.kind {
            MouseEventKind::ScrollUp => {
                self.state.scroll.scroll_up(amount);
                Some(self.state.scroll.scroll_offset != before)
            }
            MouseEventKind::ScrollDown => {
                self.state.scroll.scroll_down(amount);
                Some(self.state.scroll.scroll_offset != before)
            }
            _ => None,
        }
    }
    fn handle_widget_key(&mut self, key: crossterm::event::KeyEvent) -> (MarkdownEvent, bool) {
        let before = self.state_snapshot();
        let (event, sync_state) = {
            let mut widget = MarkdownWidget::from_state(&self.state)
                .show_toc(true)
                .show_statusline(true)
                .show_scrollbar(true);
            let event = widget.handle_key(key);
            let sync_state = widget.get_state_sync();
            (event, sync_state)
        };
        sync_state.apply_to(&mut self.state);
        self.apply_event_to_state(&event);
        let after = self.state_snapshot();
        (event, before != after)
    }

    fn handle_widget_mouse(
        &mut self,
        mouse: crossterm::event::MouseEvent,
    ) -> (MarkdownEvent, bool) {
        let before = self.state_snapshot();
        let area = self.state.inner_area();
        let (event, sync_state) = {
            let mut widget = MarkdownWidget::from_state(&self.state)
                .show_toc(true)
                .show_statusline(true)
                .show_scrollbar(true);
            let event = widget.handle_mouse(mouse, area);
            let sync_state = widget.get_state_sync();
            (event, sync_state)
        };
        sync_state.apply_to(&mut self.state);
        self.apply_event_to_state(&event);
        let after = self.state_snapshot();
        (event, before != after)
    }

    fn apply_event_to_state(&mut self, event: &MarkdownEvent) {
        match event {
            MarkdownEvent::Scrolled { offset, .. } => {
                self.state.scroll.scroll_offset = *offset;
            }
            MarkdownEvent::FocusedLine { line } => {
                self.state.scroll.current_line = *line;
            }
            MarkdownEvent::FilterModeChanged { active, filter } => {
                self.state.filter_mode = *active;
                self.state.filter = Some(filter.clone());
                self.state.scroll.filter_mode = *active;
                self.state.scroll.filter = Some(filter.clone());
            }
            MarkdownEvent::FilterModeExited { line } => {
                self.state.filter_mode = false;
                self.state.filter = None;
                self.state.scroll.filter_mode = false;
                self.state.scroll.filter = None;
                self.state.scroll.current_line = *line;
            }
            _ => {}
        }
    }

    fn state_snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            toc_hovered: self.state.toc_hovered,
            toc_hovered_entry: self.state.toc_hovered_entry,
            toc_scroll_offset: self.state.toc_scroll_offset,
            selection_active: self.state.selection_active,
            filter: self.state.filter.clone(),
            filter_mode: self.state.filter_mode,
            scroll_offset: self.state.scroll.scroll_offset,
            current_line: self.state.scroll.current_line,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StateSnapshot {
    toc_hovered: bool,
    toc_hovered_entry: Option<usize>,
    toc_scroll_offset: usize,
    selection_active: bool,
    filter: Option<String>,
    filter_mode: bool,
    scroll_offset: usize,
    current_line: usize,
}

fn main() -> io::Result<()> {
    let mut app = MarkdownPreviewDemo::new();
    run(&mut app, RunConfig::default())
}

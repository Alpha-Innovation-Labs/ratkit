use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind, MouseEvent};
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_resizable_grid::{ResizableGrid, ResizableGridWidget, ResizableGridWidgetState};

struct ResizableGridDemo {
    layout: ResizableGrid,
    state: ResizableGridWidgetState,
    last_area: Rect,
}

impl ResizableGridDemo {
    fn new() -> Self {
        let mut layout = ResizableGrid::new(0);
        let bottom_pane = layout.split_pane_horizontally(0).unwrap_or(0);
        let _top_right = layout.split_pane_vertically(0).unwrap_or(0);
        let _bottom_right = layout.split_pane_vertically(bottom_pane).unwrap_or(0);
        let _ = layout.resize_divider(0, 60);
        let _ = layout.resize_divider(bottom_pane, 40);

        Self {
            layout,
            state: ResizableGridWidgetState::default(),
            last_area: Rect::default(),
        }
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) {
        let mut widget = ResizableGridWidget::new(self.layout.clone()).with_state(self.state);
        widget.handle_mouse(mouse, self.last_area);
        self.state = widget.state();
        self.layout = widget.layout().clone();
    }
}

impl App for ResizableGridDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Crossterm(Event::Key(key))
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') =>
            {
                Ok(RunnerAction::Quit)
            }
            RunnerEvent::Crossterm(Event::Mouse(mouse)) => {
                self.handle_mouse(mouse);
                Ok(RunnerAction::Redraw)
            }
            _ => Ok(RunnerAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Resizable Grid ");
        let inner = block.inner(area);
        self.last_area = inner;
        frame.render_widget(block, area);

        let widget = ResizableGridWidget::new(self.layout.clone()).with_state(self.state);
        self.state = widget.state();
        self.layout = widget.layout().clone();
        frame.render_widget(widget, inner);
    }
}

fn main() -> io::Result<()> {
    let mut app = ResizableGridDemo::new();
    run(&mut app, RunConfig::default())
}

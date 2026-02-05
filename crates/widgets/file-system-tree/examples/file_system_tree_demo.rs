use std::io;
use std::path::PathBuf;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_file_system_tree::{FileSystemTree, FileSystemTreeState};

struct FileSystemTreeDemo {
    tree: FileSystemTree<'static>,
    state: FileSystemTreeState,
    last_selection: String,
}

impl FileSystemTreeDemo {
    fn new() -> io::Result<Self> {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let tree =
            FileSystemTree::new(root).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        let mut state = FileSystemTreeState::new();
        state.select(vec![0]);

        Ok(Self {
            tree,
            state,
            last_selection: "No selection".to_string(),
        })
    }
}

impl App for FileSystemTreeDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Crossterm(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char('q') => return Ok(RunnerAction::Quit),
                    KeyCode::Down => self.tree.select_next(&mut self.state),
                    KeyCode::Up => self.tree.select_previous(&mut self.state),
                    KeyCode::Enter => {
                        let _ = self.tree.toggle_selected(&mut self.state);
                    }
                    KeyCode::Char('/') => {
                        if !self.tree.is_filter_mode(&self.state) {
                            self.tree.enter_filter_mode(&mut self.state);
                        }
                    }
                    _ => {
                        if self.tree.is_filter_mode(&self.state) {
                            let _ = self.tree.handle_filter_key(key.code, &mut self.state);
                        }
                    }
                }

                if let Some(entry) = self.tree.get_selected_entry(&self.state) {
                    self.last_selection = entry.path.display().to_string();
                }

                Ok(RunnerAction::Redraw)
            }
            _ => Ok(RunnerAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let tree = self.tree.clone().block(
            Block::default()
                .borders(Borders::ALL)
                .title(" File System "),
        );
        frame.render_stateful_widget(tree, layout[0], &mut self.state);

        let footer = Paragraph::new(vec![
            Line::from("Up/Down select, Enter toggle, / filter"),
            Line::from(format!("Selected: {}", self.last_selection)),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Status "));
        frame.render_widget(footer, layout[1]);
    }
}

fn main() -> io::Result<()> {
    let mut app = FileSystemTreeDemo::new()?;
    run(&mut app, RunConfig::default())
}

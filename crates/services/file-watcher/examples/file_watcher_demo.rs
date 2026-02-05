use std::io;
use std::path::PathBuf;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_file_watcher::FileWatcher;

struct FileWatcherDemo {
    watcher: FileWatcher,
    last_change: String,
}

impl FileWatcherDemo {
    fn new() -> io::Result<Self> {
        let mut watcher = FileWatcher::for_directory()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        let path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        watcher
            .watch(&path)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(Self {
            watcher,
            last_change: "No changes yet".to_string(),
        })
    }
}

impl App for FileWatcherDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Tick => {
                if self.watcher.check_for_changes() {
                    let paths = self.watcher.get_changed_paths();
                    if let Some(path) = paths.first() {
                        self.last_change = format!("Changed: {}", path.display());
                    } else {
                        self.last_change = "Changes detected".to_string();
                    }
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
        let body = Paragraph::new(vec![
            Line::from("Watching current directory"),
            Line::from(self.last_change.clone()),
            Line::from("Press q to quit"),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" FileWatcher "),
        );
        frame.render_widget(body, area);
    }
}

fn main() -> io::Result<()> {
    let mut app = FileWatcherDemo::new()?;
    run(&mut app, RunConfig::default())
}

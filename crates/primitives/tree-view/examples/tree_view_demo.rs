use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders},
    Frame,
};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_tree_view::{TreeNavigator, TreeNode, TreeView, TreeViewState};

struct TreeViewDemo {
    nodes: Vec<TreeNode<&'static str>>,
    state: TreeViewState,
    navigator: TreeNavigator,
}

impl TreeViewDemo {
    fn new() -> Self {
        let nodes = vec![
            TreeNode::with_children(
                "src",
                vec![
                    TreeNode::new("lib.rs"),
                    TreeNode::new("main.rs"),
                    TreeNode::with_children(
                        "widgets",
                        vec![TreeNode::new("button.rs"), TreeNode::new("dialog.rs")],
                    ),
                ],
            ),
            TreeNode::with_children("tests", vec![TreeNode::new("smoke.rs")]),
        ];

        Self {
            nodes,
            state: TreeViewState::new(),
            navigator: TreeNavigator::new(),
        }
    }

    fn build_tree(&self) -> TreeView<'static, &'static str> {
        TreeView::new(self.nodes.clone())
            .render_fn(|data, state| {
                if state.is_selected {
                    Line::from(format!("> {}", data))
                } else {
                    Line::from(*data)
                }
            })
            .highlight_style(Style::default().bg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title(" Tree View "))
            .with_filter_ui(true)
    }
}

impl App for TreeViewDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Crossterm(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char('q') => return Ok(RunnerAction::Quit),
                    KeyCode::Char('/') => {
                        self.state.enter_filter_mode();
                    }
                    _ => {
                        let mut tree = self.build_tree();
                        let _ = tree.handle_key_event(key, &self.navigator, &mut self.state);
                    }
                }
                Ok(RunnerAction::Redraw)
            }
            _ => Ok(RunnerAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area: Rect = frame.area();
        let tree = self.build_tree();
        frame.render_stateful_widget(tree, area, &mut self.state);
    }
}

fn main() -> io::Result<()> {
    let mut app = TreeViewDemo::new();
    run(&mut app, RunConfig::default())
}

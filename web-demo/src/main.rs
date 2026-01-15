use std::cell::RefCell;
use std::rc::Rc;

use ratzilla::ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Tabs},
    Terminal,
};

use ratzilla::event::KeyCode;
use ratzilla::{DomBackend, WebRenderer};

fn main() -> std::io::Result<()> {
    let current_tab = Rc::new(RefCell::new(0usize));
    let tabs = vec![
        "Layout",
        "Tree",
        "Dialogs",
        "Markdown",
        "Scrollbar",
        "StatusLine",
        "Terminal",
    ];
    let tabs_for_render = tabs.clone();

    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    terminal.on_key_event({
        let current_tab = current_tab.clone();
        let tabs_for_keys = tabs.clone();
        move |event| {
            let mut tab = current_tab.borrow_mut();
            match event.code {
                KeyCode::Tab => {
                    *tab = (*tab + 1) % tabs_for_keys.len();
                }
                KeyCode::Esc => std::process::exit(0),
                _ => {}
            }
        }
    });

    terminal.draw_web(move |f| {
        let tab_idx = *current_tab.borrow();
        let title = match tab_idx {
            0 => "Layout - Resizable Split Panes",
            1 => "Tree View - Navigate with j/k/h/l",
            2 => "Dialogs - i/w/e/c for types",
            3 => "Markdown - Scroll with j/k",
            4 => "Scrollbar - Interactive",
            5 => "StatusLine - Vim modes: i/v/:",
            6 => "Terminal - Embedded VT100",
            _ => "Unknown",
        };

        let content = match tab_idx {
            0 => vec![
                "  ResizableSplit - Drag or use arrows",
                "",
                "  +----------+  +------------------+",
                "  |   Left   |  |     Right        |",
                "  |          |  |                  |",
                "  |          |  |                  |",
                "  +----------+  +------------------+",
                "",
                "  * Vertical/Horizontal splits",
                "  * Mouse drag support",
                "  * Min/Max ratio constraints",
            ],
            1 => vec![
                "  TreeView - Navigate with j/k/h/l",
                "",
                "  Components",
                "    > Button",
                "      Dialog",
                "      Toast",
                "  Layout",
                "    > ResizableSplit",
                "      MasterLayout",
                "",
                "  j/k: Navigate | h/l: Collapse/Expand",
            ],
            2 => vec![
                "  Dialogs - i/w/e/c for types, Esc to close",
                "",
                "        +----------------------+",
                "        |     Info Dialog      |",
                "        |                      |",
                "        |  This is an info msg |",
                "        |                      |",
                "        |        [OK]          |",
                "        +----------------------+",
                "",
                "  i: Info | w: Warning | e: Error | c: Confirm",
            ],
            3 => vec![
                "  Markdown Rendering",
                "",
                "  # Markdown Renderer",
                "",
                "  The **MarkdownRenderer** converts",
                "  markdown to styled `ratatui::Text`.",
                "",
                "  ## Features",
                "  * Bold and italic text",
                "  * Inline code snippets",
                "  * Code blocks with syntax",
            ],
            4 => vec![
                "  ClickableScrollbar",
                "",
                "  Line 1: Content for scroll demo",
                "  Line 2: Content for scroll demo",
                "  Line 3: Content for scroll demo",
                "  Line 4: Content for scroll demo",
                "  Line 5: Content for scroll demo",
                "  Line 6: Content for scroll demo",
                "  Line 7: Content for scroll demo",
                "",
                "  Click track/thumb or use PageUp/Down",
            ],
            5 => vec![
                "  StatusLine - Vim modes",
                "",
                "  +----------------------------------+",
                "  | NORMAL | src/main.rs | rust:1.70 |",
                "  +----------------------------------+",
                "",
                "  Vim-like mode indicator",
                "  * NORMAL - Command mode",
                "  * INSERT - Edit mode",
                "  * VISUAL - Selection mode",
                "  * COMMAND - :command mode",
            ],
            6 => vec![
                "  VT100Term - Embedded Terminal",
                "",
                "  $ echo 'Hello from ratatui-toolkit!'",
                "  Hello from ratatui-toolkit!",
                "",
                "  $ cargo run --example showcase",
                "  [Compiling ratatui-toolkit...]",
                "  [Finished dev]",
                "  [Running `showcase`]",
                "",
                "  Full terminal emulation with scrollback",
            ],
            _ => vec!["Unknown tab"],
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Min(0)])
            .split(f.area());

        let tab_titles: Vec<Line> = tabs_for_render
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let style = if i == tab_idx {
                    Style::default()
                        .fg(Color::Yellow)
                        .bg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                Line::from(Span::styled(*t, style))
            })
            .collect();

        let tabs_widget = Tabs::new(tab_titles)
            .select(tab_idx)
            .style(Style::default().fg(Color::Black));

        f.render_widget(tabs_widget, chunks[0]);

        let text = Text::from(content.join("\n"));
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(Color::White))
            .block(Block::bordered().title(title).border_style(Color::Yellow));

        f.render_widget(paragraph, chunks[1]);
    });

    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }

    Ok(())
}

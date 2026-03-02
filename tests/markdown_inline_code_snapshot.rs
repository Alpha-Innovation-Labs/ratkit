use insta::assert_snapshot;
use ratatui::backend::TestBackend;
use ratatui::style::Color;
use ratatui::Terminal;
use ratkit::widgets::markdown_preview::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    MarkdownWidget, ScrollState, SelectionState, SourceState, VimState,
};

fn render_snapshot_text() -> String {
    let markdown = "5. Run examples with `--features` flag: Examples require their specific features (e.g., `--features markdown-preview`)";

    let mut source = SourceState::default();
    source.set_source_string(markdown.to_string());

    let mut scroll = ScrollState::default();
    scroll.update_total_lines(markdown.lines().count().max(1));

    let mut widget = MarkdownWidget::new(
        markdown.to_string(),
        scroll,
        source,
        CacheState::default(),
        DisplaySettings::default(),
        CollapseState::default(),
        ExpandableState::default(),
        GitStatsState::default(),
        VimState::default(),
        SelectionState::default(),
        DoubleClickState::default(),
    )
    .with_has_pane(false)
    .show_toc(false)
    .show_scrollbar(false)
    .show_statusline(false);

    let backend = TestBackend::new(56, 8);
    let mut terminal = Terminal::new(backend).expect("create test terminal");
    terminal
        .draw(|frame| frame.render_widget(&mut widget, frame.area()))
        .expect("draw markdown widget");

    let buffer = terminal.backend().buffer();
    let mut out = String::new();
    for y in 0..buffer.area.height {
        let mut text_line = String::new();
        let mut bg_line = String::new();

        for x in 0..buffer.area.width {
            let cell = &buffer[(x, y)];
            let ch = cell.symbol().chars().next().unwrap_or(' ');
            text_line.push(ch);

            if cell.bg == Color::Rgb(60, 60, 60) {
                bg_line.push('^');
            } else {
                bg_line.push(' ');
            }
        }

        if text_line.contains("--features")
            || text_line.contains("markdown-preview")
            || bg_line.contains('^')
        {
            out.push_str(&format!("{y:02} T|{text_line}|\n"));
            out.push_str(&format!("{y:02} B|{bg_line}|\n"));
        }
    }

    out
}

#[test]
fn snapshot_inline_code_wrap_background() {
    let snapshot = render_snapshot_text();
    assert_snapshot!(snapshot);
}

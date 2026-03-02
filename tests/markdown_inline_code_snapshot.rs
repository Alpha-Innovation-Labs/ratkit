use insta::assert_snapshot;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use ratkit::widgets::markdown_preview::{
    render_markdown_to_elements, CacheState, CollapseState, DisplaySettings, DoubleClickState,
    ElementKind, ExpandableState, GitStatsState, MarkdownWidget, ScrollState, SelectionState,
    SourceState, VimState, INLINE_CODE_FG_FALLBACK,
};

fn render_snapshot_text(markdown: &str, width: u16, height: u16) -> String {
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

    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).expect("create test terminal");
    terminal
        .draw(|frame| frame.render_widget(&mut widget, frame.area()))
        .expect("draw markdown widget");

    let buffer = terminal.backend().buffer();
    let mut out = String::new();
    for y in 0..buffer.area.height {
        let mut text_line = String::new();
        let mut code_fg_line = String::new();

        for x in 0..buffer.area.width {
            let cell = &buffer[(x, y)];
            let ch = cell.symbol().chars().next().unwrap_or(' ');
            text_line.push(ch);

            if cell.fg == INLINE_CODE_FG_FALLBACK {
                code_fg_line.push('^');
            } else {
                code_fg_line.push(' ');
            }
        }

        if !text_line.trim().is_empty() || code_fg_line.contains('^') {
            out.push_str(&format!("{y:02} T|{text_line}|\n"));
            out.push_str(&format!("{y:02} C|{code_fg_line}|\n"));
        }
    }

    out
}

#[test]
fn snapshot_inline_code_wrap_background() {
    let markdown = "5. Run examples with `--features` flag: Examples require their specific features (e.g., `--features markdown-preview`)";
    let parsed = render_markdown_to_elements(markdown, false);
    let inline_code_segments = parsed
        .iter()
        .filter_map(|el| match &el.kind {
            ElementKind::ListItem { content, .. } | ElementKind::Paragraph(content) => {
                Some(content)
            }
            _ => None,
        })
        .flat_map(|segments| segments.iter())
        .filter(|segment| {
            matches!(
                segment,
                ratkit::widgets::markdown_preview::TextSegment::InlineCode(_)
            )
        })
        .count();
    assert_eq!(
        inline_code_segments, 2,
        "expected two inline-code segments in parsed markdown"
    );

    let snapshot = render_snapshot_text(markdown, 56, 8);
    assert_snapshot!("inline_code_wrap_background", snapshot);
}

#[test]
fn snapshot_no_dropped_first_char_on_wrapped_inline_code() {
    let markdown = "3. **Cross-feature dependencies**: Some features auto-enable others (e.g., `tree-view` enables `widget-event`, `repo-watcher` enables `file-watcher` and `git-watcher`)";
    let snapshot = render_snapshot_text(markdown, 120, 8);

    assert!(
        !snapshot.contains(" ile-watcher "),
        "first character was dropped in wrapped inline code:\n{snapshot}"
    );

    assert_snapshot!("inline_code_no_dropped_first_char", snapshot);
}

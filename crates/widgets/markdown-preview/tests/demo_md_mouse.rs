use crossterm::event::{KeyModifiers, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;
use ratkit_markdown_preview::{MarkdownState, MarkdownWidget};

#[test]
fn demo_md_mouse_scroll_changes_scroll_offset() {
    let mut state = MarkdownState::default();
    let mut content = String::new();
    for idx in 0..200 {
        content.push_str(&format!("line {idx}\n"));
    }
    state.source.set_source_string(content);
    state.cache.invalidate();
    state.scroll.total_lines = 200;
    state.scroll.viewport_height = 10;
    state.set_inner_area(Rect {
        x: 0,
        y: 0,
        width: 80,
        height: 10,
    });

    let area = state.inner_area();
    let mut widget = MarkdownWidget::from_state(&state)
        .show_toc(true)
        .show_statusline(true)
        .show_scrollbar(true);

    let _event = widget.handle_mouse(
        MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: 1,
            row: 1,
            modifiers: KeyModifiers::NONE,
        },
        area,
    );
    let sync_state = widget.get_state_sync();
    sync_state.apply_to(&mut state);
    assert!(state.scroll.scroll_offset > 0);
}

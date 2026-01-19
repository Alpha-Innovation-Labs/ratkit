use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui_toolkit::markdown_widget::state::MarkdownState;
use ratatui_toolkit::markdown_widget::widget::enums::MarkdownWidgetMode;
use ratatui_toolkit::MarkdownWidget;

#[test]
fn test_esc_exits_filter_and_clears_filter_state() {
    let mut state = MarkdownState::default();
    state.source.set_content("Line 1\nLine 2\nLine 3");

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode and type something
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('t'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    assert!(widget.filter_mode, "Should be in filter mode");
    assert_eq!(widget.filter, Some("t".to_string()));
    assert_eq!(widget.mode, MarkdownWidgetMode::Filter);

    // Press Esc to exit
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    assert!(!widget.filter_mode, "Should exit filter mode");
    assert!(widget.filter.is_none(), "Filter should be cleared");
    assert_eq!(widget.mode, MarkdownWidgetMode::Normal);
    assert!(
        widget.scroll.filter.is_none(),
        "Scroll filter should be cleared"
    );
    assert!(
        !widget.scroll.filter_mode,
        "Scroll filter mode should be cleared"
    );
    assert!(widget.cache.render.is_none(), "Cache should be cleared");
}

#[test]
fn test_esc_state_sync_workflow() {
    let mut state = MarkdownState::default();
    state.source.set_content("Line 1\nLine 2\nLine 3");

    // Enter filter mode
    let content = state.content().to_string();
    {
        let mut widget = MarkdownWidget::from_state(&content, &mut state);
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char('/'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char('t'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
        let sync = widget.get_state_sync();
        sync.apply_to(&mut state);
    }

    // Verify state is in filter mode
    assert!(state.filter_mode);
    assert_eq!(state.filter, Some("t".to_string()));

    // Exit filter mode
    let content = state.content().to_string();
    {
        let mut widget = MarkdownWidget::from_state(&content, &mut state);
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
        let sync = widget.get_state_sync();
        sync.apply_to(&mut state);
    }

    // Verify state is NOT in filter mode after sync
    assert!(!state.filter_mode, "state.filter_mode should be false");
    assert!(state.filter.is_none(), "state.filter should be None");

    // Verify new widget instance picks up correct state
    let content = state.content().to_string();
    let widget = MarkdownWidget::from_state(&content, &mut state);
    assert!(
        !widget.filter_mode,
        "New widget should not be in filter mode"
    );
    assert!(widget.filter.is_none(), "New widget should have no filter");
    assert_eq!(widget.mode, MarkdownWidgetMode::Normal);
}

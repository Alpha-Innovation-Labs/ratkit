//! Tests for markdown widget filter functionality.

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui_toolkit::markdown_widget::state::MarkdownState;
use ratatui_toolkit::markdown_widget::widget::enums::MarkdownWidgetMode;
use ratatui_toolkit::MarkdownWidget;

// ============================================================================
// Filter Mode Entry Tests
// ============================================================================

#[test]
fn test_filter_mode_entered_with_slash() {
    let mut state = MarkdownState::default();
    state
        .source
        .set_content("# Heading\n\nSome paragraph text.");

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Initially not in filter mode
    assert!(
        !widget.filter_mode,
        "Should not be in filter mode initially"
    );
    assert_eq!(widget.mode, MarkdownWidgetMode::Normal);

    // Press '/' to enter filter mode
    let slash_event = KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    };

    let result = widget.handle_key_event(slash_event);

    // Should now be in filter mode
    assert!(
        widget.filter_mode,
        "Should be in filter mode after pressing /"
    );
    assert_eq!(widget.mode, MarkdownWidgetMode::Filter);
    assert!(widget.filter.is_some(), "Filter text should be Some");
    assert!(
        widget.filter.as_ref().unwrap().is_empty(),
        "Filter text should be empty initially"
    );
}

#[test]
fn test_filter_mode_can_be_entered_multiple_times() {
    let mut state = MarkdownState::default();
    state.source.set_content("Line 1\nLine 2\nLine 3");

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode
    let slash_event = KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    };
    widget.handle_key_event(slash_event);
    assert!(widget.filter_mode);

    // Exit filter mode with Esc
    let esc_event = KeyEvent {
        code: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    };
    widget.handle_key_event(esc_event);
    assert!(!widget.filter_mode);

    // Enter filter mode again
    widget.handle_key_event(slash_event);
    assert!(widget.filter_mode);
}

// ============================================================================
// Filter Text Input Tests
// ============================================================================

#[test]
fn test_filter_text_accumulates_as_user_types() {
    let mut state = MarkdownState::default();
    state
        .source
        .set_content("# Heading\n\nSome paragraph text.");

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    assert_eq!(widget.filter, Some(String::new()));

    // Type 'h'
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('h'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    assert_eq!(widget.filter, Some("h".to_string()));

    // Type 'e'
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('e'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    assert_eq!(widget.filter, Some("he".to_string()));

    // Type 'a'
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    assert_eq!(widget.filter, Some("hea".to_string()));
}

#[test]
fn test_filter_backspace_removes_last_character() {
    let mut state = MarkdownState::default();
    state.source.set_content("Heading\nSome text");

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode and type "test"
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    for c in "test".chars() {
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
    }
    assert_eq!(widget.filter, Some("test".to_string()));

    // Press backspace
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    assert_eq!(widget.filter, Some("tes".to_string()));

    // Press backspace again
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    assert_eq!(widget.filter, Some("te".to_string()));
}

// ============================================================================
// Filter Mode Exit Tests
// ============================================================================

#[test]
fn test_esc_exits_filter_mode_and_clears_filter() {
    let mut state = MarkdownState::default();
    state.source.set_content("Line 1\nLine 2");

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode and type something
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    for c in "test".chars() {
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
    }

    assert!(widget.filter_mode);
    assert_eq!(widget.filter, Some("test".to_string()));

    // Press Esc to exit
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    assert!(!widget.filter_mode);
    assert!(widget.filter.is_none());
    assert_eq!(widget.mode, MarkdownWidgetMode::Normal);
}

#[test]
fn test_enter_exits_filter_mode_and_returns_line() {
    let mut state = MarkdownState::default();
    state
        .source
        .set_content("First line\nSecond line\nThird line");
    state.scroll.current_line = 2;

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Press Enter to exit
    let result = widget.handle_key_event(KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Should exit filter mode
    assert!(!widget.filter_mode);
    assert!(widget.filter.is_none());
    assert_eq!(widget.mode, MarkdownWidgetMode::Normal);

    // Should return the current line number
    use ratatui_toolkit::markdown_widget::foundation::events::MarkdownEvent;
    match result {
        MarkdownEvent::FilterModeExited { line } => {
            assert_eq!(line, 2);
        }
        _ => panic!("Expected FilterModeExited event"),
    }
}

// ============================================================================
// Filter Navigation Tests
// ============================================================================

#[test]
fn test_j_moves_to_next_matching_line() {
    let mut state = MarkdownState::default();
    state
        .source
        .set_content("# Heading One\nSome text\n# Heading Two\nMore text");

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);
    widget.scroll.current_line = 1;

    // Enter filter mode and search for "Heading"
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    for c in "Heading".chars() {
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
    }

    // Should be on first match (line 1)
    assert_eq!(widget.scroll.current_line, 1);

    // Press j to go to next match
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('j'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Should be on second match (line 3)
    assert_eq!(widget.scroll.current_line, 3);
}

#[test]
fn test_k_moves_to_previous_matching_line() {
    let mut state = MarkdownState::default();
    state
        .source
        .set_content("# Heading One\nSome text\n# Heading Two\nMore text");
    state.scroll.current_line = 3;

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode and search for "Heading"
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    for c in "Heading".chars() {
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
    }

    // Press k to go to previous match
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('k'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Should be on first match (line 1)
    assert_eq!(widget.scroll.current_line, 1);
}

#[test]
fn test_down_arrow_moves_to_next_matching_line() {
    let mut state = MarkdownState::default();
    state.source.set_content("Alpha\nBeta\nGamma\nDelta");
    state.scroll.current_line = 1;

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode and search for "a" (matches Alpha, Beta, Gamma, Delta)
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Press Down arrow
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Should move to next line
    assert_eq!(widget.scroll.current_line, 2);
}

#[test]
fn test_up_arrow_moves_to_previous_matching_line() {
    let mut state = MarkdownState::default();
    state.source.set_content("Alpha\nBeta\nGamma\nDelta");
    state.scroll.current_line = 3;

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode and search for "a"
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Press Up arrow
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Should move to previous line
    assert_eq!(widget.scroll.current_line, 2);
}

#[test]
fn test_ctrl_n_moves_to_next_matching_line() {
    let mut state = MarkdownState::default();
    state
        .source
        .set_content("First match\nNo match here\nSecond match");
    state.scroll.current_line = 1;

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode and search for "match"
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    for c in "match".chars() {
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
    }

    // Press Ctrl+n
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('n'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Should move to second match (line 3)
    assert_eq!(widget.scroll.current_line, 3);
}

#[test]
fn test_ctrl_p_moves_to_previous_matching_line() {
    let mut state = MarkdownState::default();
    state
        .source
        .set_content("First match\nNo match here\nSecond match");
    state.scroll.current_line = 3;

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode and search for "match"
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    for c in "match".chars() {
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
    }

    // Press Ctrl+p
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('p'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Should move to first match (line 1)
    assert_eq!(widget.scroll.current_line, 1);
}

// ============================================================================
// State Sync Tests
// ============================================================================

#[test]
fn test_filter_state_synced_to_markdown_state() {
    let mut state = MarkdownState::default();
    state.source.set_content("Test content");

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Type filter text
    for c in "test".chars() {
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
    }

    // Sync state back
    let sync = widget.get_state_sync();
    sync.apply_to(&mut state);

    // Verify state was synced
    assert!(state.filter_mode);
    assert_eq!(state.filter, Some("test".to_string()));
}

#[test]
fn test_filter_mode_restored_from_markdown_state() {
    let mut state = MarkdownState::default();
    state.source.set_content("Test content");
    state.filter_mode = true;
    state.filter = Some("test".to_string());

    let content = state.content().to_string();
    let widget = MarkdownWidget::from_state(&content, &mut state);

    // Widget should be in filter mode
    assert!(widget.filter_mode);
    assert_eq!(widget.filter, Some("test".to_string()));
    assert_eq!(widget.mode, MarkdownWidgetMode::Filter);
}

// ============================================================================
// Filter Case Insensitivity Tests
// ============================================================================

#[test]
fn test_filter_is_case_insensitive() {
    let mut state = MarkdownState::default();
    state
        .source
        .set_content("Hello World\nHELLO\nhello\nAnother line");

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);
    widget.scroll.current_line = 1;

    // Enter filter mode and search for "hello" (lowercase)
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    for c in "hello".chars() {
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
    }

    // Press j to go to next match - should find "Hello World" first, then "HELLO", then "hello"
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('j'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    assert_eq!(widget.scroll.current_line, 2); // HELLO

    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('j'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    assert_eq!(widget.scroll.current_line, 3); // hello
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_filter_with_no_matches() {
    let mut state = MarkdownState::default();
    state.source.set_content("Line 1\nLine 2\nLine 3");
    state.scroll.current_line = 1;

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode and search for something not in content
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    for c in "xyznotfound".chars() {
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
    }

    // Navigation should not crash
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('j'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('k'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
}

#[test]
fn test_filter_at_bottom_stays_at_bottom() {
    let mut state = MarkdownState::default();
    state
        .source
        .set_content("Match 1\nNo match\nMatch 2\nNo match\nMatch 3");
    state.scroll.current_line = 5;

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode and search for "Match"
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    for c in "Match".chars() {
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
    }

    // Press j at bottom - should stay at last match
    let previous_line = widget.scroll.current_line;
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('j'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Should stay at same line (or at last match)
    assert!(widget.scroll.current_line <= 5);
}

#[test]
fn test_filter_at_top_stays_at_top() {
    let mut state = MarkdownState::default();
    state.source.set_content("Match 1\nNo match\nMatch 2");
    state.scroll.current_line = 1;

    let content = state.content().to_string();
    let mut widget = MarkdownWidget::from_state(&content, &mut state);

    // Enter filter mode and search for "Match"
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    for c in "Match".chars() {
        widget.handle_key_event(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
    }

    // Press k at top - should stay at first match
    widget.handle_key_event(KeyEvent {
        code: KeyCode::Char('k'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });

    // Should stay at line 1
    assert_eq!(widget.scroll.current_line, 1);
}

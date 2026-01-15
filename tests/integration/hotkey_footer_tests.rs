use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier};
use ratatui::widgets::Widget;
use ratatui::Terminal;
use ratatui_toolkit::hotkey_footer::{HotkeyFooter, HotkeyFooterBuilder, HotkeyItem};

#[test]
fn test_hotkey_item_new() {
    let item = HotkeyItem::new("j/k", "scroll");
    assert_eq!(item.key, "j/k");
    assert_eq!(item.description, "scroll");
}

#[test]
fn test_hotkey_item_new_with_string() {
    let item = HotkeyItem::new("Enter".to_string(), "select".to_string());
    assert_eq!(item.key, "Enter");
    assert_eq!(item.description, "select");
}

#[test]
fn test_hotkey_item_clone() {
    let item = HotkeyItem::new("?", "help");
    let cloned = item.clone();
    assert_eq!(cloned.key, "?");
    assert_eq!(cloned.description, "help");
}

#[test]
fn test_hotkey_footer_new_default_styling() {
    let items = vec![HotkeyItem::new("j/k", "scroll")];
    let footer = HotkeyFooter::new(items.clone());

    assert_eq!(footer.items.len(), 1);
    assert_eq!(footer.items[0].key, "j/k");
    assert_eq!(footer.items[0].description, "scroll");
    assert_eq!(footer.key_color, Color::Cyan);
    assert_eq!(footer.description_color, Color::DarkGray);
    assert_eq!(footer.background_color, Color::Black);
}

#[test]
fn test_hotkey_footer_key_color_customization() {
    let items = vec![HotkeyItem::new("j/k", "scroll")];
    let footer = HotkeyFooter::new(items).key_color(Color::Yellow);

    assert_eq!(footer.key_color, Color::Yellow);
    assert_eq!(footer.description_color, Color::DarkGray); // unchanged
    assert_eq!(footer.background_color, Color::Black); // unchanged
}

#[test]
fn test_hotkey_footer_description_color_customization() {
    let items = vec![HotkeyItem::new("j/k", "scroll")];
    let footer = HotkeyFooter::new(items).description_color(Color::White);

    assert_eq!(footer.key_color, Color::Cyan); // unchanged
    assert_eq!(footer.description_color, Color::White);
    assert_eq!(footer.background_color, Color::Black); // unchanged
}

#[test]
fn test_hotkey_footer_background_color_customization() {
    let items = vec![HotkeyItem::new("j/k", "scroll")];
    let footer = HotkeyFooter::new(items).background_color(Color::DarkGray);

    assert_eq!(footer.key_color, Color::Cyan); // unchanged
    assert_eq!(footer.description_color, Color::DarkGray); // unchanged
    assert_eq!(footer.background_color, Color::DarkGray);
}

#[test]
fn test_hotkey_footer_chained_color_customization() {
    let items = vec![HotkeyItem::new("j/k", "scroll")];
    let footer = HotkeyFooter::new(items)
        .key_color(Color::Green)
        .description_color(Color::LightBlue)
        .background_color(Color::Rgb(40, 40, 40));

    assert_eq!(footer.key_color, Color::Green);
    assert_eq!(footer.description_color, Color::LightBlue);
    assert_eq!(footer.background_color, Color::Rgb(40, 40, 40));
}

#[test]
fn test_hotkey_footer_builder_new() {
    let footer = HotkeyFooterBuilder::new().build();
    assert_eq!(footer.items.len(), 0);
}

#[test]
fn test_hotkey_footer_builder_default() {
    let footer = HotkeyFooterBuilder::default().build();
    assert_eq!(footer.items.len(), 0);
}

#[test]
fn test_hotkey_footer_builder_add_single() {
    let footer = HotkeyFooterBuilder::new().add("j/k", "scroll").build();

    assert_eq!(footer.items.len(), 1);
    assert_eq!(footer.items[0].key, "j/k");
    assert_eq!(footer.items[0].description, "scroll");
}

#[test]
fn test_hotkey_footer_builder_add_multiple_chained() {
    let footer = HotkeyFooterBuilder::new()
        .add("j/k", "scroll")
        .add("Enter", "select")
        .add("?", "help")
        .build();

    assert_eq!(footer.items.len(), 3);
    assert_eq!(footer.items[0].key, "j/k");
    assert_eq!(footer.items[0].description, "scroll");
    assert_eq!(footer.items[1].key, "Enter");
    assert_eq!(footer.items[1].description, "select");
    assert_eq!(footer.items[2].key, "?");
    assert_eq!(footer.items[2].description, "help");
}

#[test]
fn test_hotkey_footer_builder_add_items() {
    let items = vec![
        ("j/k".to_string(), "scroll"),
        ("Enter".to_string(), "select"),
        ("?".to_string(), "help"),
    ];

    let footer = HotkeyFooterBuilder::new().add_items(items).build();

    assert_eq!(footer.items.len(), 3);
    assert_eq!(footer.items[0].key, "j/k");
    assert_eq!(footer.items[1].key, "Enter");
    assert_eq!(footer.items[2].key, "?");
}

#[test]
fn test_hotkey_footer_builder_add_items_mixed_with_add() {
    let items = vec![
        ("j/k".to_string(), "scroll"),
        ("Enter".to_string(), "select"),
    ];

    let footer = HotkeyFooterBuilder::new()
        .add("q", "quit")
        .add_items(items)
        .add("?", "help")
        .build();

    assert_eq!(footer.items.len(), 4);
    assert_eq!(footer.items[0].key, "q");
    assert_eq!(footer.items[1].key, "j/k");
    assert_eq!(footer.items[2].key, "Enter");
    assert_eq!(footer.items[3].key, "?");
}

#[test]
fn test_hotkey_footer_builder_with_color_customization() {
    let footer = HotkeyFooterBuilder::new()
        .add("j/k", "scroll")
        .build()
        .key_color(Color::Yellow)
        .description_color(Color::White);

    assert_eq!(footer.items.len(), 1);
    assert_eq!(footer.key_color, Color::Yellow);
    assert_eq!(footer.description_color, Color::White);
}

#[test]
fn test_build_line_empty_items() {
    let footer = HotkeyFooter::new(vec![]);

    // Use reflection/internal access by rendering to buffer
    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 1));
    footer.render(Rect::new(0, 0, 80, 1), &mut buffer);

    // Empty items should produce empty line (no spans except background)
    let cell = &buffer[(0, 0)];
    assert_eq!(cell.bg, Color::Black);
}

#[test]
fn test_build_line_single_item() {
    let footer = HotkeyFooter::new(vec![HotkeyItem::new("j/k", "scroll")]);

    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 1));
    footer.render(Rect::new(0, 0, 80, 1), &mut buffer);

    // Check background color is applied
    let cell = &buffer[(0, 0)];
    assert_eq!(cell.bg, Color::Black);

    // Verify content contains our hotkey
    let content: String = (0..80)
        .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
        .collect();
    assert!(content.contains("j/k"));
    assert!(content.contains("scroll"));
}

#[test]
fn test_build_line_multiple_items() {
    let items = vec![
        HotkeyItem::new("j/k", "scroll"),
        HotkeyItem::new("Enter", "select"),
        HotkeyItem::new("?", "help"),
    ];
    let footer = HotkeyFooter::new(items);

    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 1));
    footer.render(Rect::new(0, 0, 80, 1), &mut buffer);

    let content: String = (0..80)
        .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
        .collect();

    // All items should be present
    assert!(content.contains("j/k"));
    assert!(content.contains("scroll"));
    assert!(content.contains("Enter"));
    assert!(content.contains("select"));
    assert!(content.contains("?"));
    assert!(content.contains("help"));
}

#[test]
fn test_build_line_spacing() {
    let items = vec![
        HotkeyItem::new("j/k", "scroll"),
        HotkeyItem::new("Enter", "select"),
    ];
    let footer = HotkeyFooter::new(items);

    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 1));
    footer.render(Rect::new(0, 0, 80, 1), &mut buffer);

    let content: String = (0..80)
        .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
        .collect();

    // Should start with a space (leading space for first item)
    assert_eq!(content.chars().next().unwrap(), ' ');

    // Description should have trailing spaces (format!(" {}  ", item.description))
    // This creates: " scroll  " between items
    assert!(content.contains("  ")); // Double space between items
}

#[test]
fn test_build_line_custom_colors() {
    let items = vec![HotkeyItem::new("j/k", "scroll")];
    let footer = HotkeyFooter::new(items)
        .key_color(Color::Yellow)
        .description_color(Color::White)
        .background_color(Color::Rgb(40, 40, 40));

    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 1));
    footer.render(Rect::new(0, 0, 80, 1), &mut buffer);

    // Check background color
    let cell = &buffer[(0, 0)];
    assert_eq!(cell.bg, Color::Rgb(40, 40, 40));
}

#[test]
fn test_build_line_key_styling() {
    let items = vec![HotkeyItem::new("j/k", "scroll")];
    let footer = HotkeyFooter::new(items).key_color(Color::Green);

    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 1));
    footer.render(Rect::new(0, 0, 80, 1), &mut buffer);

    // Find the position where "j/k" starts (after leading space at position 1)
    let j_cell = &buffer[(1, 0)]; // First character of "j/k"

    // Key should be colored and bold
    assert_eq!(j_cell.fg, Color::Green);
    assert!(j_cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_build_line_description_styling() {
    let items = vec![HotkeyItem::new("j/k", "scroll")];
    let footer = HotkeyFooter::new(items)
        .key_color(Color::Cyan)
        .description_color(Color::Gray);

    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 1));
    footer.render(Rect::new(0, 0, 80, 1), &mut buffer);

    // Description starts after " j/k " (positions 0-4)
    let desc_cell = &buffer[(5, 0)]; // First character of "scroll"

    // Description should be colored but not bold
    assert_eq!(desc_cell.fg, Color::Gray);
    assert!(!desc_cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_build_line_many_items() {
    let items = vec![
        HotkeyItem::new("j/k", "scroll"),
        HotkeyItem::new("h/l", "navigate"),
        HotkeyItem::new("Enter", "select"),
        HotkeyItem::new("Esc", "cancel"),
        HotkeyItem::new("?", "help"),
        HotkeyItem::new("q", "quit"),
        HotkeyItem::new("Tab", "switch"),
        HotkeyItem::new("Space", "toggle"),
    ];
    let footer = HotkeyFooter::new(items.clone());

    let mut buffer = Buffer::empty(Rect::new(0, 0, 100, 1));
    footer.render(Rect::new(0, 0, 100, 1), &mut buffer);

    let content: String = (0..100)
        .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
        .collect();

    // All items should be rendered (no overflow handling, just natural line layout)
    for item in &items {
        assert!(content.contains(&item.key));
    }
}

#[test]
fn test_build_line_very_long_keys_and_descriptions() {
    let items = vec![
        HotkeyItem::new("Ctrl+Shift+Alt+X", "execute very long operation"),
        HotkeyItem::new("Meta+Super+F12", "another extremely long description"),
    ];
    let footer = HotkeyFooter::new(items);

    let mut buffer = Buffer::empty(Rect::new(0, 0, 120, 1));
    footer.render(Rect::new(0, 0, 120, 1), &mut buffer);

    let content: String = (0..120)
        .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
        .collect();

    // Long keys and descriptions should be rendered
    assert!(content.contains("Ctrl+Shift+Alt+X"));
    assert!(content.contains("execute very long operation"));
}

#[test]
fn test_widget_trait_owned() {
    let items = vec![HotkeyItem::new("j/k", "scroll")];
    let footer = HotkeyFooter::new(items);

    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 1));

    // Test owned Widget trait
    footer.render(Rect::new(0, 0, 80, 1), &mut buffer);

    let content: String = (0..80)
        .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
        .collect();

    assert!(content.contains("j/k"));
    assert!(content.contains("scroll"));
}

#[test]
fn test_widget_trait_borrowed() {
    let items = vec![HotkeyItem::new("j/k", "scroll")];
    let footer = HotkeyFooter::new(items);

    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 1));

    // Test borrowed Widget trait - use Widget trait explicitly
    <&HotkeyFooter as Widget>::render(&footer, Rect::new(0, 0, 80, 1), &mut buffer);

    let content: String = (0..80)
        .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
        .collect();

    assert!(content.contains("j/k"));
    assert!(content.contains("scroll"));
}

#[test]
fn test_render_with_frame() {
    let backend = TestBackend::new(80, 3);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            let items = vec![HotkeyItem::new("j/k", "scroll")];
            let footer = HotkeyFooter::new(items);
            let area = Rect::new(0, 2, 80, 1); // Bottom row
                                               // Use the render method that takes Frame (call it explicitly)
            HotkeyFooter::render(&footer, frame, area);
        })
        .unwrap();

    // Verify the terminal buffer contains our content
    let buffer = terminal.backend().buffer();
    let content: String = (0..80)
        .map(|x| buffer[(x, 2)].symbol().chars().next().unwrap_or(' '))
        .collect();

    assert!(content.contains("j/k"));
    assert!(content.contains("scroll"));
}

#[test]
fn test_clone_hotkey_footer() {
    let items = vec![HotkeyItem::new("j/k", "scroll")];
    let footer = HotkeyFooter::new(items)
        .key_color(Color::Yellow)
        .description_color(Color::White);

    let cloned = footer.clone();

    assert_eq!(cloned.items.len(), footer.items.len());
    assert_eq!(cloned.key_color, footer.key_color);
    assert_eq!(cloned.description_color, footer.description_color);
    assert_eq!(cloned.background_color, footer.background_color);
}

#[test]
fn test_special_characters_in_keys() {
    let items = vec![
        HotkeyItem::new("←/→", "navigate"),
        HotkeyItem::new("↑/↓", "scroll"),
        HotkeyItem::new("🔍", "search"),
    ];
    let footer = HotkeyFooter::new(items);

    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 1));
    footer.render(Rect::new(0, 0, 80, 1), &mut buffer);

    let content: String = (0..80)
        .map(|x| buffer[(x, 0)].symbol().chars().next().unwrap_or(' '))
        .collect();

    // Special characters should be rendered
    assert!(content.contains("navigate"));
    assert!(content.contains("scroll"));
    assert!(content.contains("search"));
}

#[test]
fn test_empty_key_and_description() {
    let items = vec![HotkeyItem::new("", "no key"), HotkeyItem::new("key", "")];
    let footer = HotkeyFooter::new(items);

    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 1));
    footer.render(Rect::new(0, 0, 80, 1), &mut buffer);

    // Should not panic, just render empty strings
    let cell = &buffer[(0, 0)];
    assert_eq!(cell.bg, Color::Black);
}

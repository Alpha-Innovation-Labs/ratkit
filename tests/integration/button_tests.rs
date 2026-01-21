use ratatui::layout::Rect;
use ratatui_toolkit::Button;

#[test]
fn test_button_render_with_title_click_area() {
    let mut button = Button::new("Open");
    let panel = Rect {
        x: 2,
        y: 4,
        width: 40,
        height: 1,
    };
    let title = " Click the button";

    let _ = button.render_with_title(panel, title);

    let expected_x = panel.x + title.len() as u16;
    let expected_y = panel.y;

    assert!(button.is_clicked(expected_x, expected_y));
    assert!(!button.is_clicked(expected_x.saturating_sub(1), expected_y));
}

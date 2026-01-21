use ratatui::layout::Rect;
use ratatui_toolkit::split_layout::SplitLayout;

#[test]
fn test_split_layout_creation() {
    let layout = SplitLayout::new(0);
    let panes = layout.layout_panes(Rect::new(0, 0, 100, 100));

    assert_eq!(panes.len(), 1);
    assert_eq!(panes[0].pane_id(), 0);
}

#[test]
fn test_split_layout_vertical_split() {
    let mut layout = SplitLayout::new(0);
    let pane_1 = layout.split_pane_vertically(0);

    assert!(pane_1.is_some());
    assert_eq!(pane_1.unwrap(), 1);

    let panes = layout.layout_panes(Rect::new(0, 0, 100, 100));
    assert_eq!(panes.len(), 2);

    assert_eq!(panes[0].area().width, 50);
    assert_eq!(panes[1].area().width, 50);
}

#[test]
fn test_split_layout_horizontal_split() {
    let mut layout = SplitLayout::new(0);
    let pane_1 = layout.split_pane_horizontally(0);

    assert!(pane_1.is_some());
    assert_eq!(pane_1.unwrap(), 1);

    let panes = layout.layout_panes(Rect::new(0, 0, 100, 100));
    assert_eq!(panes.len(), 2);

    assert_eq!(panes[0].area().height, 50);
    assert_eq!(panes[1].area().height, 50);
}

#[test]
fn test_split_layout_resize_divider() {
    let mut layout = SplitLayout::new(0);
    let _pane_1 = layout.split_pane_vertically(0).unwrap();

    let success = layout.resize_divider(0, 30);
    assert!(success);

    let panes = layout.layout_panes(Rect::new(0, 0, 100, 100));
    assert_eq!(panes[0].area().width, 30);
    assert_eq!(panes[1].area().width, 70);
}

#[test]
fn test_split_layout_resize_clamping() {
    let mut layout = SplitLayout::new(0);
    let _pane_1 = layout.split_pane_vertically(0).unwrap();

    let success = layout.resize_divider(0, 5);
    assert!(success);
    let panes = layout.layout_panes(Rect::new(0, 0, 100, 100));
    assert_eq!(panes[0].area().width, 10);

    let success = layout.resize_divider(0, 95);
    assert!(success);
    let panes = layout.layout_panes(Rect::new(0, 0, 100, 100));
    assert_eq!(panes[0].area().width, 90);
}

#[test]
fn test_split_layout_remove_pane() {
    let mut layout = SplitLayout::new(0);
    let pane_1 = layout.split_pane_vertically(0).unwrap();
    let pane_2 = layout.split_pane_horizontally(0).unwrap();

    let success = layout.remove_pane(pane_1.unwrap());
    assert!(success);

    let panes = layout.layout_panes(Rect::new(0, 0, 100, 100));
    assert_eq!(panes.len(), 2);
}

#[test]
fn test_split_layout_move_pane() {
    let mut layout = SplitLayout::new(0);
    let pane_1 = layout.split_pane_vertically(0).unwrap();
    let pane_2 = layout.split_pane_vertically(pane_1.unwrap()).unwrap();

    let success = layout.move_pane(0, pane_2.unwrap());
    assert!(success);

    let panes = layout.layout_panes(Rect::new(0, 0, 100, 100));
    let moved_pane = panes.iter().find(|p| p.pane_id() == 0);
    assert!(moved_pane.is_some());
}

#[test]
fn test_split_layout_five_pane_grid() {
    let mut layout = SplitLayout::new(0);
    let pane_2 = layout.split_pane_horizontally(0).unwrap();
    let pane_3 = layout.split_pane_vertically(pane_2).unwrap();
    let pane_4 = layout.split_pane_vertically(1).unwrap();

    let _ = layout.resize_divider(0, 60);
    let _ = layout.resize_divider(pane_2, 50);
    let _ = layout.resize_divider(1, 50);

    let panes = layout.layout_panes(Rect::new(0, 0, 120, 80));
    assert_eq!(panes.len(), 5);

    let ids: Vec<u32> = panes.iter().map(|p| p.pane_id()).collect();
    let unique_ids: std::collections::HashSet<u32> = ids.iter().cloned().collect();
    assert_eq!(unique_ids.len(), 5);

    for pane in &panes {
        let area = pane.area();
        assert!(area.width > 0);
        assert!(area.height > 0);
    }
}

#[test]
fn test_split_layout_multiple_splits() {
    let mut layout = SplitLayout::new(0);

    let pane_1 = layout.split_pane_vertically(0).unwrap();
    let pane_2 = layout.split_pane_horizontally(pane_1).unwrap();
    let pane_3 = layout.split_pane_horizontally(pane_2).unwrap();

    let panes = layout.layout_panes(Rect::new(0, 0, 100, 100));
    assert_eq!(panes.len(), 4);

    for i in 0..panes.len() {
        for j in (i + 1)..panes.len() {
            let area1 = panes[i].area();
            let area2 = panes[j].area();

            let no_overlap = area1.x + area1.width <= area2.x
                || area2.x + area2.width <= area1.x
                || area1.y + area1.height <= area2.y
                || area2.y + area2.height <= area1.y;

            assert!(no_overlap, "Panes {} and {} should not overlap", i, j);
        }
    }
}

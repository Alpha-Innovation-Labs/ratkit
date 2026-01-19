//! Pane layout strategies

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Layout strategy for arranging panes within a tab
#[derive(Debug, Clone)]
pub enum PaneLayout {
    /// Horizontal split with percentage widths
    /// Example: [50, 50] creates two panes side-by-side, each 50% width
    Horizontal(Vec<u16>),

    /// Vertical split with percentage heights
    /// Example: [30, 70] creates two panes stacked, 30% and 70% height
    Vertical(Vec<u16>),

    /// Grid layout with specified rows and columns
    /// Panes are filled left-to-right, top-to-bottom
    Grid { rows: usize, cols: usize },

    /// Custom layout function
    /// Takes available area and returns areas for each pane
    Custom(fn(Rect) -> Vec<Rect>),
}

impl PaneLayout {
    /// Calculate areas for the given number of panes
    pub fn calculate_areas(&self, available_area: Rect, pane_count: usize) -> Vec<Rect> {
        match self {
            PaneLayout::Horizontal(percentages) => {
                self.calculate_horizontal(available_area, percentages, pane_count)
            }
            PaneLayout::Vertical(percentages) => {
                self.calculate_vertical(available_area, percentages, pane_count)
            }
            PaneLayout::Grid { rows, cols } => {
                self.calculate_grid(available_area, *rows, *cols, pane_count)
            }
            PaneLayout::Custom(func) => func(available_area),
        }
    }

    fn calculate_horizontal(
        &self,
        available_area: Rect,
        percentages: &[u16],
        pane_count: usize,
    ) -> Vec<Rect> {
        // Use percentages if provided, otherwise split equally
        let constraints: Vec<Constraint> = if percentages.is_empty() {
            // Equal split
            let percent = 100 / pane_count.max(1) as u16;
            (0..pane_count)
                .map(|_| Constraint::Percentage(percent))
                .collect()
        } else {
            // Use provided percentages, pad with equal splits if needed
            let mut constraints = percentages
                .iter()
                .map(|&p| Constraint::Percentage(p))
                .collect::<Vec<_>>();

            // If we have more panes than percentages, split remaining space equally
            if pane_count > percentages.len() {
                let used: u16 = percentages.iter().sum();
                let remaining = 100_u16.saturating_sub(used);
                let additional_panes = pane_count - percentages.len();
                let each = remaining / additional_panes as u16;

                for _ in 0..additional_panes {
                    constraints.push(Constraint::Percentage(each));
                }
            }

            constraints
        };

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(available_area);

        chunks.iter().copied().take(pane_count).collect()
    }

    fn calculate_vertical(
        &self,
        available_area: Rect,
        percentages: &[u16],
        pane_count: usize,
    ) -> Vec<Rect> {
        let constraints: Vec<Constraint> = if percentages.is_empty() {
            let percent = 100 / pane_count.max(1) as u16;
            (0..pane_count)
                .map(|_| Constraint::Percentage(percent))
                .collect()
        } else {
            let mut constraints = percentages
                .iter()
                .map(|&p| Constraint::Percentage(p))
                .collect::<Vec<_>>();

            if pane_count > percentages.len() {
                let used: u16 = percentages.iter().sum();
                let remaining = 100_u16.saturating_sub(used);
                let additional_panes = pane_count - percentages.len();
                let each = remaining / additional_panes as u16;

                for _ in 0..additional_panes {
                    constraints.push(Constraint::Percentage(each));
                }
            }

            constraints
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(available_area);

        chunks.iter().copied().take(pane_count).collect()
    }

    fn calculate_grid(
        &self,
        available_area: Rect,
        rows: usize,
        cols: usize,
        pane_count: usize,
    ) -> Vec<Rect> {
        let mut areas = Vec::new();

        // Calculate cell size
        let cell_height = available_area.height / rows.max(1) as u16;
        let cell_width = available_area.width / cols.max(1) as u16;

        // Generate grid positions
        for i in 0..pane_count {
            let row = i / cols;
            let col = i % cols;

            if row >= rows {
                break; // Don't go beyond grid bounds
            }

            let x = available_area.x + (col as u16 * cell_width);
            let y = available_area.y + (row as u16 * cell_height);

            // Last column/row takes remaining space
            let width = if col == cols - 1 {
                available_area.width - (col as u16 * cell_width)
            } else {
                cell_width
            };

            let height = if row == rows - 1 {
                available_area.height - (row as u16 * cell_height)
            } else {
                cell_height
            };

            areas.push(Rect::new(x, y, width, height));
        }

        areas
    }
}

impl Default for PaneLayout {
    fn default() -> Self {
        // Default to equal horizontal split
        PaneLayout::Horizontal(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_equal_split() {
        let layout = PaneLayout::Horizontal(vec![]);
        let area = Rect::new(0, 0, 100, 50);
        let areas = layout.calculate_areas(area, 2);

        assert_eq!(areas.len(), 2);
        assert_eq!(areas[0].width, 50);
        assert_eq!(areas[1].width, 50);
    }

    #[test]
    fn test_horizontal_custom_percentages() {
        let layout = PaneLayout::Horizontal(vec![30, 70]);
        let area = Rect::new(0, 0, 100, 50);
        let areas = layout.calculate_areas(area, 2);

        assert_eq!(areas.len(), 2);
        // Note: Layout may not be exact due to rounding
        assert!(areas[0].width <= 30);
        assert!(areas[1].width >= 70);
    }

    #[test]
    fn test_vertical_equal_split() {
        let layout = PaneLayout::Vertical(vec![]);
        let area = Rect::new(0, 0, 100, 100);
        let areas = layout.calculate_areas(area, 2);

        assert_eq!(areas.len(), 2);
        assert_eq!(areas[0].height, 50);
        assert_eq!(areas[1].height, 50);
    }

    #[test]
    fn test_vertical_custom_percentages() {
        let layout = PaneLayout::Vertical(vec![25, 75]);
        let area = Rect::new(0, 0, 100, 100);
        let areas = layout.calculate_areas(area, 2);

        assert_eq!(areas.len(), 2);
        assert!(areas[0].height <= 25);
        assert!(areas[1].height >= 75);
    }

    #[test]
    fn test_grid_2x2() {
        let layout = PaneLayout::Grid { rows: 2, cols: 2 };
        let area = Rect::new(0, 0, 100, 100);
        let areas = layout.calculate_areas(area, 4);

        assert_eq!(areas.len(), 4);

        // Check positions
        assert_eq!(areas[0].x, 0);
        assert_eq!(areas[0].y, 0);

        assert_eq!(areas[1].x, 50);
        assert_eq!(areas[1].y, 0);

        assert_eq!(areas[2].x, 0);
        assert_eq!(areas[2].y, 50);

        assert_eq!(areas[3].x, 50);
        assert_eq!(areas[3].y, 50);
    }

    #[test]
    fn test_grid_incomplete() {
        let layout = PaneLayout::Grid { rows: 2, cols: 2 };
        let area = Rect::new(0, 0, 100, 100);
        let areas = layout.calculate_areas(area, 3);

        // Should only create 3 panes
        assert_eq!(areas.len(), 3);
    }

    #[test]
    fn test_grid_overflow() {
        let layout = PaneLayout::Grid { rows: 2, cols: 2 };
        let area = Rect::new(0, 0, 100, 100);
        let areas = layout.calculate_areas(area, 10);

        // Should only create 4 panes (2x2 grid max)
        assert_eq!(areas.len(), 4);
    }

    #[test]
    fn test_custom_layout() {
        fn custom_layout(area: Rect) -> Vec<Rect> {
            vec![
                Rect::new(area.x, area.y, area.width / 3, area.height),
                Rect::new(
                    area.x + area.width / 3,
                    area.y,
                    area.width * 2 / 3,
                    area.height,
                ),
            ]
        }

        let layout = PaneLayout::Custom(custom_layout);
        let area = Rect::new(0, 0, 90, 50);
        let areas = layout.calculate_areas(area, 2);

        assert_eq!(areas.len(), 2);
        assert_eq!(areas[0].width, 30);
        assert_eq!(areas[1].width, 60);
    }

    #[test]
    fn test_horizontal_more_panes_than_percentages() {
        let layout = PaneLayout::Horizontal(vec![40]);
        let area = Rect::new(0, 0, 100, 50);
        let areas = layout.calculate_areas(area, 3);

        assert_eq!(areas.len(), 3);
        // First pane gets 40%, remaining 60% split between 2 panes
    }

    #[test]
    fn test_vertical_more_panes_than_percentages() {
        let layout = PaneLayout::Vertical(vec![30]);
        let area = Rect::new(0, 0, 100, 100);
        let areas = layout.calculate_areas(area, 3);

        assert_eq!(areas.len(), 3);
        // First pane gets 30%, remaining 70% split between 2 panes
    }

    #[test]
    fn test_default_layout() {
        let layout = PaneLayout::default();
        let area = Rect::new(0, 0, 100, 50);
        let areas = layout.calculate_areas(area, 3);

        // Default is horizontal equal split
        assert_eq!(areas.len(), 3);
        // Each pane should get approximately 33% of the width
        for area in areas {
            assert!(area.width > 0);
        }
    }
}

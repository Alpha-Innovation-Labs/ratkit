use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn get_selection(&self) -> Option<String> {
        self.terminal.as_ref().and_then(|terminal| {
            let parser = terminal.parser.lock().unwrap();
            let screen = parser.screen();
            let size = screen.size();
            let rows = size.rows as usize;

            if rows == 0 {
                return None;
            }

            let last_row = screen.primary_grid().visible_row((rows - 1) as u16)?;
            let cols = last_row.width();

            if cols == 0 {
                return None;
            }

            let mut result = String::new();
            for col in 0..cols {
                if let Some(cell) = last_row.get(col) {
                    result.push_str(cell.text());
                }
            }

            if result.is_empty() {
                None
            } else {
                Some(result.trim().to_string())
            }
        })
    }
}

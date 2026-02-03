use crate::theme_colors::ThemeColors;

#[derive(Debug, Clone)]
pub struct ThemePickerState {
    visible: bool,
    index: usize,
    filter: String,
    current_preview: ThemeColors,
    saved_index: usize,
    original_index: Option<usize>,
}

impl ThemePickerState {
    pub fn new() -> Self {
        Self {
            visible: false,
            index: 0,
            filter: String::new(),
            current_preview: ThemeColors::default(),
            saved_index: 0,
            original_index: None,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
        self.original_index = Some(self.saved_index);
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn filter(&self) -> &str {
        &self.filter
    }

    pub fn push_filter(&mut self, c: char) {
        self.filter.push(c);
    }

    pub fn pop_filter(&mut self) {
        self.filter.pop();
    }

    pub fn clear_filter(&mut self) {
        self.filter.clear();
    }

    pub fn current_preview(&self) -> &ThemeColors {
        &self.current_preview
    }

    pub fn set_current_preview(&mut self, theme: ThemeColors) {
        self.current_preview = theme;
    }

    pub fn saved_index(&self) -> usize {
        self.saved_index
    }

    pub fn set_saved_index(&mut self, index: usize) {
        self.saved_index = index;
    }

    pub fn restore_original(&mut self) {
        if let Some(original) = self.original_index.take() {
            self.saved_index = original;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ThemePickerStateSnapshot {
    visible: bool,
    index: usize,
    filter: String,
    saved_index: usize,
}

impl From<&ThemePickerState> for ThemePickerStateSnapshot {
    fn from(state: &ThemePickerState) -> Self {
        Self {
            visible: state.visible,
            index: state.index,
            filter: state.filter.clone(),
            saved_index: state.saved_index,
        }
    }
}

impl ThemePickerStateSnapshot {
    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn filter(&self) -> &str {
        &self.filter
    }

    pub fn saved_index(&self) -> usize {
        self.saved_index
    }
}

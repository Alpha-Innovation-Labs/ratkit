use ratatui::style::{Color, Modifier, Style};

use crate::menu_bar::{MenuBar, MenuItem};

impl MenuItem {
    pub fn new(name: impl Into<String>, value: usize) -> Self {
        Self {
            name: name.into(),
            icon: None,
            value,
            selected: false,
            hovered: false,
            area: None,
        }
    }

    pub fn with_icon(name: impl Into<String>, icon: impl Into<String>, value: usize) -> Self {
        Self {
            name: name.into(),
            icon: Some(icon.into()),
            value,
            selected: false,
            hovered: false,
            area: None,
        }
    }

    pub fn display_label(&self) -> String {
        if let Some(ref icon) = self.icon {
            format!("{} {}", icon, self.name)
        } else {
            self.name.clone()
        }
    }
}

impl MenuBar {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self {
            items,
            area: None,
            normal_style: Style::default().fg(Color::White),
            selected_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            hover_style: Style::default().fg(Color::Cyan),
            selected_hover_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        }
    }

    pub fn with_selected(mut self, index: usize) -> Self {
        if index < self.items.len() {
            self.items[index].selected = true;
        }
        self
    }

    pub fn normal_style(mut self, style: Style) -> Self {
        self.normal_style = style;
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    pub fn hover_style(mut self, style: Style) -> Self {
        self.hover_style = style;
        self
    }

    pub fn selected_hover_style(mut self, style: Style) -> Self {
        self.selected_hover_style = style;
        self
    }
}

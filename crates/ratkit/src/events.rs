//! Event types and routing for UI interactions.

use crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEvent as CrosstermMouseEvent,
    MouseEventKind,
};
use ratatui::layout::Rect;
use std::fmt;

/// A keyboard input event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardEvent {
    pub key_code: KeyCode,
    pub modifiers: KeyModifiers,
    pub kind: KeyEventKind,
}

impl KeyboardEvent {
    pub fn from_crossterm(event: KeyEvent) -> Self {
        Self {
            key_code: event.code,
            modifiers: event.modifiers,
            kind: event.kind,
        }
    }

    pub fn is_key_down(&self) -> bool {
        matches!(self.kind, KeyEventKind::Press | KeyEventKind::Repeat)
    }

    pub fn is_key_up(&self) -> bool {
        matches!(self.kind, KeyEventKind::Release)
    }

    pub fn is_char(&self, c: char) -> bool {
        matches!(self.key_code, KeyCode::Char(ch) if ch == c)
    }

    pub fn is_code(&self, code: KeyCode) -> bool {
        self.key_code == code
    }

    pub fn has_modifier(&self, modifier: KeyModifiers) -> bool {
        self.modifiers.contains(modifier)
    }

    pub fn is_enter(&self) -> bool {
        self.is_code(KeyCode::Enter)
    }

    pub fn is_escape(&self) -> bool {
        self.is_code(KeyCode::Esc)
    }

    pub fn is_tab(&self) -> bool {
        self.is_code(KeyCode::Tab)
    }

    pub fn is_backtab(&self) -> bool {
        self.is_code(KeyCode::BackTab)
    }

    pub fn is_backspace(&self) -> bool {
        self.is_code(KeyCode::Backspace)
    }

    pub fn is_delete(&self) -> bool {
        self.is_code(KeyCode::Delete)
    }

    pub fn is_space(&self) -> bool {
        self.is_char(' ')
    }
}

impl fmt::Display for KeyboardEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let modifiers_str = if self.modifiers.is_empty() {
            String::new()
        } else {
            format!("{:?}+", self.modifiers)
        };
        write!(f, "{}{:?}", modifiers_str, self.key_code)
    }
}

/// A mouse input event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: KeyModifiers,
}

impl MouseEvent {
    pub fn from_crossterm(event: CrosstermMouseEvent) -> Self {
        Self {
            kind: event.kind,
            column: event.column,
            row: event.row,
            modifiers: event.modifiers,
        }
    }

    pub fn position(&self) -> (u16, u16) {
        (self.column, self.row)
    }

    pub fn x(&self) -> u16 {
        self.column
    }

    pub fn y(&self) -> u16 {
        self.row
    }

    pub fn is_inside(&self, rect: Rect) -> bool {
        self.column >= rect.x
            && self.column < rect.x + rect.width
            && self.row >= rect.y
            && self.row < rect.y + rect.height
    }

    pub fn is_click(&self) -> bool {
        matches!(
            self.kind,
            MouseEventKind::Down(crossterm::event::MouseButton::Left)
                | MouseEventKind::Down(crossterm::event::MouseButton::Right)
                | MouseEventKind::Down(crossterm::event::MouseButton::Middle)
        )
    }

    pub fn is_drag(&self) -> bool {
        matches!(
            self.kind,
            MouseEventKind::Drag(crossterm::event::MouseButton::Left)
                | MouseEventKind::Drag(crossterm::event::MouseButton::Right)
                | MouseEventKind::Drag(crossterm::event::MouseButton::Middle)
        )
    }

    pub fn is_scroll(&self) -> bool {
        matches!(
            self.kind,
            MouseEventKind::ScrollUp
                | MouseEventKind::ScrollDown
                | MouseEventKind::ScrollLeft
                | MouseEventKind::ScrollRight
        )
    }
}

impl fmt::Display for MouseEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at ({}, {})", self.kind, self.column, self.row)
    }
}

/// A periodic tick event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickEvent {
    pub count: u64,
}

impl TickEvent {
    pub fn new(count: u64) -> Self {
        Self { count }
    }
}

/// A terminal resize event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResizeEvent {
    pub width: u16,
    pub height: u16,
}

impl ResizeEvent {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }
}

/// Unified event type for the runner.
#[derive(Debug, Clone)]
pub enum RunnerEvent {
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
    Tick(TickEvent),
    Resize(ResizeEvent),
}

impl RunnerEvent {
    pub fn is_keyboard(&self) -> bool {
        matches!(self, RunnerEvent::Keyboard(_))
    }

    pub fn is_mouse(&self) -> bool {
        matches!(self, RunnerEvent::Mouse(_))
    }

    pub fn is_tick(&self) -> bool {
        matches!(self, RunnerEvent::Tick(_))
    }

    pub fn is_resize(&self) -> bool {
        matches!(self, RunnerEvent::Resize(_))
    }

    pub fn as_keyboard(&self) -> Option<&KeyboardEvent> {
        match self {
            RunnerEvent::Keyboard(event) => Some(event),
            _ => None,
        }
    }

    pub fn as_mouse(&self) -> Option<&MouseEvent> {
        match self {
            RunnerEvent::Mouse(event) => Some(event),
            _ => None,
        }
    }

    pub fn as_tick(&self) -> Option<&TickEvent> {
        match self {
            RunnerEvent::Tick(event) => Some(event),
            _ => None,
        }
    }

    pub fn as_resize(&self) -> Option<&ResizeEvent> {
        match self {
            RunnerEvent::Resize(event) => Some(event),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_event() {
        let event = KeyboardEvent {
            key_code: KeyCode::Char('a'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
        };

        assert!(event.is_key_down());
        assert!(!event.is_key_up());
        assert!(event.is_char('a'));
        assert!(!event.is_char('b'));
        assert!(event.has_modifier(KeyModifiers::CONTROL));
    }

    #[test]
    fn test_mouse_event() {
        let event = MouseEvent {
            kind: MouseEventKind::Down(crossterm::event::MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::empty(),
        };

        assert_eq!(event.position(), (10, 5));
        assert_eq!(event.x(), 10);
        assert_eq!(event.y(), 5);
        assert!(event.is_click());
        assert!(!event.is_drag());
        assert!(!event.is_scroll());
    }

    #[test]
    fn test_mouse_inside_rect() {
        let event = MouseEvent {
            kind: MouseEventKind::Down(crossterm::event::MouseButton::Left),
            column: 5,
            row: 3,
            modifiers: KeyModifiers::empty(),
        };

        let rect = Rect::new(0, 0, 10, 10);
        assert!(event.is_inside(rect));

        let rect2 = Rect::new(10, 0, 10, 10);
        assert!(!event.is_inside(rect2));
    }

    #[test]
    fn test_resize_event() {
        let event = ResizeEvent::new(80, 24);
        assert_eq!(event.width, 80);
        assert_eq!(event.height, 24);
        assert_eq!(event.area(), Rect::new(0, 0, 80, 24));
    }

    #[test]
    fn test_runner_event_dispatch() {
        let keyboard = RunnerEvent::Keyboard(KeyboardEvent {
            key_code: KeyCode::Enter,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
        });

        assert!(keyboard.is_keyboard());
        assert!(!keyboard.is_mouse());
        assert!(keyboard.as_keyboard().is_some());
        assert!(keyboard.as_mouse().is_none());
    }
}

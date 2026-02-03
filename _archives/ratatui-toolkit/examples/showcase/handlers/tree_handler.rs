//! Tree tab handler.

use super::TabHandler;
use crate::app::{App, TreePaneFocus};
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};

pub struct TreeHandler;

impl TabHandler for TreeHandler {
    fn handle_key(&mut self, app: &mut App, key: KeyEvent) {
        match app.tree_focus {
            TreePaneFocus::FileTree => {
                if key.code == KeyCode::Char('c') {
                    app.tree_focus = TreePaneFocus::ComponentTree;
                    return;
                }

                if let Some(ref mut file_tree) = app.file_tree {
                    file_tree.handle_key(key.code);
                }
            }
            TreePaneFocus::ComponentTree => {
                if key.code == KeyCode::Char('f') {
                    app.tree_focus = TreePaneFocus::FileTree;
                    return;
                }

                app.component_tree.handle_key(key.code);
            }
        }
    }

    fn handle_mouse(&mut self, app: &mut App, mouse: MouseEvent) {
        match app.tree_focus {
            TreePaneFocus::FileTree => {
                if let Some(ref mut file_tree) = app.file_tree {
                    file_tree.handle_mouse(mouse);
                }
            }
            TreePaneFocus::ComponentTree => {
                app.component_tree.handle_mouse(mouse);
            }
        }
    }

    fn needs_fast_refresh(&self, _app: &App) -> bool {
        false
    }
}

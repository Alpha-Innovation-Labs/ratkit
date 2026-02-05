//! Focus management with focus stack and traversal.

use std::collections::VecDeque;
use tracing::{debug, trace};

use crate::error::{LayoutError, LayoutResult};
use crate::registry::ElementRegistry;
use crate::types::ElementId;

/// Focus change request type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusRequest {
    /// Move focus to a specific element.
    To(ElementId),
    /// Move focus to the next element (forward).
    Next,
    /// Move focus to the previous element (backward).
    Previous,
    /// Move focus to the first element.
    First,
    /// Move focus to the last element.
    Last,
    /// Release focus (no element focused).
    Release,
}

/// Focus manager for managing element focus state.
#[derive(Debug)]
pub struct FocusManager {
    registry: ElementRegistry,
    focus_stack: VecDeque<ElementId>,
    captured_by: Option<ElementId>,
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FocusManager {
    pub fn new() -> Self {
        Self {
            registry: ElementRegistry::new(),
            focus_stack: VecDeque::new(),
            captured_by: None,
        }
    }

    pub fn registry(&self) -> &ElementRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut ElementRegistry {
        &mut self.registry
    }

    /// Get the currently focused element (top of focus stack).
    pub fn focused(&self) -> Option<ElementId> {
        if let Some(capturer) = self.captured_by {
            Some(capturer)
        } else {
            self.focus_stack.front().copied()
        }
    }

    /// Check if an element is focused.
    pub fn is_focused(&self, id: ElementId) -> bool {
        self.focused() == Some(id)
    }

    /// Get the element that has captured focus.
    pub fn captured_by(&self) -> Option<ElementId> {
        self.captured_by
    }

    /// Capture focus for an element (e.g., modals, dialogs).
    pub fn capture_focus(&mut self, id: ElementId) -> LayoutResult<()> {
        self.registry.get_strong_ref(id)?;

        debug!("Focus captured by element: {}", id);
        self.captured_by = Some(id);

        Ok(())
    }

    /// Release focus capture.
    pub fn release_capture(&mut self) -> LayoutResult<()> {
        if let Some(id) = self.captured_by {
            debug!("Focus capture released from element: {}", id);
            self.captured_by = None;
        }

        Ok(())
    }

    /// Process a focus change request.
    pub fn handle_request(&mut self, request: FocusRequest) -> LayoutResult<Option<ElementId>> {
        let previous = self.focused();

        match request {
            FocusRequest::To(id) => {
                self.focus_to(id)?;
            }
            FocusRequest::Next => {
                self.focus_next()?;
            }
            FocusRequest::Previous => {
                self.focus_previous()?;
            }
            FocusRequest::First => {
                self.focus_first()?;
            }
            FocusRequest::Last => {
                self.focus_last()?;
            }
            FocusRequest::Release => {
                self.release_focus()?;
            }
        }

        let current = self.focused();

        if previous != current {
            self.notify_focus_change(previous, current)?;
        }

        Ok(current)
    }

    fn focus_to(&mut self, id: ElementId) -> LayoutResult<()> {
        self.registry.get_strong_ref(id)?;

        if let Some(index) = self.focus_stack.iter().position(|&x| x == id) {
            self.focus_stack.remove(index);
        }

        self.focus_stack.push_front(id);

        debug!("Focus moved to element: {}", id);

        Ok(())
    }

    fn focus_next(&mut self) -> LayoutResult<()> {
        let focusable = self.registry.focusable_elements();

        if focusable.is_empty() {
            return Err(LayoutError::focus("No focusable elements"));
        }

        let current = self.focused();

        let current_index = if let Some(id) = current {
            focusable.iter().position(|(elem_id, _)| elem_id == &id)
        } else {
            None
        };

        let next_index = match current_index {
            Some(index) => (index + 1) % focusable.len(),
            None => 0,
        };

        let next_id = focusable[next_index].0;
        self.focus_to(next_id)
    }

    fn focus_previous(&mut self) -> LayoutResult<()> {
        let focusable = self.registry.focusable_elements();

        if focusable.is_empty() {
            return Err(LayoutError::focus("No focusable elements"));
        }

        let current = self.focused();

        let current_index = if let Some(id) = current {
            focusable.iter().position(|(elem_id, _)| elem_id == &id)
        } else {
            None
        };

        let prev_index = match current_index {
            Some(index) => {
                if index == 0 {
                    focusable.len() - 1
                } else {
                    index - 1
                }
            }
            None => focusable.len() - 1,
        };

        let prev_id = focusable[prev_index].0;
        self.focus_to(prev_id)
    }

    fn focus_first(&mut self) -> LayoutResult<()> {
        let focusable = self.registry.focusable_elements();

        if focusable.is_empty() {
            return Err(LayoutError::focus("No focusable elements"));
        }

        let first_id = focusable.first().map(|(id, _)| *id).unwrap();
        self.focus_to(first_id)
    }

    fn focus_last(&mut self) -> LayoutResult<()> {
        let focusable = self.registry.focusable_elements();

        if focusable.is_empty() {
            return Err(LayoutError::focus("No focusable elements"));
        }

        let last_id = focusable.last().map(|(id, _)| *id).unwrap();
        self.focus_to(last_id)
    }

    fn release_focus(&mut self) -> LayoutResult<()> {
        let previous = self.focus_stack.pop_front();

        debug!("Focus released: previous = {:?}", previous);

        if !self.focus_stack.is_empty() {
            debug!(
                "Focused on next element in stack: {:?}",
                self.focus_stack.front()
            );
        } else if !self.registry.focusable_elements().is_empty() {
            let focusable = self.registry.focusable_elements();
            let next_id = focusable.first().map(|(id, _)| *id).unwrap();
            self.focus_stack.push_front(next_id);
            debug!("Auto-focused on next element: {}", next_id);
        }

        Ok(())
    }

    fn notify_focus_change(
        &mut self,
        previous: Option<ElementId>,
        current: Option<ElementId>,
    ) -> LayoutResult<()> {
        if let Some(id) = previous {
            if let Ok(element) = self.registry.get_strong_ref(id) {
                element.on_focus_loss();
            }
        }

        if let Some(id) = current {
            if let Ok(element) = self.registry.get_strong_ref(id) {
                element.on_focus_gain();
            }
        }

        trace!("Focus change notified: {:?} -> {:?}", previous, current);

        Ok(())
    }

    /// Remove an element from the focus stack and restore fallback focus.
    pub fn remove_element(&mut self, id: ElementId) -> LayoutResult<()> {
        if self.captured_by == Some(id) {
            self.captured_by = None;
        }

        let was_focused = self.focus_stack.front() == Some(&id);

        self.focus_stack.retain(|&x| x != id);

        if was_focused {
            debug!("Focused element removed, restoring fallback focus: {}", id);

            if !self.focus_stack.is_empty() {
                let next_id = self.focus_stack.front().copied().unwrap();
                self.notify_focus_change(Some(id), Some(next_id))?;
            } else {
                self.notify_focus_change(Some(id), None)?;
            }
        }

        Ok(())
    }

    /// Rebuild focus stack from current focusable elements.
    pub fn rebuild_focus_stack(&mut self) -> LayoutResult<()> {
        let focusable = self.registry.focusable_elements();
        let current = self.focused();

        self.focus_stack.clear();

        for (id, _) in focusable {
            self.focus_stack.push_back(id);
        }

        debug!("Focus stack rebuilt: {} elements", self.focus_stack.len());

        if let Some(id) = current {
            if self.focus_stack.contains(&id) {
                if let Some(index) = self.focus_stack.iter().position(|&x| x == id) {
                    let id = self.focus_stack.remove(index).unwrap();
                    self.focus_stack.push_front(id);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ElementMetadata, Region};
    use std::sync::Arc;

    fn create_test_manager() -> FocusManager {
        FocusManager::new()
    }

    fn register_focusable(
        manager: &mut FocusManager,
        id: ElementId,
        region: Region,
    ) -> Arc<DummyElement> {
        let metadata = crate::types::ElementMetadata::new(id, region).with_focusable(true);

        let element = Arc::new(DummyElement::new(id));
        let _ = manager.registry_mut().register(metadata, element.clone());
        element
    }

    #[test]
    fn test_focus_manager_init() {
        let manager = create_test_manager();
        assert!(manager.focused().is_none());
    }

    #[test]
    fn test_focus_to() {
        let mut manager = create_test_manager();
        let id = ElementId::new();
        let _element = register_focusable(&mut manager, id, Region::Center);

        let result = manager.handle_request(FocusRequest::To(id));
        assert!(result.is_ok());
        assert_eq!(manager.focused(), Some(id));
    }

    #[test]
    fn test_focus_to_nonexistent() {
        let mut manager = create_test_manager();
        let id = ElementId::new();

        let result = manager.handle_request(FocusRequest::To(id));
        assert!(result.is_err());
    }

    #[test]
    fn test_focus_next() {
        let mut manager = create_test_manager();
        let id1 = ElementId::new();
        let id2 = ElementId::new();
        let id3 = ElementId::new();

        let _e1 = register_focusable(&mut manager, id1, Region::Top);
        let _e2 = register_focusable(&mut manager, id2, Region::Center);
        let _e3 = register_focusable(&mut manager, id3, Region::Bottom);

        manager.handle_request(FocusRequest::First).unwrap();
        assert_eq!(manager.focused(), Some(id1));

        manager.handle_request(FocusRequest::Next).unwrap();
        assert_eq!(manager.focused(), Some(id2));

        manager.handle_request(FocusRequest::Next).unwrap();
        assert_eq!(manager.focused(), Some(id3));

        manager.handle_request(FocusRequest::Next).unwrap();
        assert_eq!(manager.focused(), Some(id1));
    }

    #[test]
    fn test_focus_previous() {
        let mut manager = create_test_manager();
        let id1 = ElementId::new();
        let id2 = ElementId::new();
        let id3 = ElementId::new();

        let _e1 = register_focusable(&mut manager, id1, Region::Top);
        let _e2 = register_focusable(&mut manager, id2, Region::Center);
        let _e3 = register_focusable(&mut manager, id3, Region::Bottom);

        manager.handle_request(FocusRequest::Last).unwrap();
        assert_eq!(manager.focused(), Some(id3));

        manager.handle_request(FocusRequest::Previous).unwrap();
        assert_eq!(manager.focused(), Some(id2));

        manager.handle_request(FocusRequest::Previous).unwrap();
        assert_eq!(manager.focused(), Some(id1));

        manager.handle_request(FocusRequest::Previous).unwrap();
        assert_eq!(manager.focused(), Some(id3));
    }

    #[test]
    fn test_focus_first_last() {
        let mut manager = create_test_manager();
        let id1 = ElementId::new();
        let id2 = ElementId::new();

        let _e1 = register_focusable(&mut manager, id1, Region::Top);
        let _e2 = register_focusable(&mut manager, id2, Region::Bottom);

        manager.handle_request(FocusRequest::First).unwrap();
        assert_eq!(manager.focused(), Some(id1));

        manager.handle_request(FocusRequest::Last).unwrap();
        assert_eq!(manager.focused(), Some(id2));
    }

    #[test]
    fn test_focus_release() {
        let mut manager = create_test_manager();
        let id1 = ElementId::new();
        let id2 = ElementId::new();

        let _e1 = register_focusable(&mut manager, id1, Region::Top);
        let _e2 = register_focusable(&mut manager, id2, Region::Bottom);

        manager.handle_request(FocusRequest::To(id1)).unwrap();
        manager.handle_request(FocusRequest::Release).unwrap();

        assert!(manager.focused().is_some());
    }

    #[test]
    fn test_focus_capture() {
        let mut manager = create_test_manager();
        let id = ElementId::new();
        let _element = register_focusable(&mut manager, id, Region::Center);

        manager.capture_focus(id).unwrap();
        assert_eq!(manager.captured_by(), Some(id));
        assert_eq!(manager.focused(), Some(id));

        manager.release_capture().unwrap();
        assert!(manager.captured_by().is_none());
    }

    #[test]
    fn test_remove_element() {
        let mut manager = create_test_manager();
        let id1 = ElementId::new();
        let id2 = ElementId::new();
        let id3 = ElementId::new();

        let _e1 = register_focusable(&mut manager, id1, Region::Top);
        let e2 = register_focusable(&mut manager, id2, Region::Center);
        let _e3 = register_focusable(&mut manager, id3, Region::Bottom);

        manager.handle_request(FocusRequest::To(id1)).unwrap();
        manager.handle_request(FocusRequest::To(id2)).unwrap();
        drop(e2);
        manager.remove_element(id2).unwrap();

        assert_eq!(manager.focused(), Some(id1));
    }

    #[test]
    fn test_rebuild_focus_stack() {
        let mut manager = create_test_manager();
        let id1 = ElementId::new();
        let id2 = ElementId::new();

        let _e1 = register_focusable(&mut manager, id1, Region::Top);
        let _e2 = register_focusable(&mut manager, id2, Region::Bottom);

        manager.handle_request(FocusRequest::To(id1)).unwrap();
        manager.rebuild_focus_stack().unwrap();

        assert_eq!(manager.focused(), Some(id1));
    }

    struct DummyElement {
        id: ElementId,
    }

    impl DummyElement {
        fn new(id: ElementId) -> Self {
            Self { id }
        }
    }

    impl crate::registry::Element for DummyElement {
        fn id(&self) -> ElementId {
            self.id
        }

        fn on_metadata_update(&self, _metadata: &crate::types::ElementMetadata) {}

        fn on_render(&self) {}

        fn on_keyboard(&self, _event: &crate::events::KeyboardEvent) -> bool {
            false
        }

        fn on_mouse(&self, _event: &crate::events::MouseEvent) -> bool {
            false
        }

        fn on_focus_gain(&self) {}

        fn on_focus_loss(&self) {}

        fn on_tick(&self) {}
    }
}

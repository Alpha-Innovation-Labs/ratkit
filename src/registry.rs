//! Element registry for managing UI elements.

use std::collections::HashMap;
use std::sync::{Arc, Weak};
use tracing::{debug, trace};

use crate::error::{LayoutError, LayoutResult};
use crate::types::{ElementId, ElementMetadata, Region, Visibility};

/// Weak reference to an element.
pub type ElementWeakRef = Weak<dyn Element>;

/// Strong reference to an element.
pub type ElementRef = Arc<dyn Element>;

/// Handle to a registered element.
#[derive(Debug, Clone)]
pub struct ElementHandle {
    id: ElementId,
    weak_ref: ElementWeakRef,
}

impl ElementHandle {
    pub fn new(id: ElementId, weak_ref: ElementWeakRef) -> Self {
        Self { id, weak_ref }
    }

    pub fn id(&self) -> ElementId {
        self.id
    }

    pub fn upgrade(&self) -> Option<ElementRef> {
        self.weak_ref.upgrade()
    }

    pub fn is_alive(&self) -> bool {
        self.weak_ref.strong_count() > 0
    }
}

/// Trait for elements that can be registered with the layout manager.
pub trait Element: Send + Sync {
    /// Returns the element's unique identifier.
    fn id(&self) -> ElementId;

    /// Called when the element's metadata is updated.
    fn on_metadata_update(&self, metadata: &ElementMetadata);

    /// Called when the element should render itself.
    fn on_render(&self);

    /// Called when the element receives keyboard input.
    fn on_keyboard(&self, event: &KeyboardEvent) -> bool;

    /// Called when the element receives mouse input.
    fn on_mouse(&self, event: &MouseEvent) -> bool;

    /// Called when the element receives focus.
    fn on_focus_gain(&self);

    /// Called when the element loses focus.
    fn on_focus_loss(&self);

    /// Called on each tick event.
    fn on_tick(&self);
}

/// Registry for managing UI elements with weak references.
#[derive(Debug)]
pub struct ElementRegistry {
    elements: HashMap<ElementId, (ElementMetadata, ElementWeakRef)>,
}

impl Default for ElementRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ElementRegistry {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    /// Register a new element with the registry.
    pub fn register(
        &mut self,
        metadata: ElementMetadata,
        element: ElementRef,
    ) -> LayoutResult<ElementHandle> {
        let id = metadata.id;

        if self.elements.contains_key(&id) {
            return Err(LayoutError::element_already_registered(id));
        }

        let weak_ref = Arc::downgrade(&element) as ElementWeakRef;
        let handle_ref = weak_ref.clone();
        self.elements.insert(id, (metadata, weak_ref));

        debug!("Registered element: {}", id);

        Ok(ElementHandle::new(id, handle_ref))
    }

    /// Unregister an element from the registry.
    pub fn unregister(&mut self, id: ElementId) -> LayoutResult<()> {
        if self.elements.remove(&id).is_none() {
            return Err(LayoutError::element_not_found(id));
        }

        debug!("Unregistered element: {}", id);

        Ok(())
    }

    /// Get metadata for an element.
    pub fn get_metadata(&self, id: ElementId) -> LayoutResult<&ElementMetadata> {
        self.elements
            .get(&id)
            .map(|(metadata, _)| metadata)
            .ok_or_else(|| LayoutError::element_not_found(id))
    }

    /// Get mutable metadata for an element.
    pub fn get_metadata_mut(&mut self, id: ElementId) -> LayoutResult<&mut ElementMetadata> {
        self.elements
            .get_mut(&id)
            .map(|(metadata, _)| metadata)
            .ok_or_else(|| LayoutError::element_not_found(id))
    }

    /// Get a weak reference to an element.
    pub fn get_weak_ref(&self, id: ElementId) -> LayoutResult<ElementWeakRef> {
        self.elements
            .get(&id)
            .map(|(_, weak_ref)| weak_ref.clone())
            .ok_or_else(|| LayoutError::element_not_found(id))
    }

    /// Get a strong reference to an element (if still alive).
    pub fn get_strong_ref(&self, id: ElementId) -> LayoutResult<ElementRef> {
        self.get_weak_ref(id)?
            .upgrade()
            .ok_or_else(|| LayoutError::element_not_found(id))
    }

    /// Update element metadata and notify the element.
    pub fn update_metadata(
        &mut self,
        id: ElementId,
        mut update: impl FnMut(&mut ElementMetadata),
    ) -> LayoutResult<()> {
        let (metadata, weak_ref) = self
            .elements
            .get_mut(&id)
            .ok_or_else(|| LayoutError::element_not_found(id))?;

        update(metadata);

        if let Some(strong_ref) = weak_ref.upgrade() {
            strong_ref.on_metadata_update(metadata);
        }

        trace!("Updated metadata for element: {}", id);

        Ok(())
    }

    /// Set element visibility.
    pub fn set_visibility(&mut self, id: ElementId, visibility: Visibility) -> LayoutResult<()> {
        self.update_metadata(id, |metadata| {
            metadata.visibility = visibility;
        })
    }

    /// Set element z-order.
    pub fn set_z_order(&mut self, id: ElementId, z_order: u32) -> LayoutResult<()> {
        self.update_metadata(id, |metadata| {
            metadata.z_order = z_order;
        })
    }

    /// Get all registered element IDs.
    pub fn all_ids(&self) -> Vec<ElementId> {
        self.elements.keys().copied().collect()
    }

    /// Get elements for a specific region, sorted by z-order (highest first).
    pub fn elements_by_region(&self, region: Region) -> Vec<(ElementId, ElementMetadata)> {
        let mut elements: Vec<_> = self
            .elements
            .iter()
            .filter(|(_, (metadata, _))| metadata.region == region && metadata.is_visible())
            .map(|(id, (metadata, _))| (*id, metadata.clone()))
            .collect();

        elements.sort_by(|a, b| b.1.z_order.cmp(&a.1.z_order));
        elements
    }

    /// Get focusable elements sorted by region and z-order.
    pub fn focusable_elements(&self) -> Vec<(ElementId, ElementMetadata)> {
        let mut elements: Vec<_> = self
            .elements
            .iter()
            .filter(|(_, (metadata, _))| metadata.can_receive_focus())
            .map(|(id, (metadata, _))| (*id, metadata.clone()))
            .collect();

        elements.sort_by(|a, b| {
            (a.1.region as u32)
                .cmp(&(b.1.region as u32))
                .then(b.1.z_order.cmp(&a.1.z_order))
        });
        elements
    }

    /// Clean up dead weak references.
    pub fn cleanup_dead_refs(&mut self) -> usize {
        let initial_count = self.elements.len();

        self.elements
            .retain(|_, (_, weak_ref)| weak_ref.strong_count() > 0);

        let cleaned = initial_count - self.elements.len();

        if cleaned > 0 {
            debug!("Cleaned up {} dead element references", cleaned);
        }

        cleaned
    }

    /// Get the number of registered elements.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

// Forward declaration for use in trait definition
use super::events::{KeyboardEvent, MouseEvent};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_element_id_generation() {
        let id1 = ElementId::new();
        let id2 = ElementId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_metadata_defaults() {
        let id = ElementId::new();
        let metadata = ElementMetadata::new(id, Region::Center);
        assert_eq!(metadata.id, id);
        assert_eq!(metadata.region, Region::Center);
        assert!(metadata.is_visible());
        assert!(!metadata.focusable);
    }

    #[test]
    fn test_metadata_builders() {
        let id = ElementId::new();
        let metadata = ElementMetadata::new(id, Region::Top)
            .with_visibility(Visibility::Hidden)
            .with_z_order(10)
            .with_focusable(true)
            .with_fixed_height(3);

        assert!(!metadata.is_visible());
        assert_eq!(metadata.z_order, 10);
        assert!(metadata.focusable);
        assert_eq!(metadata.fixed_height, Some(3));
    }

    #[test]
    fn test_registry_registration() {
        let mut registry = ElementRegistry::new();
        assert!(registry.is_empty());

        let id = ElementId::new();
        let metadata = ElementMetadata::new(id, Region::Center);

        let _weak_ref = Weak::<DummyElement>::new();
        let handle = registry.register(metadata, Arc::new(DummyElement::new(id)));

        assert!(handle.is_ok());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_registry_duplicate_registration() {
        let mut registry = ElementRegistry::new();
        let id = ElementId::new();
        let metadata = ElementMetadata::new(id, Region::Center);

        let weak_ref = Weak::<DummyElement>::new();
        let _ = registry.register(metadata.clone(), Arc::new(DummyElement::new(id)));

        let result = registry.register(metadata, Arc::new(DummyElement::new(id)));
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_unregister() {
        let mut registry = ElementRegistry::new();
        let id = ElementId::new();
        let metadata = ElementMetadata::new(id, Region::Center);

        let _ = registry.register(metadata, Arc::new(DummyElement::new(id)));
        assert_eq!(registry.len(), 1);

        let result = registry.unregister(id);
        assert!(result.is_ok());
        assert!(registry.is_empty());
    }

    #[test]
    fn test_registry_get_metadata() {
        let mut registry = ElementRegistry::new();
        let id = ElementId::new();
        let metadata = ElementMetadata::new(id, Region::Center);

        let _ = registry.register(metadata, Arc::new(DummyElement::new(id)));
        let result = registry.get_metadata(id);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, id);
    }

    #[test]
    fn test_registry_cleanup() {
        let mut registry = ElementRegistry::new();
        let id1 = ElementId::new();
        let id2 = ElementId::new();

        let metadata1 = ElementMetadata::new(id1, Region::Center);
        let metadata2 = ElementMetadata::new(id2, Region::Top);

        let _ = registry.register(metadata1, Arc::new(DummyElement::new(id1)));
        let _ = registry.register(metadata2, Arc::new(DummyElement::new(id2)));

        assert_eq!(registry.len(), 2);

        let cleaned = registry.cleanup_dead_refs();
        assert_eq!(cleaned, 2);
        assert!(registry.is_empty());
    }

    struct DummyElement {
        id: ElementId,
    }

    impl DummyElement {
        fn new(id: ElementId) -> Self {
            Self { id }
        }
    }

    impl Element for DummyElement {
        fn id(&self) -> ElementId {
            self.id
        }

        fn on_metadata_update(&self, _metadata: &ElementMetadata) {}

        fn on_render(&self) {}

        fn on_keyboard(&self, _event: &KeyboardEvent) -> bool {
            false
        }

        fn on_mouse(&self, _event: &MouseEvent) -> bool {
            false
        }

        fn on_focus_gain(&self) {}

        fn on_focus_loss(&self) {}

        fn on_tick(&self) {}
    }
}

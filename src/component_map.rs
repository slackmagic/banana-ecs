use crate::component_store::ComponentStore;
use crate::{Component, Entity};
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct ComponentMap {
    store: HashMap<TypeId, Box<Any>>,
}

impl ComponentMap {
    pub fn new() -> ComponentMap {
        ComponentMap {
            store: HashMap::new(),
        }
    }

    pub fn set<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        self.store
            //TODO: Define id
            .insert(TypeId::of::<C>(), Box::new(ComponentStore::<C>::new(1)))
            .map(|previous_comp| *previous_comp.downcast::<C>().expect(""))
    }

    fn borrow<C: Component>(&self) -> Option<&C> {
        self.store
            .get(&TypeId::of::<C>())
            .map(|retrieved_comp| retrieved_comp.downcast_ref().expect(""))
    }

    fn borrow_mut<C: Component>(&mut self) -> Option<&mut C> {
        self.store
            .get_mut(&TypeId::of::<C>())
            .map(|retrieved_comp| retrieved_comp.downcast_mut().expect(""))
    }

    #[allow(map_clone)]
    fn clone<C: Component + Clone>(&mut self) -> Option<C> {
        self.store
            .get(&TypeId::of::<C>())
            .map(|retrieved_comp| retrieved_comp.downcast_ref().expect(""))
            .map(Clone::clone)
    }

    fn contains<C: Component>(&self) -> bool {
        self.store.contains_key(&TypeId::of::<C>())
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }
}
#[cfg(test)]
mod component_store_tests {
    use super::*;

    #[test]
    fn should_create_component_map() {
        //S
        let mut store: ComponentMap = ComponentMap::new();
        let ent: Entity = Entity { id: 12 };

        //E
        store.set(ent, 0 as usize);
        store.set(ent, "".to_owned());

        //V
        assert_eq!(store.len(), 2);
        assert!(store.contains::<String>());
        assert!(store.contains::<usize>());
        assert!(!store.contains::<u32>());
    }

    #[test]
    fn should_return_item() {
        let mut store: ComponentMap = ComponentMap::new();
        let ent: Entity = Entity { id: 12 };
        store.set(ent, "ok".to_owned());

        assert_eq!(store.clone::<String>().unwrap(), "ok".to_owned());
    }

    #[test]
    fn should_return_none_if_item_not_found() {
        let store: ComponentMap = ComponentMap::new();
        assert_eq!(store.borrow::<usize>(), None);
    }

    #[test]
    fn should_update_item() {
        let mut store: ComponentMap = ComponentMap::new();
        let ent: Entity = Entity { id: 12 };

        store.set(ent, "ok".to_owned());
        store.set(ent, "updated".to_owned());

        assert_eq!(store.clone::<String>().unwrap(), "updated".to_owned());
    }
}

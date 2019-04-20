use crate::component_store::ComponentStore;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait Component: Any {}
impl<T: Any> Component for T {}

#[derive(Default)]
pub struct Components {
    store: HashMap<TypeId, Box<Any>>,
}

impl Components {
    fn set<C: Component>(&mut self, component: C) -> Option<C> {
        self.store
            .insert(TypeId::of::<C>(), Box::new(component))
            .map(|old| {
                *old.downcast::<C>()
                    .expect("ComponentMap.set: internal downcast error")
            })
    }
}

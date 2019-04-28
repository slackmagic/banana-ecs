use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct EntityStore {
    current_id: u32,
    store: HashMap<u32, Vec<TypeId>>,
}

impl EntityStore {
    pub fn new() -> EntityStore {
        EntityStore {
            current_id: 0,
            store: HashMap::new(),
        }
    }

    pub fn get_id(&mut self) -> u32 {
        self.current_id += 1;
        self.current_id
    }

    pub fn insert(&mut self, id: u32, type_id: TypeId) {
        if self.store.contains_key(&id) {
            self.store
                .get_mut(&id)
                .and_then(|types| Some(types.push(type_id)));
        } else {
            let mut vec_to_insert = Vec::new();
            vec_to_insert.push(type_id);
            self.store.insert(id, vec_to_insert);
        }
    }

    pub fn remove(&mut self, id: u32, type_id: TypeId) {
        if self.store.contains_key(&id) {
            let types: &mut Vec<TypeId> = self.store.get_mut(&id).unwrap().clone();
            self.store.insert(id, types);
        }
    }
}

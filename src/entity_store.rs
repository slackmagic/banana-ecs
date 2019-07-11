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

    pub fn get_new_id(&mut self) -> u32 {
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

    pub fn contains_id(&self, id: u32) -> bool {
        self.store.contains_key(&id)
    }

    pub fn contains_type(self, id: u32, type_id: TypeId) -> bool {
        let mut is_contained: bool = false;
        if self.store.contains_key(&id) {
            let types: &Vec<TypeId> = self.store.get(&id).unwrap();

            //TODO: Use below code when available
            // types.remove_item(&type_id)
            for index in 0..types.len() {
                if types.get(index).unwrap() == &type_id {
                    is_contained = true;
                }
            }
        }

        is_contained
    }

    pub fn remove(&mut self, id: u32, type_id: TypeId) {
        if self.store.contains_key(&id) {
            let types: &mut Vec<TypeId> = self.store.get_mut(&id).unwrap();

            //TODO: Use below code when available
            // types.remove_item(&type_id)
            for index in 0..types.len() {
                if types.get(index).unwrap() == &type_id {
                    types.remove(index);
                }
            }
        }
    }

    pub fn remove_all(&mut self, id: u32) {
        self.store.remove(&id);
    }
}

#[cfg(test)]
mod system_tests {
    use super::*;

    #[test]
    fn should_create_store() {
        let mut store: EntityStore = EntityStore::new();
        assert_eq!(store.get_new_id(), 1);
    }

    #[test]
    fn should_insert_data() {
        let mut store: EntityStore = EntityStore::new();
        let id: u32 = store.get_new_id();

        store.insert(id, TypeId::of::<u32>());
        assert!(&store.contains_id(id));
        assert!(&store.contains_type(id, TypeId::of::<u32>()));
    }

    #[test]
    fn should_remove_data() {
        // unimplemented!();

        let mut store: EntityStore = EntityStore::new();
        let id: u32 = store.get_new_id();

        store.insert(id, TypeId::of::<u32>());
        store.remove(id, TypeId::of::<u32>());

        assert!(!&store.contains_id(id));
        assert!(!&store.contains_type(id, TypeId::of::<u32>()));
    }
}

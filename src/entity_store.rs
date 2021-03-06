use std::any::TypeId;
use std::collections::HashMap;

pub struct EntityStore {
    current_id: u32,
    count_entities: usize,
    store: HashMap<u32, Vec<TypeId>>,
}

impl EntityStore {
    pub fn new() -> EntityStore {
        EntityStore {
            current_id: 0,
            count_entities: 0,
            store: HashMap::new(),
        }
    }

    pub fn get_new_id(&mut self) -> u32 {
        //TODO: RECYCLE ID
        self.current_id += 1;
        self.count_entities += 1;
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

    pub fn count(&self) -> usize {
        self.count_entities
    }

    pub fn contains_id(&self, id: u32) -> bool {
        self.store.contains_key(&id)
    }

    pub fn contains_type(&self, id: u32, type_id: TypeId) -> bool {
        let mut is_contained: bool = false;
        if self.store.contains_key(&id) {
            let types: &Vec<TypeId> = self.store.get(&id).unwrap();

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

            if types.len() == 0 {
                self.store.remove_entry(&id);
            }
        }
    }

    pub fn remove_all(&mut self, id: u32) {
        self.store.remove(&id);
        self.count_entities -= 1;
    }
}

#[cfg(test)]
mod system_tests {
    use super::*;

    #[test]
    fn should_create_store() {
        let mut store: EntityStore = EntityStore::new();
        assert_eq!(&store.get_new_id(), &1);
        assert_eq!(&store.count(), &1);
    }

    #[test]
    fn should_insert_data() {
        let mut store: EntityStore = EntityStore::new();
        let id: u32 = store.get_new_id();

        store.insert(id, TypeId::of::<u32>());
        assert!(&store.contains_id(id));
        assert!(&store.contains_type(id, TypeId::of::<u32>()));
        assert_eq!(&store.count(), &1);
    }

    #[test]
    fn should_remove_data() {
        let mut store: EntityStore = EntityStore::new();
        let id: u32 = store.get_new_id();

        store.insert(id, TypeId::of::<u32>());
        store.remove(id, TypeId::of::<u32>());

        assert!(!&store.contains_type(id, TypeId::of::<u32>()));
        assert!(!&store.contains_id(id));
    }

    #[test]
    fn should_remove_all_data() {
        let mut store: EntityStore = EntityStore::new();
        let id: u32 = store.get_new_id();

        store.insert(id, TypeId::of::<u32>());
        store.insert(id, TypeId::of::<String>());
        store.remove_all(id);

        assert!(!&store.contains_type(id, TypeId::of::<u32>()));
        assert!(!&store.contains_type(id, TypeId::of::<String>()));
        assert!(!&store.contains_id(id));
    }
}

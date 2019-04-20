use crate::Component;
use std::collections::HashMap;

pub struct ComponentStore<T> {
    id: usize,
    store: HashMap<u32, Box<T>>,
}

impl<T> ComponentStore<T> {
    pub fn new(id: usize) -> ComponentStore<T> {
        ComponentStore {
            id: id,
            store: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn set(&mut self, entity_id: u32, entity: T) -> T {
        *self.store.insert(entity_id, Box::new(entity)).unwrap()
    }

    pub fn get(&mut self, entity_id: u32) -> &mut T {
        //TODO: Send Option
        let boxed_entity = self.store.get_mut(&entity_id).unwrap();
        &mut *boxed_entity
    }

    pub fn remove(&mut self, entity_id: u32) {
        self.store.remove(&entity_id);
    }

    pub fn count(&self) -> usize {
        self.store.len()
    }
}

#[cfg(test)]
mod component_tests {

    use super::ComponentStore;

    struct MyStruct {
        pub title: String,
        pub value: u32,
    }

    #[test]
    fn should_create_component() {
        let store: ComponentStore<u32> = ComponentStore::new(123);
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn should_get_component_id() {
        let store: ComponentStore<u32> = ComponentStore::new(123);
        assert_eq!(store.get_id(), 123);
    }

    #[test]
    fn should_add_new_items() {
        let store: &mut ComponentStore<u32> = &mut ComponentStore::new(123);
        store.set(1, 456);
        store.set(2, 789);

        assert_eq!(2, store.count());
    }

    #[test]
    fn should_edit_items() {
        let store: &mut ComponentStore<MyStruct> = &mut ComponentStore::new(123);
        let test_entity = MyStruct {
            title: "OK".to_owned(),
            value: 10,
        };

        store.set(1, test_entity);

        let entity: &mut MyStruct = &mut store.get(1);
        assert_eq!(entity.title, "OK");
        assert_eq!(entity.value, 10);

        entity.title = "NEW VALUE".to_owned();
        entity.value = entity.value + 1;
        let updated_entity: &mut MyStruct = &mut store.get(1);

        assert_eq!(updated_entity.title, "NEW VALUE");
        assert_eq!(updated_entity.value, 11);
    }

    #[test]
    fn should_delete_item() {
        let store: &mut ComponentStore<u32> = &mut ComponentStore::new(123);
        store.set(1, 456);

        assert_eq!(1, store.count());
        store.remove(1);

        assert_eq!(0, store.count());
    }
}

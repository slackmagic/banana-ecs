use std::collections::HashMap;

pub struct Component<T> {
    id: u32,
    store: HashMap<u32, Box<T>>,
}

impl<T> Component<T> {
    pub fn new(id: u32) -> Component<T> {
        Component {
            id: id,
            store: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn add(&mut self, entity_id: u32, entity: T) {
        self.store.insert(entity_id, Box::new(entity));
    }

    pub fn get(&mut self, entity_id: u32) -> &mut T {
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

    use crate::component::Component;

    struct MyStruct {
        pub title: String,
        pub value: u32,
    }

    #[test]
    fn should_create_component() {
        let comp: Component<u32> = Component::new(123);
        assert_eq!(comp.count(), 0);
    }

    #[test]
    fn should_get_component_id() {
        let comp: Component<u32> = Component::new(123);
        assert_eq!(comp.get_id(), 123);
    }

    #[test]
    fn should_add_new_items() {
        let comp: &mut Component<u32> = &mut Component::new(123);
        comp.add(1, 456);
        comp.add(2, 789);

        assert_eq!(2, comp.count());
    }

    #[test]
    fn should_edit_items() {
        let comp: &mut Component<MyStruct> = &mut Component::new(123);
        let test_entity = MyStruct {
            title: "OK".to_owned(),
            value: 10,
        };

        comp.add(1, test_entity);

        let entity: &mut MyStruct = &mut comp.get(1);
        assert_eq!(entity.title, "OK");
        assert_eq!(entity.value, 10);

        entity.title = "NEW VALUE".to_owned();
        entity.value = entity.value + 1;
        let updated_entity: &mut MyStruct = &mut comp.get(1);

        assert_eq!(updated_entity.title, "NEW VALUE");
        assert_eq!(updated_entity.value, 11);
    }

    #[test]
    fn should_delete_item() {
        let comp: &mut Component<u32> = &mut Component::new(123);
        comp.add(1, 456);

        assert_eq!(1, comp.count());
        comp.remove(1);

        assert_eq!(0, comp.count());
    }
}

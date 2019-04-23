use crate::component_store::ComponentStore;

use crate::{Component, Entity};
use std::any::{Any, TypeId};
use std::collections::HashMap;

const UNDEFINED: usize = 1;

struct System {
    current_id: u32,
    components: HashMap<TypeId, Box<Any>>,
    entities: HashMap<u32, Entity>,
}

impl System {
    pub fn new() -> System {
        System {
            current_id: 0,
            components: HashMap::new(),
            entities: HashMap::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        self.current_id += 1;
        let new_entity: Entity = Entity {
            id: self.current_id,
        };

        self.entities.insert(new_entity.id, new_entity);
        new_entity
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.entities.remove(&entity.id);

        //TODO: REMOVE FROM ALL STORAGE
        for (type_id, store) in self.components.iter() {
            unsafe {
                store.downcast_ref();
            }
        }
    }

    pub fn count_entities(&self) -> usize {
        self.entities.len()
    }

    pub fn set<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        let type_of_c = TypeId::of::<C>();

        //Check if a related typed store exists.
        if !self.contains_type_id(&type_of_c) {
            //TODO: Define id
            self.components
                .insert(type_of_c, Box::new(ComponentStore::<C>::new(UNDEFINED)));
        }

        //Insert the component into the related store.
        self.get_store::<C>()
            .and_then(|store| store.set(entity.id, component))
    }

    pub fn get_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        self.get_store::<C>()
            .and_then(|store| store.borrow_mut(entity.id))
    }

    pub fn get<C: Component>(&mut self, entity: Entity) -> Option<&C> {
        self.get_store::<C>()
            .and_then(|store| store.borrow(entity.id))
    }

    pub fn remove<C: Component>(&mut self, entity: Entity) -> Option<C> {
        let ret = self
            .get_store::<C>()
            .and_then(|store| store.remove(entity.id));

        self.clean_store::<C>();
        ret
    }

    pub fn count<C: Component>(&mut self) -> usize {
        //TODO: Manage error
        self.get_store::<C>()
            .and_then(|store| Some(store.len()))
            .unwrap_or(0)
    }

    fn contains_type_id(&self, id: &TypeId) -> bool {
        self.components.contains_key(id)
    }

    fn get_store<C: Component>(&mut self) -> Option<&mut ComponentStore<C>> {
        //TODO: Safely Unwrap
        //TODO: Manage error
        self.components
            .get_mut(&TypeId::of::<C>())
            .and_then(|store: &mut Box<Any>| {
                Some(store.downcast_mut::<ComponentStore<C>>().expect(""))
            })
    }

    fn clean_store<C: Component>(&mut self) {
        let store = self.get_store::<C>().unwrap();
        if store.len() == 0 {
            self.components.remove(&TypeId::of::<C>());
        }
    }
}

#[cfg(test)]
mod system_tests {
    use super::*;
    use crate::components_lib::position::Position;
    use crate::components_lib::velocity::Velocity;

    #[test]
    fn should_create_system() {
        let sys: System = System::new();
        assert_eq!(sys.count_entities(), 0);
    }

    #[test]
    fn should_create_entity() {
        let mut sys: System = System::new();
        let ent: Entity = sys.new_entity();
        assert_eq!(sys.count_entities(), 1);
    }

    #[test]
    fn should_set_entity_component() {
        let mut sys: System = System::new();
        let ent = sys.new_entity();

        let pos = Position { x: 0, y: 0 };
        let vel = Velocity { vel: 0.2 };

        sys.set(ent, pos);
        sys.set(ent, vel);

        assert_eq!(sys.count_entities(), 1);
    }

    #[test]
    fn should_get_entity_component() {
        let mut sys: System = System::new();
        let ent = sys.new_entity();

        let pos = Position { x: 0, y: 0 };
        let vel = Velocity { vel: 0.2 };

        sys.set(ent, pos);
        sys.set(ent, vel);
        assert_eq!(sys.count_entities(), 1);

        let ret_pos = sys.get::<Position>(ent).unwrap().clone();
        let ret_vel = sys.get::<Velocity>(ent).unwrap().clone();

        assert_eq!(ret_pos.x, 0);
        assert_eq!(ret_pos.y, 0);
        assert_eq!(ret_vel.vel, 0.2);
    }

    #[test]
    fn should_delete_entity() {
        let mut sys: System = System::new();
        let ent = sys.new_entity();

        let pos = Position { x: 0, y: 0 };
        let vel = Velocity { vel: 0.2 };

        sys.set(ent, pos);
        sys.set(ent, vel);
        assert_eq!(sys.count_entities(), 1);
        assert_eq!(sys.count::<Position>(), 1);

        sys.remove::<Position>(ent);
        assert_eq!(sys.count::<Position>(), 0);
    }
}

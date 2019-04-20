use crate::component::Component;
use crate::component_map::ComponentMap;
use crate::component_store::ComponentStore;
use crate::components_lib::position::Position;
use crate::components_lib::velocity::Velocity;
use crate::entity::Entity;
use std::any::{Any, TypeId};
use std::collections::HashMap;

const MAX_ENTS: usize = 200;
const UNDEFINED: usize = 1;

struct System {
    current_id: u32,
    components: HashMap<TypeId, Box<Any>>,
    entities: Vec<Entity>,
}

impl System {
    pub fn new() -> System {
        System {
            current_id: 0,
            components: HashMap::new(),
            entities: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        self.current_id += 1;
        let new_entity: Entity = Entity {
            id: self.current_id,
        };

        self.entities.push(new_entity);
        new_entity
    }

    pub fn count_entities(&self) -> usize {
        self.entities.len()
    }

    pub fn set<C: Component>(&mut self, entity: Entity, component: C) {
        let type_of_c = &TypeId::of::<C>();

        //Check if a related typed store exists.
        if !self.contains_type_id(type_of_c) {
            //TODO: Define id
            self.components.insert(
                TypeId::of::<C>(),
                Box::new(ComponentStore::<C>::new(UNDEFINED)),
            );
        }

        let store: &mut ComponentStore<C> = self.get_store::<C>();

        //Insert the component into the related store.
        store.set(entity.id, component);
    }

    pub fn get<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        let type_of_c = &TypeId::of::<C>();

        //Check if a related typed store exists.
        if !self.contains_type_id(type_of_c) {
            return None;
        }

        let store: &mut ComponentStore<C> = self.get_store::<C>();

        //Get the component from the related store.
        Some(store.get(entity.id))
    }

    fn contains_type_id(&self, id: &TypeId) -> bool {
        self.components.contains_key(id)
    }

    fn get_store<C: Component>(&mut self) -> &mut ComponentStore<C> {
        //TODO: Safely Unwrap
        //TODO: Manage error
        self.components
            .get_mut(&TypeId::of::<C>())
            .map(|store: &mut Box<Any>| store.downcast_mut::<ComponentStore<C>>().expect(""))
            .unwrap()
    }
}

#[cfg(test)]
mod system_tests {
    use super::*;
    #[test]
    fn should_create_system() {
        let sys: System = System::new();
        assert_eq!(sys.count_entities(), 0);
    }

    #[test]
    fn should_create_entity() {
        let mut sys: System = System::new();
        let mut ent: Entity = sys.new_entity();
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
}

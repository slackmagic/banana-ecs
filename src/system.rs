use crate::component_store::ComponentStore;
use crate::entity_store::EntityStore;

use crate::{Component, Entity};
use std::any::{Any, TypeId};
use std::collections::HashMap;

const UNDEFINED: usize = 1;

pub struct System {
    components: HashMap<TypeId, Box<Any>>,
    entity_store: EntityStore,
}

impl System {
    pub fn new() -> System {
        System {
            components: HashMap::new(),
            entity_store: EntityStore::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let new_entity: Entity = Entity {
            id: self.entity_store.get_new_id(),
        };
        new_entity
    }

    pub fn count_entities(&self) -> usize {
        self.entity_store.count()
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
        self.get_mut_store::<C>()
            .and_then(|store| store.set(entity.id, component))
    }

    pub fn get_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        self.get_mut_store::<C>()
            .and_then(|store| store.borrow_mut(entity.id))
    }

    pub fn get<C: Component>(&mut self, entity: Entity) -> Option<&C> {
        self.get_mut_store::<C>()
            .and_then(|store| store.borrow(entity.id))
    }

    pub fn get_mut_components<C: Component>(&mut self) -> Option<&mut ComponentStore<C>> {
        self.get_mut_store::<C>()
    }

    pub fn get_component<C: Component>(&mut self) -> Option<&mut ComponentStore<C>> {
        self.get_mut_store::<C>()
    }

    pub fn remove<C: Component>(&mut self, entity: Entity) -> Option<C> {
        let ret = self
            .get_mut_store::<C>()
            .and_then(|store| store.remove(entity.id));

        self.clean_store::<C>();
        ret
    }

    pub fn remove_components<C: Component>(&mut self) {
        self.components.remove(&TypeId::of::<C>());
    }

    pub fn count<C: Component>(&mut self) -> usize {
        //TODO: Manage error
        self.get_mut_store::<C>()
            .and_then(|store| Some(store.len()))
            .unwrap_or(0)
    }

    fn contains_type_id(&self, id: &TypeId) -> bool {
        self.components.contains_key(id)
    }

    fn get_mut_store<C: Component>(&mut self) -> Option<&mut ComponentStore<C>> {
        //TODO: Manage error
        self.components
            .get_mut(&TypeId::of::<C>())
            .and_then(|store: &mut Box<Any>| {
                Some(store.downcast_mut::<ComponentStore<C>>().expect(""))
            })
    }

    fn clean_store<C: Component>(&mut self) {
        let store = self.get_mut_store::<C>().unwrap();
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
        let _ent: Entity = sys.new_entity();
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

    #[test]
    fn should_delete_components() {
        let mut sys: System = System::new();
        let ent1 = sys.new_entity();
        let ent2 = sys.new_entity();

        let pos1 = Position { x: 0, y: 0 };
        let pos2 = Position { x: 0, y: 0 };
        sys.set(ent1, pos1);
        sys.set(ent2, pos2);

        assert_eq!(sys.count_entities(), 2);
        assert_eq!(sys.count::<Position>(), 2);

        sys.remove_components::<Position>();
        assert_eq!(sys.count::<Position>(), 0);
    }

    #[test]
    fn should_iter_mut_on_components() {
        let mut sys: System = System::new();
        let ent1 = sys.new_entity();
        let ent2 = sys.new_entity();

        let pos1 = Position { x: 0, y: 0 };
        let pos2 = Position { x: 10, y: 20 };
        let vel1 = Velocity { vel: 0.5678 };

        sys.set(ent1, pos1);
        sys.set(ent1, vel1);
        sys.set(ent2, pos2);

        assert_eq!(sys.count_entities(), 2);
        assert_eq!(sys.count::<Position>(), 2);

        {
            //Check/Update Position components
            //--------------------------------------------------------------
            let components = sys.get_mut_components::<Position>().unwrap();
            assert_eq!(components.len(), 2);

            for component in components.iter_mut() {
                let id: &u32 = component.0;
                let value: &mut Position = component.1;
                println!("Position : {:?}:{:?}", id, value);

                value.x += 10;
                value.y += 100;
            }
        }

        {
            //Check/Update Velocity components
            //--------------------------------------------------------------
            let components = sys.get_mut_components::<Velocity>().unwrap();
            assert_eq!(components.len(), 1);

            for component in components.iter_mut() {
                let id: &u32 = component.0;
                let value: &mut Velocity = component.1;
                println!("Velocity : {:?}:{:?}", id, value);

                value.vel *= 2.0;
            }
        }

        {
            //Check Position components
            //--------------------------------------------------------------
            let components = sys.get_mut_components::<Position>().unwrap();
            assert_eq!(components.len(), 2);

            for component in components.iter() {
                let id: &u32 = component.0;
                let value: &Position = component.1;
                println!("Position : {:?}:{:?}", id, value);
            }
        }

        {
            //Check Velocity components
            //--------------------------------------------------------------
            let components = sys.get_mut_components::<Velocity>().unwrap();
            assert_eq!(components.len(), 1);

            for component in components.iter() {
                let id: &u32 = component.0;
                let value: &Velocity = component.1;
                println!("Velocity : {:?}:{:?}", id, value);
            }
        }
    }
}

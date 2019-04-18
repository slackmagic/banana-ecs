use crate::component::Component;
use std::collections::HashMap;

const MAX_ENTS: usize = 200;
const EMPTY: u32 = 0;

struct World {
    entity_masks: [u32; MAX_ENTS],
    current_entity_id: u32,
}

impl World {
    pub fn new() -> World {
        World {
            current_entity_id: 0,
            entity_masks: [EMPTY; MAX_ENTS],
        }
    }

    pub fn get_new_entity_id(&mut self) -> u32 {
        self.current_entity_id += 1;
        self.current_entity_id
    }

    pub fn delete_entity(&mut self, entity_id: u32) {
        self.entity_masks[entity_id as usize] = EMPTY;
    }

    pub fn count_entities(&self) -> u32 {
        self.current_entity_id
    }
}

#[cfg(test)]
mod world_tests {

    use crate::world::World;

    #[test]
    fn should_create_world() {
        let mut my_world = World::new();
        assert_eq!(my_world.get_new_entity_id(), 1);
    }

    #[test]
    fn should_count_entities() {
        let mut my_world = World::new();
        assert_eq!(my_world.count_entities(), 0);
    }

}

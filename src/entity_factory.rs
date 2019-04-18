use crate::component::Component;
use std::collections::HashMap;

const MAX_ENTS: usize = 200;
const EMPTY: u32 = 0;
const RESERVED: u32 = 1 << 0;

struct EntityFactory {
    entity_masks: [u32; MAX_ENTS],
    entities_number: u32,
}

impl EntityFactory {
    pub fn new() -> EntityFactory {
        EntityFactory {
            entities_number: 0,
            entity_masks: [EMPTY as u32; MAX_ENTS],
        }
    }

    pub fn get_new_entity_id(&mut self) -> Option<u32> {
        for index in 0..(self.entities_number + 1) {
            println!("Check POS {} ", index);
            if self.entity_masks[index as usize] == EMPTY {
                println!("--> Empty found at {} ", index);
                self.entity_masks[index as usize] = RESERVED;

                //TODO ADJUST WHEN INSERT IN MIDDLE
                self.entities_number = index + 1;
                return Some(self.entities_number);
            }
        }
        None
    }

    pub fn delete_entity(&mut self, entity_id: u32) {
        if self.entity_masks[entity_id as usize] != EMPTY {}
    }

    pub fn count_entities(&self) -> u32 {
        //WHERE ENTITY MASK <> 0
        self.entities_number
    }
}

#[cfg(test)]
mod world_tests {

    use crate::entity_factory::EntityFactory;

    #[test]
    fn should_create_world() {
        let mut factory = EntityFactory::new();
        println!("{}", factory.get_new_entity_id().unwrap());
        println!("{}", factory.get_new_entity_id().unwrap());
        println!("{}", factory.get_new_entity_id().unwrap());
        assert!(factory.get_new_entity_id().is_some());
        assert_eq!(factory.get_new_entity_id().unwrap(), 5);
    }

    #[test]
    fn should_count_entities() {
        let mut factory = EntityFactory::new();
        assert_eq!(factory.count_entities(), 0);
    }

}

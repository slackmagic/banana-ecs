use crate::component::Component;
use std::collections::HashMap;

const MAX_ENTS: usize = 200;
const EMPTY: u32 = 0;
const RESERVED: u32 = 1 << 0;

struct EntityFactory {
    entity_masks: [u32; MAX_ENTS],
    reusable_space: Vec<u32>,
    entities_number: u32,
}

impl EntityFactory {
    pub fn new() -> EntityFactory {
        EntityFactory {
            entities_number: 0,
            reusable_space: Vec::new(),
            entity_masks: [EMPTY as u32; MAX_ENTS],
        }
    }

    pub fn add_entity(&mut self) -> Option<u32> {
        let mut index: u32;

        if self.reusable_space.len() > 0 {
            // Get any reusable place if available.
            index = self.reusable_space.pop().unwrap();
            println!("REUSE POS # {}", index);
        } else {
            // Search the next available space
            if self.entities_number >= MAX_ENTS as u32 {
                return None;
            } else {
                index = self.entities_number;
            }
        }

        self.entity_masks[index as usize] = RESERVED;
        self.entities_number += 1;

        Some(index)
    }

    pub fn delete_entity(&mut self, entity_id: u32) {
        self.entity_masks[entity_id as usize] = EMPTY;
        self.reusable_space.push(entity_id);
        self.entities_number -= 1;
    }

    pub fn count_entities(&self) -> u32 {
        //WHERE ENTITY MASK <> 0
        self.entities_number
    }

    pub fn count_reusable_space(&self) -> u32 {
        self.reusable_space.len() as u32
    }
}

#[cfg(test)]
mod world_tests {

    use crate::entity_factory::EntityFactory;

    #[test]
    fn should_add_entities() {
        let mut factory = EntityFactory::new();

        println!("{}", factory.add_entity().unwrap());
        println!("{}", factory.add_entity().unwrap());

        assert_eq!(factory.add_entity().unwrap(), 2);
        assert_eq!(factory.count_entities(), 3);
    }

    #[test]
    fn should_add_entities_complex() {
        let mut factory = EntityFactory::new();

        for _ in 0..50 {
            factory.add_entity();
        }

        factory.delete_entity(10);
        assert_eq!(factory.count_reusable_space(), 1);
        assert_eq!(factory.add_entity().unwrap(), 10);

        factory.delete_entity(20);
        factory.delete_entity(30);
        assert_eq!(factory.count_reusable_space(), 2);
        assert_eq!(factory.add_entity().unwrap(), 30);
        assert_eq!(factory.add_entity().unwrap(), 20);

        assert_eq!(factory.count_entities(), 50);
    }

    #[test]
    fn should_count_entities() {
        let mut factory = EntityFactory::new();
        assert_eq!(factory.count_entities(), 0);
    }

}

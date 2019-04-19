use crate::component_store::ComponentStore;

struct World {
    pub firstStore: ComponentStore<u128>,
    pub secondStore: ComponentStore<String>,
    pub thirdStore: ComponentStore<Vec<u32>>,
}

impl World {
    pub fn new() -> World {
        World {
            firstStore: ComponentStore::new(1 as usize),
            secondStore: ComponentStore::new(2 as usize),
            thirdStore: ComponentStore::new(3 as usize),
        }
    }
}

#[cfg(test)]
mod world_tests {
    use crate::*;
    #[test]
    fn create_world() {
        let mut world: world::World = world::World::new();
        let mut factory: entity_factory::EntityFactory<world::World> =
            entity_factory::EntityFactory::new(world);

        //TODO DEFINE PARTS
        //DEFINE CONTENT
        factory.parts.firstStore.add(1, 2 as u128);

        factory.add_entity();
        factory.delete_entity(0);
        factory.count_entities();
    }
}

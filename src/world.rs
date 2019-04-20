use crate::component_store::ComponentStore;

pub const POSITION: u32 = 1 << 0;
pub const VELOCITY: u32 = 1 << 1;
pub const INPUT: u32 = 1 << 4;

struct World {
    pub position: ComponentStore<u128>,
    pub velocity: ComponentStore<String>,
    pub input: ComponentStore<Vec<u32>>,
}

impl World {
    pub fn new() -> World {
        World {
            position: ComponentStore::new(1 as usize),
            velocity: ComponentStore::new(2 as usize),
            input: ComponentStore::new(3 as usize),
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
        factory.parts.position.add(1, 2 as u128);

        factory.add_entity();
        factory.delete_entity(0);
        factory.count_entities();
    }
}

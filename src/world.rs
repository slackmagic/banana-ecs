use std::collections::HashMap;
use crate::component::Component;

const MAX_ENTS :usize = 200;

struct World {
    entity_masks: [u32; MAX_ENTS],
}

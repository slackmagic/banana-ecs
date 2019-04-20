use std::any::Any;

pub mod component_map;
pub mod component_store;
pub mod components_lib;
pub mod entity;
pub mod system;

//ENTITY
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Entity {
    pub id: u32,
}

//COMPONENT
pub trait Component: Any {}
impl<T: Any> Component for T {}

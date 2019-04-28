use std::any::{Any, TypeId};

pub mod component_store;
pub mod components_lib;
pub mod entity_store;
pub mod system;

//ENTITY
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Entity {
    pub id: u32,
}

//COMPONENT
pub trait Component: Any {}
impl<T: Any> Component for T {}

//TODO: Clean & implement
pub type StoreResult<T> = Result<T, ObjectNotFound>;

//TODO: Clean & implement
pub enum ObjectNotFound {
    /// A requested entity ID was not present in the system.
    Entity(Entity),
    /// A requested component was not present on an entity.
    Component(TypeId),
}

# 🦀🍌 banana-ecs
## Home Entity Component System.


### System:
- Main entry point !
- Manage the Entity/ComponentStore creation
- Store all Entities (Hashmap of Entities)
- Store all ComponentStores (Hashmap of Boxed ComponentStore)


### ComponentStore:
- Store Components of the defined type
- Data are Boxed

### EntityStore:
- Store all entities and their associated types.

### Entity:
- Compound by:
    - an ID generated and provided by the Factory.
    - an ID ?


---

## TODO :
- Remove entity management responsability from System to EntityStore.
- Add a way to parse all particular typed component.
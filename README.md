# banana-ecs
## A home custom Entity Component System.

### ComponentStore:
- Store a Part type for Entities.

### Part:


### EntityFactory: 
- Manage and organize of the Entities ID store table.
- Main rules: 
    - A table space can be reused.
    - An ID cannot be reused (_TODO_).

### Entity:
- Compound by:
    - an ID generated and provided by the Factory.
    - a byte token for determining which Part are used.


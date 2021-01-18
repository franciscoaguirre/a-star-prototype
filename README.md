# a-star-prototype
Prototype for the a* pathfinding algorithm for Bevy game engine

![Latest version screenshot](images/2.gif)

## Roadmap
### Obstruction
- Right click - add wall [x]
- Path cannot be constructed by using obstructed tiles [ ]
- Path gets recalcualated on obstruction of one of its tiles [ ]
- Cannot select an obstructed tile [ ]
- When clicked on obstructed tile, find path to nearest unobstructed tile [ ]

### Costs
- Add cost to different tiles [ ]
- Find clicked space on tile [ ]
- Set area costs with public method [ ]

### Nav Mesh agent
- Provide Nav Mesh Agent API with different strategies implemented [ ]
- Set Nav Mesh Agent to ignore costs [ ]

### Nav Mesh
- Implement Nav Mesh instead of grid [ ]
- Create Nav Mesh with bakeable mesh [ ]

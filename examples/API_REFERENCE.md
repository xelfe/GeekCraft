# GeekCraft - JavaScript API Reference

## Important Notice

This API reference describes the **complete intended API** for GeekCraft. However, many features are currently **placeholders** and not yet implemented on the server side.

### Current Server Support (v0.2.0+)

**✅ WORKING FEATURES:**
- Authentication system (register, login, logout with Bearer tokens)
- Multiplayer support (concurrent authenticated users)
- Game tick counter (real-time updates)
- Player list (array of player IDs)
- Code submission and storage

**🚧 PLACEHOLDER FEATURES (NOT YET IMPLEMENTED):**
- Units and unit management
- Structures and buildings
- Resources and resource collection
- Movement and pathfinding
- Combat system
- Full game simulation engine

The API described below represents the target implementation. Bot examples using these APIs will work once the server features are implemented. For current functionality, see the working examples: `auth_example.js`, `multiplayer_example.js`, and the HTML viewer.

---

## Table of Contents
- [Introduction](#introduction)
- [Bot Structure](#bot-structure)
- [GameState API](#gamestate-api)
- [Unit API](#unit-api)
- [Structure API](#structure-api)
- [Resource API](#resource-api)
- [Events](#events)
- [Examples](#examples)

---

## Introduction

GeekCraft allows players to control their units and bases via a JavaScript API. Your bot is a JavaScript class that responds to game events and controls units through the game state (GameState).

**Note**: Currently, only basic bot structure and code submission are supported. Unit control, structures, and resources will be available in future releases.

---

## Bot Structure

Your bot must be an exported JavaScript class with the following methods:

```javascript
class MyBot {
    constructor() {
        // Initialize your bot
        this.name = "MyBot";
        this.version = "1.0.0";
    }

    onInit() {
        // Called at game startup
    }

    onTick(gameState) {
        // Called every tick (main loop)
        // This is where you implement your strategy
    }

    onUnitCreated(unit) {
        // Called when a unit is created
    }

    onUnitDestroyed(unit) {
        // Called when a unit is destroyed
    }

    onResourceCollected(unit, resource, amount) {
        // Called when resources are collected
    }

    onStructureBuilt(structure) {
        // Called when a structure is built
    }

    onUnitAttack(attacker, target) {
        // Called during an attack
    }

    onUnitDamaged(unit, damage) {
        // Called when a unit takes damage
    }
}

module.exports = MyBot;
```

---

## GameState API

The `gameState` object is passed to the `onTick()` method and provides all information about the current game state.

### Unit Management

#### `gameState.getMyUnits()`
Returns an array of all your units.

```javascript
const myUnits = gameState.getMyUnits();
console.log(`I have ${myUnits.length} units`);
```

**Returns:** `Unit[]`

---

#### `gameState.getEnemyUnits()`
Returns an array of all visible enemy units.

```javascript
const enemies = gameState.getEnemyUnits();
if (enemies.length > 0) {
    console.log("Enemies detected!");
}
```

**Returns:** `Unit[]`

---

#### `gameState.getAllUnits()`
Returns all units in the game (friendly and enemy).

**Returns:** `Unit[]`

---

#### `gameState.getUnitById(id)`
Retrieves a specific unit by its ID.

**Parameters:**
- `id` (string) : The unit's unique identifier

**Returns:** `Unit | null`

---

### Resource Management

#### `gameState.getMyResources()`
Returns your current resources.

```javascript
const resources = gameState.getMyResources();
console.log(`Minerals: ${resources.minerals}, Gas: ${resources.gas}`);
```

**Returns:** 
```javascript
{
    minerals: number,
    gas: number,
    supply: number,
    maxSupply: number
}
```

---

#### `gameState.findNearestResource(position)`
Finds the nearest resource to a given position.

**Parameters:**
- `position` (Object) : `{x: number, y: number}`

**Returns:** `Resource | null`

```javascript
const resource = gameState.findNearestResource(unit.position);
if (resource) {
    unit.moveTo(resource.position);
}
```

---

#### `gameState.getAllResources()`
Returns all resources on the map.

**Returns:** `Resource[]`

---

### Base and Structure Management

#### `gameState.getMyBases()`
Returns all your bases.

**Returns:** `Structure[]`

---

#### `gameState.getMyMainBase()`
Returns your main base.

**Returns:** `Structure | null`

---

#### `gameState.buildStructure(type, position)`
Builds a structure at a given position.

**Parameters:**
- `type` (string) : Structure type (`'base'`, `'turret'`, `'factory'`)
- `position` (Object) : `{x: number, y: number}`

**Returns:** `boolean` - `true` if construction succeeded

```javascript
const pos = {x: 100, y: 100};
if (gameState.buildStructure('turret', pos)) {
    console.log("Turret built!");
}
```

---

#### `gameState.isStructureAt(position)`
Checks if there is a structure at a given position.

**Parameters:**
- `position` (Object) : `{x: number, y: number}`

**Returns:** `boolean`

---

### Map Information

#### `gameState.getMapSize()`
Returns the map dimensions.

**Returns:** 
```javascript
{
    width: number,
    height: number
}
```

---

#### `gameState.findExpansionLocation()`
Finds an optimal location for an expansion.

**Returns:** `{x: number, y: number} | null`

---

#### `gameState.getPatrolPoints()`
Returns recommended patrol points.

**Returns:** `Array<{x: number, y: number}>`

---

#### `gameState.isWalkable(position)`
Checks if a position is accessible.

**Parameters:**
- `position` (Object) : `{x: number, y: number}`

**Returns:** `boolean`

---

## Unit API

`Unit` objects represent game units (workers, soldiers, etc.).

### Properties

```javascript
unit.id          // string - Unique identifier
unit.type        // string - Unit type ('worker', 'soldier', etc.)
unit.position    // {x: number, y: number} - Current position
unit.health      // number - Current health points
unit.maxHealth   // number - Maximum health points
unit.owner       // string - Unit owner
unit.speed       // number - Movement speed
```

---

### Movement Methods

#### `unit.moveTo(position)`
Moves the unit to a position.

**Parameters:**
- `position` (Object) : `{x: number, y: number}`

**Returns:** `boolean`

```javascript
unit.moveTo({x: 50, y: 100});
```

---

#### `unit.stop()`
Stops all unit actions.

**Returns:** `void`

---

#### `unit.isIdle()`
Checks if the unit is idle.

**Returns:** `boolean`

```javascript
if (unit.isIdle()) {
    // Assign a new task
}
```

---

### Resource Actions

#### `unit.harvest(resource)`
Makes the unit harvest a resource.

**Parameters:**
- `resource` (Resource) : The resource to harvest

**Returns:** `boolean`

```javascript
const resource = gameState.findNearestResource(unit.position);
if (resource) {
    unit.harvest(resource);
}
```

---

#### `unit.deposit()`
Deposits carried resources at the nearest base.

**Returns:** `boolean`

---

#### `unit.isCarryingResource()`
Checks if the unit is carrying resources.

**Returns:** `boolean`

---

#### `unit.getCarriedAmount()`
Returns the amount of resources carried.

**Returns:** `number`

---

### Combat Actions

#### `unit.attack(target)`
Attacks a target (unit or structure).

**Parameters:**
- `target` (Unit | Structure) : The target to attack

**Returns:** `boolean`

```javascript
const enemies = gameState.getEnemyUnits();
if (enemies.length > 0) {
    unit.attack(enemies[0]);
}
```

---

#### `unit.defend(position)`
Puts the unit in defensive position.

**Parameters:**
- `position` (Object) : `{x: number, y: number}`

**Returns:** `boolean`

---

#### `unit.canAttack(target)`
Checks if the unit can attack a target.

**Parameters:**
- `target` (Unit | Structure)

**Returns:** `boolean`

---

### Information

#### `unit.getDistanceTo(position)`
Calculates the distance to a position.

**Parameters:**
- `position` (Object) : `{x: number, y: number}`

**Returns:** `number`

```javascript
const dist = unit.getDistanceTo({x: 100, y: 100});
console.log(`Distance: ${dist}`);
```

---

## Structure API

Structures include bases, turrets, factories, etc.

### Properties

```javascript
structure.id          // string - Unique identifier
structure.type        // string - Structure type
structure.position    // {x: number, y: number}
structure.health      // number
structure.maxHealth   // number
structure.owner       // string
```

### Methods

#### `structure.produceUnit(unitType)`
Produces a new unit (if it's a base or factory).

**Parameters:**
- `unitType` (string) : Unit type to produce

**Returns:** `boolean`

```javascript
const base = gameState.getMyMainBase();
if (base.canProduceUnit()) {
    base.produceUnit('worker');
}
```

---

#### `structure.canProduceUnit()`
Checks if the structure can produce a unit.

**Returns:** `boolean`

---

## Resource API

Represents a resource on the map.

### Properties

```javascript
resource.id        // string
resource.type      // string - 'minerals' or 'gas'
resource.position  // {x: number, y: number}
resource.amount    // number - Remaining quantity
```

---

## Events

### Lifecycle

1. **`onInit()`** - Called once at startup
2. **`onTick(gameState)`** - Called every frame (60 FPS)
3. Specific events triggered based on actions

### Unit Events

- **`onUnitCreated(unit)`** - New unit created
- **`onUnitDestroyed(unit)`** - Unit destroyed
- **`onUnitAttack(attacker, target)`** - Attack performed
- **`onUnitDamaged(unit, damage)`** - Unit damaged

### Resource Events

- **`onResourceCollected(unit, resource, amount)`** - Resources collected

### Structure Events

- **`onStructureBuilt(structure)`** - Structure built
- **`onStructureDestroyed(structure)`** - Structure destroyed

---

## Examples

### Example 1: Simple Harvester Bot

```javascript
class HarvesterBot {
    onTick(gameState) {
        const workers = gameState.getMyUnits()
            .filter(u => u.type === 'worker');
        
        for (const worker of workers) {
            if (worker.isIdle()) {
                if (!worker.isCarryingResource()) {
                    const resource = gameState.findNearestResource(worker.position);
                    if (resource) {
                        worker.moveTo(resource.position);
                        worker.harvest(resource);
                    }
                } else {
                    const base = gameState.getMyMainBase();
                    worker.moveTo(base.position);
                    worker.deposit();
                }
            }
        }
    }
}
```

### Example 2: Military Bot

```javascript
class MilitaryBot {
    onTick(gameState) {
        const soldiers = gameState.getMyUnits()
            .filter(u => u.type === 'soldier');
        
        const enemies = gameState.getEnemyUnits();
        
        if (enemies.length > 0) {
            // Attack as a group
            const target = enemies[0];
            for (const soldier of soldiers) {
                if (soldier.canAttack(target)) {
                    soldier.attack(target);
                }
            }
        } else {
            // Patrol
            const points = gameState.getPatrolPoints();
            soldiers.forEach((s, i) => {
                s.moveTo(points[i % points.length]);
            });
        }
    }
}
```

### Example 3: Economy Bot

```javascript
class EconomyBot {
    constructor() {
        this.targetWorkers = 10;
    }
    
    onTick(gameState) {
        const resources = gameState.getMyResources();
        const workers = gameState.getMyUnits()
            .filter(u => u.type === 'worker');
        
        // Produce workers
        if (workers.length < this.targetWorkers && 
            resources.minerals >= 50) {
            const base = gameState.getMyMainBase();
            if (base.canProduceUnit()) {
                base.produceUnit('worker');
            }
        }
        
        // Build an expansion
        if (resources.minerals >= 400) {
            const location = gameState.findExpansionLocation();
            if (location) {
                gameState.buildStructure('base', location);
            }
        }
    }
}
```

---

## Tips and Best Practices

### Performance
- Avoid creating new objects every tick
- Use caches for expensive calculations
- Limit nested loops

### Strategy
- Use roles to organize your units
- Balance resource collection and unit production
- Group units for stronger attacks
- Think about expansion to increase resources

### Debugging
- Use `console.log()` for debugging
- Always check for `null` values before use
- Test your bots against different opponents

---

## Limits and Restrictions

### JavaScript Sandbox
- No file system access
- No network access
- Execution time limited per tick (100ms max)
- Limited memory (128 MB)

### API
- Maximum 100 commands per tick
- Some actions cost resources
- Units have limited action range

---

## Support

For more help:
- Check the examples in `/examples`
- Visit the project wiki
- Join the community on Discord

**Happy coding and have fun! 🎮**

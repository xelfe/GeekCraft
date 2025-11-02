/**
 * Advanced Bot Example for GeekCraft
 * 
 * This bot demonstrates more advanced strategies including:
 * - Resource management
 * - Unit roles and assignments
 * - Building construction
 * - Combat tactics
 * 
 * @author GeekCraft Team
 * @version 2.0.0
 */

class AdvancedBot {
    constructor() {
        this.name = "AdvancedBot";
        this.version = "2.0.0";
        this.unitRoles = new Map(); // Track unit roles
        this.resourceGoal = 1000;
        this.combatSquads = [];
    }

    onInit() {
        console.log(`${this.name} v${this.version} initialized with advanced AI!`);
    }

    onTick(gameState) {
        const myUnits = gameState.getMyUnits();
        const myResources = gameState.getMyResources();
        
        // Strategy 1: Assign roles to units
        this.assignRoles(myUnits, gameState);
        
        // Strategy 2: Manage workers
        this.manageWorkers(myUnits, gameState);
        
        // Strategy 3: Build structures if we have enough resources
        if (myResources.minerals >= 100) {
            this.buildStructures(gameState);
        }
        
        // Strategy 4: Manage combat units
        this.manageCombatUnits(myUnits, gameState);
        
        // Strategy 5: Produce new units
        if (myResources.minerals >= 50) {
            this.produceUnits(gameState);
        }
    }

    /**
     * Assign roles to units (worker or soldier)
     */
    assignRoles(units, gameState) {
        for (const unit of units) {
            if (!this.unitRoles.has(unit.id)) {
                // Assign role based on number of units
                const workerCount = Array.from(this.unitRoles.values())
                    .filter(role => role === 'worker').length;
                
                if (workerCount < 5) {
                    this.unitRoles.set(unit.id, 'worker');
                    console.log(`Unit ${unit.id} assigned as worker`);
                } else {
                    this.unitRoles.set(unit.id, 'soldier');
                    console.log(`Unit ${unit.id} assigned as soldier`);
                }
            }
        }
    }

    /**
     * Manage worker-type units
     */
    manageWorkers(units, gameState) {
        const workers = units.filter(u => this.unitRoles.get(u.id) === 'worker');
        
        for (const worker of workers) {
            if (worker.isIdle()) {
                if (!worker.isCarryingResource()) {
                    // Go get resources
                    const resource = gameState.findNearestResource(worker.position);
                    
                    if (resource) {
                        worker.moveTo(resource.position);
                        worker.harvest(resource);
                    }
                } else {
                    // Deposit resources at base
                    const base = gameState.getMyMainBase();
                    if (base) {
                        worker.moveTo(base.position);
                        worker.deposit();
                    }
                }
            }
        }
    }

    /**
     * Build strategic structures
     */
    buildStructures(gameState) {
        const bases = gameState.getMyBases();
        
        // Build an expansion if we have fewer than 2 bases
        if (bases.length < 2) {
            const expansionLocation = gameState.findExpansionLocation();
            if (expansionLocation) {
                gameState.buildStructure('base', expansionLocation);
                console.log(`Building expansion base at ${expansionLocation}`);
            }
        }
        
        // Build defenses around the main base
        const mainBase = gameState.getMyMainBase();
        if (mainBase && gameState.getMyResources().minerals >= 75) {
            const defensePositions = this.findDefensePositions(mainBase);
            for (const pos of defensePositions) {
                if (!gameState.isStructureAt(pos)) {
                    gameState.buildStructure('turret', pos);
                    console.log(`Building turret at ${pos}`);
                    break; // One at a time
                }
            }
        }
    }

    /**
     * Find optimal positions for defenses
     */
    findDefensePositions(base) {
        const positions = [];
        const radius = 5;
        const directions = [
            { x: radius, y: 0 },
            { x: -radius, y: 0 },
            { x: 0, y: radius },
            { x: 0, y: -radius }
        ];
        
        for (const dir of directions) {
            positions.push({
                x: base.position.x + dir.x,
                y: base.position.y + dir.y
            });
        }
        
        return positions;
    }

    /**
     * Manage combat units
     */
    manageCombatUnits(units, gameState) {
        const soldiers = units.filter(u => this.unitRoles.get(u.id) === 'soldier');
        
        // Organize soldiers into squads of 3
        if (soldiers.length >= 3) {
            const enemies = gameState.findEnemyUnits();
            
            if (enemies.length > 0) {
                // Attack the nearest enemy
                const target = this.findNearestEnemy(soldiers[0].position, enemies);
                
                for (const soldier of soldiers) {
                    soldier.attack(target);
                }
                
                console.log(`Squad of ${soldiers.length} soldiers attacking enemy ${target.id}`);
            } else {
                // Patrol if no enemies
                const patrolPoints = gameState.getPatrolPoints();
                
                for (let i = 0; i < soldiers.length; i++) {
                    const point = patrolPoints[i % patrolPoints.length];
                    soldiers[i].moveTo(point);
                }
            }
        }
    }

    /**
     * Find the nearest enemy
     */
    findNearestEnemy(position, enemies) {
        let nearest = enemies[0];
        let minDistance = this.distance(position, nearest.position);
        
        for (const enemy of enemies) {
            const dist = this.distance(position, enemy.position);
            if (dist < minDistance) {
                minDistance = dist;
                nearest = enemy;
            }
        }
        
        return nearest;
    }

    /**
     * Calculate distance between two positions
     */
    distance(pos1, pos2) {
        const dx = pos1.x - pos2.x;
        const dy = pos1.y - pos2.y;
        return Math.sqrt(dx * dx + dy * dy);
    }

    /**
     * Produce new units
     */
    produceUnits(gameState) {
        const bases = gameState.getMyBases();
        const myResources = gameState.getMyResources();
        
        for (const base of bases) {
            if (base.canProduceUnit() && myResources.minerals >= 50) {
                // Decide which unit type to produce
                const workerCount = Array.from(this.unitRoles.values())
                    .filter(role => role === 'worker').length;
                
                const unitType = workerCount < 5 ? 'worker' : 'soldier';
                base.produceUnit(unitType);
                console.log(`Producing unit of type ${unitType}`);
                break; // One unit at a time
            }
        }
    }

    onUnitDestroyed(unit) {
        this.unitRoles.delete(unit.id);
        console.log(`Unit ${unit.id} destroyed, reassigning roles...`);
    }

    onResourceCollected(unit, resource, amount) {
        const currentResources = gameState.getMyResources();
        console.log(`Total resources: ${currentResources.minerals} (goal: ${this.resourceGoal})`);
    }
}

module.exports = AdvancedBot;

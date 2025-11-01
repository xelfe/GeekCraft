/**
 * Advanced Bot Example for GeekCraft
 * 
 * Ce bot démontre des stratégies plus avancées incluant :
 * - Gestion des ressources
 * - Rôles et assignations d'unités
 * - Construction de bâtiments
 * - Tactiques de combat
 * 
 * @author GeekCraft Team
 * @version 2.0.0
 */

class AdvancedBot {
    constructor() {
        this.name = "AdvancedBot";
        this.version = "2.0.0";
        this.unitRoles = new Map(); // Suivre les rôles des unités
        this.resourceGoal = 1000;
        this.combatSquads = [];
    }

    onInit() {
        console.log(`${this.name} v${this.version} initialisé avec IA avancée!`);
    }

    onTick(gameState) {
        const myUnits = gameState.getMyUnits();
        const myResources = gameState.getMyResources();
        
        // Stratégie 1 : Assigner des rôles aux unités
        this.assignRoles(myUnits, gameState);
        
        // Stratégie 2 : Gérer les travailleurs
        this.manageWorkers(myUnits, gameState);
        
        // Stratégie 3 : Construire des structures si on a assez de ressources
        if (myResources.minerals >= 100) {
            this.buildStructures(gameState);
        }
        
        // Stratégie 4 : Gérer les unités de combat
        this.manageCombatUnits(myUnits, gameState);
        
        // Stratégie 5 : Produire de nouvelles unités
        if (myResources.minerals >= 50) {
            this.produceUnits(gameState);
        }
    }

    /**
     * Assigne des rôles aux unités (travailleur ou soldat)
     */
    assignRoles(units, gameState) {
        for (const unit of units) {
            if (!this.unitRoles.has(unit.id)) {
                // Assigner le rôle en fonction du nombre d'unités
                const workerCount = Array.from(this.unitRoles.values())
                    .filter(role => role === 'worker').length;
                
                if (workerCount < 5) {
                    this.unitRoles.set(unit.id, 'worker');
                    console.log(`Unité ${unit.id} assignée comme travailleur`);
                } else {
                    this.unitRoles.set(unit.id, 'soldier');
                    console.log(`Unité ${unit.id} assignée comme soldat`);
                }
            }
        }
    }

    /**
     * Gère les unités de type travailleur
     */
    manageWorkers(units, gameState) {
        const workers = units.filter(u => this.unitRoles.get(u.id) === 'worker');
        
        for (const worker of workers) {
            if (worker.isIdle()) {
                if (!worker.isCarryingResource()) {
                    // Aller chercher des ressources
                    const resource = gameState.findNearestResource(worker.position);
                    
                    if (resource) {
                        worker.moveTo(resource.position);
                        worker.harvest(resource);
                    }
                } else {
                    // Déposer les ressources à la base
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
     * Construit des structures stratégiques
     */
    buildStructures(gameState) {
        const bases = gameState.getMyBases();
        
        // Construire une expansion si on a moins de 2 bases
        if (bases.length < 2) {
            const expansionLocation = gameState.findExpansionLocation();
            if (expansionLocation) {
                gameState.buildStructure('base', expansionLocation);
                console.log(`Construction d'une base d'expansion à ${expansionLocation}`);
            }
        }
        
        // Construire des défenses autour de la base principale
        const mainBase = gameState.getMyMainBase();
        if (mainBase && gameState.getMyResources().minerals >= 75) {
            const defensePositions = this.findDefensePositions(mainBase);
            for (const pos of defensePositions) {
                if (!gameState.isStructureAt(pos)) {
                    gameState.buildStructure('turret', pos);
                    console.log(`Construction d'une tourelle à ${pos}`);
                    break; // Une à la fois
                }
            }
        }
    }

    /**
     * Trouve les positions optimales pour les défenses
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
     * Gère les unités de combat
     */
    manageCombatUnits(units, gameState) {
        const soldiers = units.filter(u => this.unitRoles.get(u.id) === 'soldier');
        
        // Organiser les soldats en escouades de 3
        if (soldiers.length >= 3) {
            const enemies = gameState.findEnemyUnits();
            
            if (enemies.length > 0) {
                // Attaquer l'ennemi le plus proche
                const target = this.findNearestEnemy(soldiers[0].position, enemies);
                
                for (const soldier of soldiers) {
                    soldier.attack(target);
                }
                
                console.log(`Escouade de ${soldiers.length} soldats attaque l'ennemi ${target.id}`);
            } else {
                // Patrouiller si aucun ennemi
                const patrolPoints = gameState.getPatrolPoints();
                
                for (let i = 0; i < soldiers.length; i++) {
                    const point = patrolPoints[i % patrolPoints.length];
                    soldiers[i].moveTo(point);
                }
            }
        }
    }

    /**
     * Trouve l'ennemi le plus proche
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
     * Calcule la distance entre deux positions
     */
    distance(pos1, pos2) {
        const dx = pos1.x - pos2.x;
        const dy = pos1.y - pos2.y;
        return Math.sqrt(dx * dx + dy * dy);
    }

    /**
     * Produit de nouvelles unités
     */
    produceUnits(gameState) {
        const bases = gameState.getMyBases();
        const myResources = gameState.getMyResources();
        
        for (const base of bases) {
            if (base.canProduceUnit() && myResources.minerals >= 50) {
                // Décider quel type d'unité produire
                const workerCount = Array.from(this.unitRoles.values())
                    .filter(role => role === 'worker').length;
                
                const unitType = workerCount < 5 ? 'worker' : 'soldier';
                base.produceUnit(unitType);
                console.log(`Production d'une unité de type ${unitType}`);
                break; // Une unité à la fois
            }
        }
    }

    onUnitDestroyed(unit) {
        this.unitRoles.delete(unit.id);
        console.log(`Unité ${unit.id} détruite, réassignation des rôles...`);
    }

    onResourceCollected(unit, resource, amount) {
        const currentResources = gameState.getMyResources();
        console.log(`Ressources totales : ${currentResources.minerals} (objectif: ${this.resourceGoal})`);
    }
}

module.exports = AdvancedBot;

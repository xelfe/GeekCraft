# GeekCraft - Référence API JavaScript

## Table des matières
- [Introduction](#introduction)
- [Structure d'un Bot](#structure-dun-bot)
- [API GameState](#api-gamestate)
- [API Unit](#api-unit)
- [API Structure](#api-structure)
- [API Resource](#api-resource)
- [Événements](#événements)
- [Exemples](#exemples)

---

## Introduction

GeekCraft permet aux joueurs de contrôler leurs unités et bases via une API JavaScript. Votre bot est une classe JavaScript qui répond aux événements du jeu et contrôle les unités via l'état du jeu (GameState).

---

## Structure d'un Bot

Votre bot doit être une classe JavaScript exportée avec les méthodes suivantes :

```javascript
class MyBot {
    constructor() {
        // Initialisation de votre bot
        this.name = "MyBot";
        this.version = "1.0.0";
    }

    onInit() {
        // Appelé au démarrage du jeu
    }

    onTick(gameState) {
        // Appelé à chaque tick (boucle principale)
        // C'est ici que vous implémentez votre stratégie
    }

    onUnitCreated(unit) {
        // Appelé quand une unité est créée
    }

    onUnitDestroyed(unit) {
        // Appelé quand une unité est détruite
    }

    onResourceCollected(unit, resource, amount) {
        // Appelé quand des ressources sont collectées
    }

    onStructureBuilt(structure) {
        // Appelé quand une structure est construite
    }

    onUnitAttack(attacker, target) {
        // Appelé lors d'une attaque
    }

    onUnitDamaged(unit, damage) {
        // Appelé quand une unité subit des dégâts
    }
}

module.exports = MyBot;
```

---

## API GameState

L'objet `gameState` est passé à la méthode `onTick()` et fournit toutes les informations sur l'état actuel du jeu.

### Gestion des Unités

#### `gameState.getMyUnits()`
Retourne un tableau de toutes vos unités.

```javascript
const myUnits = gameState.getMyUnits();
console.log(`J'ai ${myUnits.length} unités`);
```

**Retour :** `Unit[]`

---

#### `gameState.getEnemyUnits()`
Retourne un tableau de toutes les unités ennemies visibles.

```javascript
const enemies = gameState.getEnemyUnits();
if (enemies.length > 0) {
    console.log("Ennemis détectés !");
}
```

**Retour :** `Unit[]`

---

#### `gameState.getAllUnits()`
Retourne toutes les unités dans le jeu (alliées et ennemies).

**Retour :** `Unit[]`

---

#### `gameState.getUnitById(id)`
Récupère une unité spécifique par son ID.

**Paramètres :**
- `id` (string) : L'identifiant unique de l'unité

**Retour :** `Unit | null`

---

### Gestion des Ressources

#### `gameState.getMyResources()`
Retourne vos ressources actuelles.

```javascript
const resources = gameState.getMyResources();
console.log(`Minéraux: ${resources.minerals}, Gaz: ${resources.gas}`);
```

**Retour :** 
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
Trouve la ressource la plus proche d'une position donnée.

**Paramètres :**
- `position` (Object) : `{x: number, y: number}`

**Retour :** `Resource | null`

```javascript
const resource = gameState.findNearestResource(unit.position);
if (resource) {
    unit.moveTo(resource.position);
}
```

---

#### `gameState.getAllResources()`
Retourne toutes les ressources sur la carte.

**Retour :** `Resource[]`

---

### Gestion des Bases et Structures

#### `gameState.getMyBases()`
Retourne toutes vos bases.

**Retour :** `Structure[]`

---

#### `gameState.getMyMainBase()`
Retourne votre base principale.

**Retour :** `Structure | null`

---

#### `gameState.buildStructure(type, position)`
Construit une structure à une position donnée.

**Paramètres :**
- `type` (string) : Type de structure (`'base'`, `'turret'`, `'factory'`)
- `position` (Object) : `{x: number, y: number}`

**Retour :** `boolean` - `true` si la construction a réussi

```javascript
const pos = {x: 100, y: 100};
if (gameState.buildStructure('turret', pos)) {
    console.log("Tourelle construite !");
}
```

---

#### `gameState.isStructureAt(position)`
Vérifie s'il y a une structure à une position donnée.

**Paramètres :**
- `position` (Object) : `{x: number, y: number}`

**Retour :** `boolean`

---

### Informations sur la Carte

#### `gameState.getMapSize()`
Retourne les dimensions de la carte.

**Retour :** 
```javascript
{
    width: number,
    height: number
}
```

---

#### `gameState.findExpansionLocation()`
Trouve un emplacement optimal pour une expansion.

**Retour :** `{x: number, y: number} | null`

---

#### `gameState.getPatrolPoints()`
Retourne des points de patrouille recommandés.

**Retour :** `Array<{x: number, y: number}>`

---

#### `gameState.isWalkable(position)`
Vérifie si une position est accessible.

**Paramètres :**
- `position` (Object) : `{x: number, y: number}`

**Retour :** `boolean`

---

## API Unit

Les objets `Unit` représentent les unités du jeu (travailleurs, soldats, etc.).

### Propriétés

```javascript
unit.id          // string - Identifiant unique
unit.type        // string - Type d'unité ('worker', 'soldier', etc.)
unit.position    // {x: number, y: number} - Position actuelle
unit.health      // number - Points de vie actuels
unit.maxHealth   // number - Points de vie maximum
unit.owner       // string - Propriétaire de l'unité
unit.speed       // number - Vitesse de déplacement
```

---

### Méthodes de Déplacement

#### `unit.moveTo(position)`
Déplace l'unité vers une position.

**Paramètres :**
- `position` (Object) : `{x: number, y: number}`

**Retour :** `boolean`

```javascript
unit.moveTo({x: 50, y: 100});
```

---

#### `unit.stop()`
Arrête toutes les actions de l'unité.

**Retour :** `void`

---

#### `unit.isIdle()`
Vérifie si l'unité est inactive.

**Retour :** `boolean`

```javascript
if (unit.isIdle()) {
    // Assigner une nouvelle tâche
}
```

---

### Actions sur les Ressources

#### `unit.harvest(resource)`
Fait récolter une ressource par l'unité.

**Paramètres :**
- `resource` (Resource) : La ressource à récolter

**Retour :** `boolean`

```javascript
const resource = gameState.findNearestResource(unit.position);
if (resource) {
    unit.harvest(resource);
}
```

---

#### `unit.deposit()`
Dépose les ressources transportées à la base la plus proche.

**Retour :** `boolean`

---

#### `unit.isCarryingResource()`
Vérifie si l'unité transporte des ressources.

**Retour :** `boolean`

---

#### `unit.getCarriedAmount()`
Retourne la quantité de ressources transportées.

**Retour :** `number`

---

### Actions de Combat

#### `unit.attack(target)`
Attaque une cible (unité ou structure).

**Paramètres :**
- `target` (Unit | Structure) : La cible à attaquer

**Retour :** `boolean`

```javascript
const enemies = gameState.getEnemyUnits();
if (enemies.length > 0) {
    unit.attack(enemies[0]);
}
```

---

#### `unit.defend(position)`
Met l'unité en position défensive.

**Paramètres :**
- `position` (Object) : `{x: number, y: number}`

**Retour :** `boolean`

---

#### `unit.canAttack(target)`
Vérifie si l'unité peut attaquer une cible.

**Paramètres :**
- `target` (Unit | Structure)

**Retour :** `boolean`

---

### Informations

#### `unit.getDistanceTo(position)`
Calcule la distance jusqu'à une position.

**Paramètres :**
- `position` (Object) : `{x: number, y: number}`

**Retour :** `number`

```javascript
const dist = unit.getDistanceTo({x: 100, y: 100});
console.log(`Distance: ${dist}`);
```

---

## API Structure

Les structures incluent les bases, tourelles, usines, etc.

### Propriétés

```javascript
structure.id          // string - Identifiant unique
structure.type        // string - Type de structure
structure.position    // {x: number, y: number}
structure.health      // number
structure.maxHealth   // number
structure.owner       // string
```

### Méthodes

#### `structure.produceUnit(unitType)`
Produit une nouvelle unité (si c'est une base ou usine).

**Paramètres :**
- `unitType` (string) : Type d'unité à produire

**Retour :** `boolean`

```javascript
const base = gameState.getMyMainBase();
if (base.canProduceUnit()) {
    base.produceUnit('worker');
}
```

---

#### `structure.canProduceUnit()`
Vérifie si la structure peut produire une unité.

**Retour :** `boolean`

---

## API Resource

Représente une ressource sur la carte.

### Propriétés

```javascript
resource.id        // string
resource.type      // string - 'minerals' ou 'gas'
resource.position  // {x: number, y: number}
resource.amount    // number - Quantité restante
```

---

## Événements

### Cycle de Vie

1. **`onInit()`** - Appelé une fois au démarrage
2. **`onTick(gameState)`** - Appelé à chaque frame (60 FPS)
3. Événements spécifiques déclenchés selon les actions

### Événements d'Unités

- **`onUnitCreated(unit)`** - Nouvelle unité créée
- **`onUnitDestroyed(unit)`** - Unité détruite
- **`onUnitAttack(attacker, target)`** - Attaque effectuée
- **`onUnitDamaged(unit, damage)`** - Unité endommagée

### Événements de Ressources

- **`onResourceCollected(unit, resource, amount)`** - Ressources collectées

### Événements de Structures

- **`onStructureBuilt(structure)`** - Structure construite
- **`onStructureDestroyed(structure)`** - Structure détruite

---

## Exemples

### Exemple 1 : Bot Récolteur Simple

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

### Exemple 2 : Bot Militaire

```javascript
class MilitaryBot {
    onTick(gameState) {
        const soldiers = gameState.getMyUnits()
            .filter(u => u.type === 'soldier');
        
        const enemies = gameState.getEnemyUnits();
        
        if (enemies.length > 0) {
            // Attaquer en groupe
            const target = enemies[0];
            for (const soldier of soldiers) {
                if (soldier.canAttack(target)) {
                    soldier.attack(target);
                }
            }
        } else {
            // Patrouiller
            const points = gameState.getPatrolPoints();
            soldiers.forEach((s, i) => {
                s.moveTo(points[i % points.length]);
            });
        }
    }
}
```

### Exemple 3 : Bot Économique

```javascript
class EconomyBot {
    constructor() {
        this.targetWorkers = 10;
    }
    
    onTick(gameState) {
        const resources = gameState.getMyResources();
        const workers = gameState.getMyUnits()
            .filter(u => u.type === 'worker');
        
        // Produire des travailleurs
        if (workers.length < this.targetWorkers && 
            resources.minerals >= 50) {
            const base = gameState.getMyMainBase();
            if (base.canProduceUnit()) {
                base.produceUnit('worker');
            }
        }
        
        // Construire une expansion
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

## Conseils et Bonnes Pratiques

### Performance
- Évitez de créer de nouveaux objets à chaque tick
- Utilisez des caches pour les calculs coûteux
- Limitez les boucles imbriquées

### Stratégie
- Utilisez des rôles pour organiser vos unités
- Équilibrez collecte de ressources et production d'unités
- Groupez les unités pour des attaques plus fortes
- Pensez à l'expansion pour augmenter les ressources

### Débogage
- Utilisez `console.log()` pour déboguer
- Vérifiez toujours les valeurs `null` avant utilisation
- Testez vos bots contre différents adversaires

---

## Limites et Restrictions

### Sandbox JavaScript
- Pas d'accès au système de fichiers
- Pas d'accès réseau
- Temps d'exécution limité par tick (100ms max)
- Mémoire limitée (128 MB)

### API
- Maximum 100 commandes par tick
- Certaines actions coûtent des ressources
- Les unités ont une portée d'action limitée

---

## Support

Pour plus d'aide :
- Consultez les exemples dans `/examples`
- Visitez le wiki du projet
- Rejoignez la communauté sur Discord

**Bon codage et amusez-vous bien ! 🎮**

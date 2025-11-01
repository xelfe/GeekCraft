# GeekCraft - Résumé du Projet

## Vue d'ensemble

**GeekCraft** est un jeu de programmation inspiré de **Screeps** et **Starcraft**, où les joueurs programment des bots en JavaScript pour contrôler des unités dans un environnement de stratégie en temps réel.

### Concept Clé

Le moteur de jeu est **headless par conception** - il ne fournit **aucun rendu graphique intégré**. Les joueurs sont **complètement libres** de créer leur propre interface de visualisation en utilisant la technologie de leur choix.

## Architecture

### Backend - Moteur de Jeu (Rust)

Le serveur GeekCraft est écrit en Rust pour des raisons de performance et de sécurité :

```
GeekCraft (Rust)
├── Moteur de simulation
│   ├── Monde de jeu (grille, ressources)
│   ├── Entités (unités, bâtiments)
│   └── Logique de jeu (déplacement, combat, ressources)
│
├── Sandbox JavaScript
│   ├── Exécution sécurisée des bots
│   ├── Limitations de temps et mémoire
│   └── API de scripting
│
└── Serveur réseau
    ├── WebSocket pour temps réel
    ├── REST API pour requêtes
    └── Diffusion d'événements
```

**Caractéristiques :**
- ⚡ Performance élevée
- 🔒 Exécution sécurisée des scripts
- 🌐 Communication temps réel
- 📊 Simulation déterministe

### Frontend - Bots des Joueurs (JavaScript)

Les joueurs programment leurs bots en JavaScript :

```javascript
class MonBot {
    onTick(gameState) {
        // Stratégie du bot
        const units = gameState.getMyUnits();
        for (const unit of units) {
            // Logique de contrôle
        }
    }
}
```

**API Disponible :**
- 🎮 Contrôle des unités
- 📊 Accès au state du jeu
- 🏗️ Construction de bâtiments
- ⚔️ Gestion du combat
- 💎 Collecte de ressources

### Visualisation - Client Personnalisé (Au choix)

Les joueurs créent leur propre interface :

**Options disponibles :**

#### Web
- **HTML5 Canvas** (exemple fourni)
- **React/Vue/Angular**
- **Three.js** (3D)
- **Phaser/PixiJS** (2D gaming)

#### Desktop
- **Unity**
- **Godot**
- **Electron**
- **Tauri**

#### Terminal
- **Blessed** (Node.js)
- **Rich** (Python)
- **Cursive** (Rust)

#### Autre
- Tout ce qui peut se connecter via WebSocket !

## Communication

### Protocol WebSocket

```
Joueur Bot (JS) ←→ Serveur (Rust) ←→ Viewer (Libre)
                     ↓
                  Simulation
```

**Messages types :**

```javascript
// Serveur → Client
{
    "type": "gameState",
    "data": {
        "tick": 123,
        "players": [...],
        "units": [...],
        "resources": [...]
    }
}

// Client → Serveur  
{
    "type": "command",
    "data": {
        "unitId": 42,
        "action": "moveTo",
        "target": {"x": 100, "y": 200}
    }
}
```

## Flux de Jeu

1. **Joueur code son bot** en JavaScript
2. **Bot soumis au serveur** via API
3. **Serveur exécute le bot** à chaque tick
4. **État du jeu mis à jour** selon les actions
5. **Serveur diffuse l'état** via WebSocket
6. **Viewer affiche** le jeu (si connecté)

```
[Code Bot] → [Serveur] → [Simulation]
                ↓
            [WebSocket]
                ↓
         [Votre Viewer]
```

## Exemples Fournis

### 1. Bots JavaScript

#### `basic_bot.js`
Bot simple pour débuter :
- Déplacement vers ressources
- Réponse aux événements
- Gestion basique

#### `advanced_bot.js`
Stratégies avancées :
- Rôles d'unités (workers, soldiers)
- Gestion de ressources
- Construction de bases
- Tactiques de combat

#### `template_bot.js`
Template vide pour démarrer

### 2. Viewer HTML

Un exemple complet de client web :
- Interface moderne (HTML/CSS/JS)
- Canvas pour rendu 2D
- Connexion WebSocket
- Contrôles de caméra
- Sélection d'unités
- Console de logs

**Localisation :** `examples/viewer/`

### 3. Documentation

#### `API_REFERENCE.md`
Documentation complète de l'API JavaScript pour les bots

## Structure du Projet

```
GeekCraft/
├── src/                    # Code source Rust
│   ├── main.rs            # Point d'entrée
│   ├── lib.rs             # Bibliothèque
│   ├── game/              # Moteur de jeu
│   │   ├── world.rs       # Monde de jeu
│   │   ├── entities.rs    # Entités
│   │   └── simulation.rs  # Simulation
│   ├── api/               # API de scripting
│   │   ├── scripting.rs   # Interface bots
│   │   └── events.rs      # Événements
│   ├── network/           # Serveur réseau
│   │   └── server.rs      # WebSocket/REST
│   └── scripting/         # Sandbox JS
│       └── sandbox.rs     # Exécution sécurisée
│
├── examples/              # Exemples
│   ├── basic_bot.js      # Bot simple
│   ├── advanced_bot.js   # Bot avancé
│   ├── template_bot.js   # Template
│   ├── API_REFERENCE.md  # Doc API
│   └── viewer/           # Viewer HTML exemple
│       ├── index.html
│       ├── viewer.js
│       ├── style.css
│       └── README.md
│
├── tests/                # Tests
├── assets/               # Ressources (optionnel)
├── Cargo.toml           # Config Rust
├── BUILD.md             # Guide de construction
└── README.md            # Documentation
```

## Roadmap

### Phase 1 : Fondations ✅
- [x] Structure du projet
- [x] Documentation de base
- [x] Exemples de bots JS
- [x] Viewer HTML exemple

### Phase 2 : Moteur de Base 🚧
- [ ] Simulation du monde
- [ ] Système d'entités
- [ ] Gestion des ressources
- [ ] Système de déplacement

### Phase 3 : Scripting ⏳
- [ ] Sandbox JavaScript (Boa/Deno)
- [ ] API de scripting
- [ ] Système d'événements
- [ ] Limitations de sécurité

### Phase 4 : Réseau ⏳
- [ ] Serveur WebSocket
- [ ] REST API
- [ ] Authentification
- [ ] Multi-joueurs

### Phase 5 : Gameplay 📅
- [ ] Système de combat
- [ ] Construction de bâtiments
- [ ] Tech tree
- [ ] Fog of war

### Phase 6 : Avancé 📅
- [ ] Replays
- [ ] Classements
- [ ] Tournois
- [ ] IA de référence

## Technologies Utilisées

### Backend
- **Rust** 1.70+ - Langage principal
- **Tokio** - Runtime async
- **Warp/Actix** - Serveur web
- **Boa/Deno** - Moteur JavaScript
- **Serde** - Sérialisation JSON

### Exemples Frontend
- **HTML5/CSS3/JavaScript** - Viewer de base
- **Canvas API** - Rendu 2D
- **WebSocket API** - Communication temps réel

## Comment Contribuer

1. **Moteur de jeu** : Implémenter la simulation
2. **API JavaScript** : Enrichir les fonctionnalités bots
3. **Documentation** : Améliorer les guides
4. **Exemples** : Créer plus de bots et viewers
5. **Tests** : Ajouter des tests unitaires et d'intégration

## Philosophie du Projet

### Liberté de Visualisation

GeekCraft ne vous impose **aucune contrainte graphique**. Vous êtes libre de :
- Créer un rendu 3D spectaculaire
- Faire une interface terminal minimaliste
- Développer une app mobile
- Même ne rien afficher et juste logger !

### Open Source et Éducatif

Le projet vise à :
- Enseigner la programmation
- Promouvoir Rust et JavaScript
- Créer une communauté de développeurs
- Partager les connaissances

### Performance et Sécurité

- Code Rust pour la vitesse
- Sandbox pour la sécurité
- API claire et documentée
- Tests et validation

## Démarrage Rapide

```bash
# 1. Clone le projet
git clone https://github.com/xelfe/GeekCraft.git
cd GeekCraft/GeekCraft

# 2. Build le serveur
cargo build --release

# 3. Lance le serveur
cargo run --release

# 4. Ouvre le viewer exemple
open examples/viewer/index.html

# 5. Crée ton bot
cp examples/template_bot.js my_bot.js
# Édite my_bot.js avec ton code

# 6. Soumets ton bot (via API ou interface)
```

## Ressources

- **Documentation** : `README.md`, `BUILD.md`, `API_REFERENCE.md`
- **Exemples** : Dossier `examples/`
- **Repository** : https://github.com/xelfe/GeekCraft
- **Licence** : MIT

## Contact et Support

- **Issues** : GitHub Issues
- **Discussions** : GitHub Discussions
- **Contributions** : Pull Requests bienvenues !

---

**GeekCraft** - Votre jeu, votre code, votre vision ! 🎮🚀

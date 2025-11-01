# GeekCraft

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Description
GeekCraft est un jeu de programmation inspiré de **Screeps** et **Starcraft**, où les joueurs programment leurs propres bots en **JavaScript** pour contrôler des unités dans un environnement de jeu en temps réel. 

**Le moteur de jeu est headless par conception** - il ne fournit pas d'interface graphique intégrée. Les joueurs sont **libres de créer leur propre visualisation** en utilisant la technologie de leur choix (web, desktop, terminal, etc.). Un **viewer HTML de base** est fourni comme exemple et point de départ.

## Project Structure
```
GeekCraft
├── src
│   ├── main.rs          # Point d'entrée, initialise le serveur et démarre le moteur
│   ├── lib.rs           # Bibliothèque principale, exporte les modules
│   ├── game
│   │   ├── mod.rs       # Module de jeu
│   │   ├── world.rs     # Structures et fonctions du monde de jeu
│   │   ├── entities.rs   # Définit les entités (unités, bâtiments)
│   │   └── simulation.rs # Logique de simulation (temps, interactions)
│   ├── api
│   │   ├── mod.rs       # Module API
│   │   ├── scripting.rs  # Système de scripting pour les bots
│   │   └── events.rs     # Événements de jeu
│   ├── network
│   │   ├── mod.rs       # Module réseau
│   │   └── server.rs     # Communication client-serveur (WebSocket/REST)
│   └── scripting
│       ├── mod.rs       # Module de scripting
│       └── sandbox.rs    # Environnement sécurisé pour exécuter les scripts
├── assets
│   └── textures          # Textures d'exemple (optionnel)
├── examples
│   ├── basic_bot.js      # Bot basique en JavaScript
│   ├── advanced_bot.js   # Bot avancé avec stratégies
│   ├── template_bot.js   # Template pour créer son bot
│   ├── API_REFERENCE.md  # Documentation de l'API JavaScript
│   └── viewer
│       ├── index.html    # Viewer HTML de base
│       ├── viewer.js     # Logique du viewer
│       └── style.css     # Styles du viewer
├── tests
│   └── integration_tests.rs # Tests d'intégration
├── Cargo.toml            # Configuration Cargo
├── BUILD.md              # Guide de construction détaillé
└── README.md             # Documentation principale
```

## Caractéristiques

- 🎮 **Moteur de jeu Rust** - Performance et fiabilité
- 🤖 **Programmation en JavaScript** - Créez vos bots avec un langage familier
- 🌐 **API WebSocket/REST** - Communication temps réel avec vos clients
- 🎨 **Headless par conception** - Aucun graphisme intégré, totale liberté de visualisation
- 🖥️ **Viewer HTML d'exemple** - Point de départ pour créer votre propre interface
- 🔒 **Sandbox JavaScript sécurisé** - Exécution sûre des scripts utilisateurs
- 📊 **Simulation de monde** - Gestion des ressources, unités et combats
- 🔌 **API ouverte** - Créez votre client avec la technologie de votre choix (React, Unity, terminal, etc.)

## Installation

### Prérequis
- Rust 1.70+ ([installer Rust](https://rustup.rs))
- Cargo (inclus avec Rust)

### Étapes d'installation

1. **Cloner le repository**
   ```bash
   git clone https://github.com/xelfe/GeekCraft.git
   cd GeekCraft/GeekCraft
   ```

2. **Construire le projet**
   ```bash
   cargo build --release
   ```

3. **Lancer le serveur de jeu**
   ```bash
   cargo run --release
   ```

4. **Tester avec un bot d'exemple**
   ```bash
   # Le serveur démarre sur http://localhost:3030
   # Soumettez votre bot via l'API ou l'interface web
   ```

## Démarrage Rapide

### Créer votre premier bot

1. Copiez le template de bot :
   ```bash
   cp examples/template_bot.js my_bot.js
   ```

2. Éditez `my_bot.js` avec votre stratégie :
   ```javascript
   class MyBot {
       onTick(gameState) {
           const units = gameState.getMyUnits();
           // Votre logique ici
       }
   }
   module.exports = MyBot;
   ```

3. Soumettez votre bot au serveur via l'API

### Exemples disponibles

- **`basic_bot.js`** - Bot simple pour débuter
- **`advanced_bot.js`** - Stratégies avancées (rôles, construction, combat)
- **`template_bot.js`** - Template vide pour créer votre bot
- **`API_REFERENCE.md`** - Documentation complète de l'API

## Architecture Technique

### Backend (Rust)
- **Moteur de jeu** : Simulation du monde, gestion des entités
- **Serveur API** : WebSocket + REST pour la communication
- **Sandbox JavaScript** : Exécution sécurisée des scripts utilisateurs
- **Headless** : Aucun rendu graphique intégré

### API JavaScript (pour les bots)
- Accès au state du jeu
- Contrôle des unités
- Gestion des ressources
- Construction de bâtiments
- Stratégies de combat

### Clients de visualisation
- **Viewer HTML de base** fourni en exemple
- **Créez le vôtre** : React, Vue, Unity, Godot, terminal ASCII, etc.
- **API WebSocket** pour recevoir les mises à jour en temps réel
- **Liberté totale** de design et de technologie

## Commandes utiles

```bash
# Développement
cargo run                    # Lancer en mode debug
cargo build --release        # Build optimisé
cargo test                   # Lancer les tests
cargo doc --open            # Générer et ouvrir la documentation

# Exemples
cargo run --example basic_bot
```

## Roadmap

- [x] Structure de base du projet
- [x] API JavaScript pour les bots
- [ ] Moteur de simulation du monde
- [ ] Serveur WebSocket/REST
- [ ] Sandbox JavaScript sécurisé
- [ ] Interface graphique de base
- [ ] Système de ressources
- [ ] Système de combat
- [ ] Multijoueur
- [ ] Replays et statistiques

## Contribution

Les contributions sont les bienvenues ! Voici comment participer :

1. Fork le projet
2. Créez une branche (`git checkout -b feature/AmazingFeature`)
3. Commit vos changements (`git commit -m 'Add AmazingFeature'`)
4. Push vers la branche (`git push origin feature/AmazingFeature`)
5. Ouvrez une Pull Request

## Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.

## Contact

Projet : [https://github.com/xelfe/GeekCraft](https://github.com/xelfe/GeekCraft)
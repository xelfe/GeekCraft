# GeekCraft - Structure Finale du Projet

## Vue d'ensemble

Le projet GeekCraft a été mis à jour pour refléter son architecture **headless** (sans graphismes intégrés), donnant aux joueurs la liberté totale de créer leurs propres interfaces de visualisation.

## Structure Actuelle

```
GeekCraft/
│
├── 📄 README.md              # Documentation principale du projet
├── 📄 BUILD.md               # Guide de construction détaillé
├── 📄 PROJECT_SUMMARY.md     # Résumé complet du projet
├── 📄 Cargo.toml             # Configuration Cargo/Rust
├── 📄 LICENSE                # Licence MIT
├── 📄 .gitignore             # Fichiers Git à ignorer
│
├── 📁 src/                   # Code source Rust
│   ├── main.rs              # Point d'entrée de l'application
│   ├── lib.rs               # Bibliothèque principale
│   │
│   ├── 📁 game/             # Moteur de jeu
│   │   ├── mod.rs           # Module de jeu
│   │   ├── world.rs         # Monde de jeu
│   │   ├── entities.rs      # Entités (unités, bâtiments)
│   │   └── simulation.rs    # Logique de simulation
│   │
│   ├── 📁 api/              # API de scripting
│   │   ├── mod.rs           # Module API
│   │   ├── scripting.rs     # Interface de scripting
│   │   └── events.rs        # Système d'événements
│   │
│   ├── 📁 network/          # Serveur réseau
│   │   ├── mod.rs           # Module réseau
│   │   └── server.rs        # Serveur WebSocket/REST
│   │
│   └── 📁 scripting/        # Sandbox JavaScript
│       ├── mod.rs           # Module de scripting
│       └── sandbox.rs       # Environnement sécurisé
│
├── 📁 examples/             # Exemples et documentation
│   │
│   ├── 📄 API_REFERENCE.md  # Documentation complète de l'API JavaScript
│   ├── 📄 basic_bot.js      # Bot basique pour débuter
│   ├── 📄 advanced_bot.js   # Bot avancé avec stratégies
│   ├── 📄 template_bot.js   # Template vide pour créer son bot
│   │
│   └── 📁 viewer/           # Viewer HTML de base (exemple)
│       ├── index.html       # Interface utilisateur
│       ├── viewer.js        # Logique du viewer
│       ├── style.css        # Styles CSS
│       └── README.md        # Documentation du viewer
│
├── 📁 tests/                # Tests
│   └── integration_tests.rs # Tests d'intégration
│
└── 📁 assets/               # Ressources (optionnel)
    └── textures             # Fichier placeholder
```

## Changements Effectués

### ✅ Supprimé (Architecture Headless)

- ❌ `src/graphics/` - Module de rendu graphique intégré
- ❌ `assets/shaders/` - Shaders graphiques
- ❌ `examples/basic_bot.rs` - Exemple Rust (remplacé par JS)

**Raison :** Le moteur est maintenant headless. Les joueurs créent leurs propres viewers.

### ✅ Ajouté

#### Exemples JavaScript
- ✅ `examples/basic_bot.js` - Bot simple pour débuter
- ✅ `examples/advanced_bot.js` - Stratégies avancées
- ✅ `examples/template_bot.js` - Template de démarrage
- ✅ `examples/API_REFERENCE.md` - Documentation API complète

#### Viewer HTML de Base
- ✅ `examples/viewer/index.html` - Interface utilisateur moderne
- ✅ `examples/viewer/viewer.js` - Client WebSocket avec rendu Canvas
- ✅ `examples/viewer/style.css` - Styles dark theme
- ✅ `examples/viewer/README.md` - Documentation du viewer

#### Documentation
- ✅ `PROJECT_SUMMARY.md` - Résumé complet du projet
- ✅ Mise à jour de `README.md` - Architecture headless
- ✅ Mise à jour de `BUILD.md` - Guide de construction

### ✅ Modifié

- 🔧 `src/lib.rs` - Suppression du module graphics
- 🔧 `README.md` - Refonte complète pour architecture headless
- 🔧 `BUILD.md` - Ajout d'exemples de démarrage

## Concepts Clés

### 1. Moteur Headless

Le serveur GeekCraft ne fournit **aucun rendu graphique**. Il est responsable de :
- ✅ Simulation du monde de jeu
- ✅ Exécution des bots JavaScript
- ✅ Gestion des ressources et entités
- ✅ Communication réseau (WebSocket/REST)

### 2. Liberté de Visualisation

Les joueurs sont **totalement libres** de créer leur propre interface :
- 🌐 **Web** : React, Vue, Three.js, Canvas, etc.
- 🖥️ **Desktop** : Unity, Godot, Electron, Tauri
- 💻 **Terminal** : ASCII art, Blessed, Rich
- 📱 **Mobile** : React Native, Flutter
- 🎨 **Autre** : Tout ce qui peut se connecter via WebSocket

### 3. Programmation en JavaScript

Les joueurs programment leurs bots en JavaScript avec :
- 📝 API intuitive et documentée
- 🔒 Exécution sécurisée (sandbox)
- ⚡ Événements en temps réel
- 🎮 Contrôle total des unités

## Flux de Travail

```
1. Joueur écrit son bot en JavaScript
   ↓
2. Bot soumis au serveur Rust
   ↓
3. Serveur exécute le bot (sandbox)
   ↓
4. Simulation du monde mise à jour
   ↓
5. État diffusé via WebSocket
   ↓
6. Viewer personnalisé affiche le jeu
```

## Pour Démarrer

### 1. Construire le Serveur

```bash
cd GeekCraft
cargo build --release
```

### 2. Lancer le Serveur

```bash
cargo run --release
# Serveur démarre sur ws://localhost:3030
```

### 3. Tester le Viewer

```bash
cd examples/viewer
open index.html  # ou double-cliquer
```

### 4. Créer Votre Bot

```bash
cp examples/template_bot.js my_bot.js
# Éditez my_bot.js avec votre stratégie
```

### 5. (Optionnel) Créer Votre Viewer

Utilisez la technologie de votre choix ! Le viewer HTML fourni est juste un exemple.

## Documentation Disponible

| Fichier | Description |
|---------|-------------|
| `README.md` | Documentation principale |
| `BUILD.md` | Guide de construction |
| `PROJECT_SUMMARY.md` | Résumé du projet |
| `examples/API_REFERENCE.md` | API JavaScript pour les bots |
| `examples/viewer/README.md` | Documentation du viewer HTML |

## Prochaines Étapes

### Phase Actuelle : Fondations ✅

- [x] Structure du projet
- [x] Documentation complète
- [x] Exemples de bots JavaScript
- [x] Viewer HTML de base
- [x] Architecture headless définie

### Prochaines Phases

1. **Moteur de Simulation** 🚧
   - Implémentation du monde de jeu
   - Système d'entités
   - Gestion des ressources
   - Logique de déplacement

2. **Sandbox JavaScript** 🚧
   - Intégration d'un moteur JS (Boa/Deno)
   - API de scripting fonctionnelle
   - Limitations de sécurité
   - Gestion d'événements

3. **Serveur Réseau** 🚧
   - WebSocket server
   - REST API
   - Authentification
   - Multi-joueurs

4. **Gameplay** 📅
   - Combat
   - Construction
   - Tech tree
   - Balance

## Technologies

### Backend (Rust)
- **Rust** 1.70+
- **Tokio** - Async runtime
- **Warp/Actix** - Web framework (à décider)
- **Boa/Deno** - Moteur JavaScript (à décider)
- **Serde** - JSON serialization

### Frontend (Exemples)
- **HTML5/CSS3/JavaScript** - Viewer de base
- **Canvas API** - Rendu 2D
- **WebSocket API** - Communication temps réel

## Licence

MIT License - Voir fichier `LICENSE`

## Contribution

Les contributions sont bienvenues ! Domaines prioritaires :
- 🎮 Implémentation du moteur de simulation
- 🔒 Sandbox JavaScript sécurisé
- 🌐 Serveur WebSocket/REST
- 📚 Documentation et tutoriels
- 🎨 Viewers alternatifs (React, Unity, etc.)

## Contact

- **Repository** : https://github.com/xelfe/GeekCraft
- **Issues** : GitHub Issues
- **Discussions** : GitHub Discussions

---

**GeekCraft** - Votre jeu, votre code, votre vision ! 🎮🚀

*Dernière mise à jour : 1er novembre 2025*

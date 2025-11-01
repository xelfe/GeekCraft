# Changelog - GeekCraft

Toutes les modifications notables du projet sont documentées ici.

## [0.1.0] - 2025-11-01

### Architecture Headless 🎮

Le projet a été refactoré pour adopter une architecture **headless** (sans graphiques intégrés).

### Ajouté ✅

#### Exemples JavaScript pour les Bots
- `examples/basic_bot.js` - Bot simple pour les débutants
  - Déplacement vers ressources
  - Gestion basique des unités
  - Réponse aux événements
  
- `examples/advanced_bot.js` - Bot avancé avec stratégies complexes
  - Système de rôles (workers, soldiers)
  - Gestion avancée des ressources
  - Construction de bases d'expansion
  - Tactiques de combat en groupe
  
- `examples/template_bot.js` - Template vide pour démarrer
  - Structure de base d'un bot
  - Méthodes à implémenter
  - Commentaires explicatifs

#### Viewer HTML de Base
- `examples/viewer/index.html` - Interface utilisateur moderne
  - Layout responsive
  - Panneaux configurables
  - Console de logs intégrée
  
- `examples/viewer/viewer.js` - Client WebSocket complet
  - Connexion au serveur
  - Rendu Canvas 2D
  - Gestion de caméra (zoom, déplacement)
  - Sélection d'unités
  - Mise à jour temps réel
  
- `examples/viewer/style.css` - Styles modernes
  - Dark theme
  - Responsive design
  - Animations subtiles
  
- `examples/viewer/README.md` - Documentation du viewer
  - Guide d'utilisation
  - Personnalisation
  - Protocol WebSocket
  - Création de viewers alternatifs

#### Documentation Complète
- `examples/API_REFERENCE.md` - Documentation API JavaScript
  - GameState API
  - Unit API
  - Exemples d'utilisation
  - Bonnes pratiques
  
- `PROJECT_SUMMARY.md` - Résumé complet du projet
  - Vue d'ensemble
  - Architecture
  - Technologies
  - Roadmap
  
- `STRUCTURE.md` - Structure détaillée
  - Arborescence complète
  - Changements effectués
  - Flux de travail
  
- `QUICKSTART.md` - Guide de démarrage rapide
  - Installation
  - Construction
  - Premiers pas
  - Commandes utiles

### Modifié 🔧

- `README.md` - Refonte complète
  - Description headless
  - Nouvelles caractéristiques
  - Structure mise à jour
  - Exemples JavaScript
  
- `BUILD.md` - Mise à jour
  - Section "Next Steps" améliorée
  - Références au viewer
  - Guide viewer personnalisé
  
- `src/lib.rs` - Suppression du module graphics
  - Architecture headless
  - Modules mis à jour

### Supprimé ❌

#### Modules Graphiques (Architecture Headless)
- `src/graphics/` - Module de rendu intégré
  - `src/graphics/mod.rs`
  - `src/graphics/renderer.rs`
  - `src/graphics/ui.rs`
  
- `assets/shaders/` - Shaders graphiques
  
- `examples/basic_bot.rs` - Exemple Rust (remplacé par JavaScript)

**Raison :** Le moteur est maintenant headless. Les joueurs créent leurs propres clients de visualisation.

### Philosophie du Projet

#### Liberté Totale de Visualisation
Les joueurs peuvent créer leur propre interface avec :
- Technologies web (React, Vue, Canvas, Three.js)
- Moteurs de jeu (Unity, Godot)
- Applications desktop (Electron, Tauri)
- Interfaces terminal (ASCII art)
- N'importe quelle technologie supportant WebSocket

#### Open Source et Éducatif
- Enseigner la programmation
- Promouvoir Rust et JavaScript
- Créer une communauté
- Partager les connaissances

### Roadmap

#### Phase 1 : Fondations ✅ (Actuelle)
- [x] Structure du projet
- [x] Documentation complète
- [x] Exemples de bots JavaScript
- [x] Viewer HTML de base
- [x] Architecture headless

#### Phase 2 : Moteur de Base 🚧 (À venir)
- [ ] Simulation du monde
- [ ] Système d'entités
- [ ] Gestion des ressources
- [ ] Système de déplacement

#### Phase 3 : Scripting ⏳
- [ ] Sandbox JavaScript (Boa/Deno)
- [ ] API de scripting complète
- [ ] Système d'événements
- [ ] Limitations de sécurité

#### Phase 4 : Réseau ⏳
- [ ] Serveur WebSocket
- [ ] REST API
- [ ] Authentification
- [ ] Multi-joueurs

#### Phase 5 : Gameplay 📅
- [ ] Système de combat
- [ ] Construction de bâtiments
- [ ] Tech tree
- [ ] Fog of war

#### Phase 6 : Avancé 📅
- [ ] Système de replays
- [ ] Classements
- [ ] Tournois
- [ ] IA de référence

### Technologies Utilisées

#### Backend
- **Rust** 1.70+ - Langage principal
- **Tokio** - Runtime async (prévu)
- **Warp/Actix** - Web framework (à décider)
- **Boa/Deno** - Moteur JavaScript (à décider)
- **Serde** - Sérialisation JSON (prévu)

#### Frontend (Exemples)
- **HTML5/CSS3/JavaScript** - Viewer de base
- **Canvas API** - Rendu 2D
- **WebSocket API** - Communication temps réel

### Fichiers du Projet

#### Documentation
- `README.md` - Documentation principale
- `BUILD.md` - Guide de construction
- `PROJECT_SUMMARY.md` - Résumé complet
- `STRUCTURE.md` - Structure détaillée
- `QUICKSTART.md` - Démarrage rapide
- `CHANGELOG.md` - Ce fichier
- `LICENSE` - Licence MIT

#### Code Source (Rust)
- `src/main.rs` - Point d'entrée
- `src/lib.rs` - Bibliothèque
- `src/game/*` - Moteur de jeu
- `src/api/*` - API de scripting
- `src/network/*` - Serveur réseau
- `src/scripting/*` - Sandbox JavaScript

#### Exemples (JavaScript)
- `examples/basic_bot.js` - Bot simple
- `examples/advanced_bot.js` - Bot avancé
- `examples/template_bot.js` - Template
- `examples/API_REFERENCE.md` - Doc API

#### Viewer (HTML/CSS/JS)
- `examples/viewer/index.html` - UI
- `examples/viewer/viewer.js` - Logique
- `examples/viewer/style.css` - Styles
- `examples/viewer/README.md` - Doc

#### Tests
- `tests/integration_tests.rs` - Tests d'intégration

#### Configuration
- `Cargo.toml` - Configuration Cargo
- `.gitignore` - Fichiers ignorés par Git

### Contribution

Les contributions sont bienvenues dans ces domaines :
- 🎮 Implémentation du moteur de simulation
- �� Sandbox JavaScript sécurisé
- 🌐 Serveur WebSocket/REST
- 📚 Documentation et tutoriels
- 🎨 Viewers alternatifs (React, Unity, etc.)
- 🧪 Tests unitaires et d'intégration

### Licence

MIT License - Voir fichier `LICENSE`

### Contact

- **Repository** : https://github.com/xelfe/GeekCraft
- **Issues** : https://github.com/xelfe/GeekCraft/issues
- **Discussions** : https://github.com/xelfe/GeekCraft/discussions

---

**GeekCraft** - Votre jeu, votre code, votre vision ! 🎮🚀

*Format inspiré de [Keep a Changelog](https://keepachangelog.com/)*

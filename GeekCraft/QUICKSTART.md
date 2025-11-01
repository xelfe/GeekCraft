# 🚀 GeekCraft - Guide de Démarrage Rapide

## Construction du Projet

Pour construire GeekCraft, suivez ces étapes simples :

### Prérequis

1. **Installer Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Vérifier l'installation**
   ```bash
   rustc --version  # Doit être 1.70+
   cargo --version
   ```

### Construction

```bash
# 1. Naviguer vers le projet
cd GeekCraft

# 2. Construire en mode release (optimisé)
cargo build --release

# ⏳ Première compilation : 3-7 minutes
# Les compilations suivantes seront beaucoup plus rapides
```

### Lancement

```bash
# Lancer le serveur
cargo run --release

# Le serveur démarre sur ws://localhost:3030
```

## Tester le Viewer

Une fois le serveur lancé :

```bash
# Dans un nouveau terminal
cd examples/viewer

# Ouvrir le viewer HTML
open index.html  # macOS
xdg-open index.html  # Linux
start index.html  # Windows

# Ou utilisez un serveur HTTP local
python3 -m http.server 8000
# Puis ouvrez http://localhost:8000
```

## Créer Votre Premier Bot

```bash
# 1. Copier le template
cp examples/template_bot.js my_bot.js

# 2. Éditer avec votre stratégie
nano my_bot.js  # ou votre éditeur préféré

# 3. Tester votre bot
# (via l'API du serveur ou l'interface web)
```

## Commandes Utiles

```bash
# Build rapide (debug)
cargo build

# Build optimisé (release)
cargo build --release

# Lancer directement
cargo run

# Lancer en release
cargo run --release

# Tests
cargo test

# Documentation
cargo doc --open

# Nettoyer
cargo clean

# Vérifier sans compiler
cargo check

# Formater le code
cargo fmt

# Linter
cargo clippy
```

## Structure du Projet

```
GeekCraft/
├── src/              # Code source Rust (moteur)
├── examples/         # Bots JS et viewer HTML
│   ├── *.js         # Exemples de bots
│   └── viewer/      # Viewer HTML de base
├── tests/           # Tests
└── assets/          # Ressources (optionnel)
```

## Documentation

- **README.md** - Documentation principale
- **BUILD.md** - Guide de construction détaillé
- **PROJECT_SUMMARY.md** - Résumé du projet
- **STRUCTURE.md** - Structure complète
- **examples/API_REFERENCE.md** - API JavaScript
- **examples/viewer/README.md** - Documentation viewer

## Workflow de Développement

```
1. Éditer le code → 2. Compiler → 3. Tester → 4. Répéter
         ↓                ↓           ↓
    src/**/*.rs     cargo build   cargo test
```

## Troubleshooting

### Erreur de compilation

```bash
# Nettoyer et reconstruire
cargo clean
cargo build --release
```

### Linker non trouvé (Linux)

```bash
sudo apt-get install build-essential
```

### Linker non trouvé (macOS)

```bash
xcode-select --install
```

### Dépendances obsolètes

```bash
cargo update
```

## Next Steps

1. ✅ Construire le serveur (vous êtes ici)
2. 📖 Lire l'API Reference (`examples/API_REFERENCE.md`)
3. 🤖 Créer votre bot (`examples/template_bot.js`)
4. 🎨 Tester le viewer (`examples/viewer/`)
5. 🚀 Créer votre propre viewer (optionnel)

## Support

- **Documentation** : Voir les fichiers `.md`
- **Issues** : https://github.com/xelfe/GeekCraft/issues
- **Discussions** : https://github.com/xelfe/GeekCraft/discussions

## En Résumé

```bash
# Build une fois
cd GeekCraft
cargo build --release

# Lancez
cargo run --release

# Testez
open examples/viewer/index.html

# Codez
cp examples/template_bot.js my_bot.js
# Éditez my_bot.js avec votre stratégie !
```

**C'est tout ! Vous êtes prêt à coder ! 🎮🚀**

---

*Pour plus de détails, consultez `BUILD.md`*

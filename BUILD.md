# Guide de Construction - GeekCraft

## Prérequis

Avant de construire GeekCraft, assurez-vous d'avoir :

- **Rust 1.70+** : [Installation](https://rustup.rs)
- **Cargo** (inclus avec Rust)
- **Git** (pour cloner le dépôt)

### Vérifier l'installation

```bash
rustc --version  # devrait afficher 1.70 ou plus
cargo --version
```

## Construction du Projet

### 1. Cloner le dépôt

```bash
git clone https://github.com/xelfe/GeekCraft.git
cd GeekCraft/GeekCraft
```

### 2. Build en mode Debug (développement)

```bash
cargo build
```

Cette commande :
- Télécharge toutes les dépendances
- Compile le projet en mode debug
- Crée l'exécutable dans `target/debug/geekcraft`

**Temps estimé :** 2-5 minutes (première compilation)

### 3. Build en mode Release (production)

```bash
cargo build --release
```

Cette commande :
- Compile avec optimisations complètes
- Crée l'exécutable dans `target/release/geekcraft`
- **Recommandé pour le déploiement**

**Temps estimé :** 3-7 minutes

### 4. Vérifier la construction

```bash
# En mode debug
./target/debug/geekcraft --version

# En mode release
./target/release/geekcraft --version
```

## Exécution

### Mode Debug

```bash
cargo run
```

ou directement :

```bash
./target/debug/geekcraft
```

### Mode Release

```bash
cargo run --release
```

ou directement :

```bash
./target/release/geekcraft
```

### Avec logs détaillés

```bash
RUST_LOG=debug cargo run
```

Niveaux de log disponibles : `error`, `warn`, `info`, `debug`, `trace`

## Tests

### Exécuter tous les tests

```bash
cargo test
```

### Tests avec sortie détaillée

```bash
cargo test -- --nocapture
```

### Tests d'un module spécifique

```bash
cargo test game::
cargo test network::
```

### Tests d'intégration

```bash
cargo test --test integration_tests
```

## Documentation

### Générer la documentation

```bash
cargo doc --no-deps
```

### Ouvrir la documentation dans le navigateur

```bash
cargo doc --open
```

## Nettoyage

### Nettoyer les builds

```bash
cargo clean
```

Cela supprime le dossier `target/` (utile si vous rencontrez des problèmes de compilation).

## Problèmes Courants

### Erreur : "linker not found"

**Linux :**
```bash
sudo apt-get install build-essential
```

**macOS :**
```bash
xcode-select --install
```

### Erreur de dépendances

```bash
cargo update
cargo clean
cargo build
```

### Manque de mémoire lors de la compilation

```bash
# Limiter le parallélisme
cargo build -j 2
```

## Scripts de Build

### Script de build rapide (Linux/macOS)

Créez un fichier `build.sh` :

```bash
#!/bin/bash
set -e

echo "🔨 Construction de GeekCraft..."
cargo build --release

echo "✓ Build terminé !"
echo "📍 Exécutable : ./target/release/geekcraft"
```

Rendez-le exécutable :
```bash
chmod +x build.sh
./build.sh
```

### Script de build et test (Linux/macOS)

Créez un fichier `build-and-test.sh` :

```bash
#!/bin/bash
set -e

echo "🔨 Construction..."
cargo build --release

echo "🧪 Tests..."
cargo test

echo "📚 Documentation..."
cargo doc --no-deps

echo "✓ Tout est prêt !"
```

## Build pour la Production

### Build optimisé

```bash
cargo build --release --locked
```

### Taille de l'exécutable

Réduire la taille de l'exécutable (optionnel) :

Ajoutez dans `Cargo.toml` :

```toml
[profile.release]
opt-level = "z"     # Optimiser pour la taille
lto = true          # Link Time Optimization
codegen-units = 1   # Meilleure optimisation
strip = true        # Supprimer les symboles de debug
```

Puis :

```bash
cargo build --release
```

## Build Cross-Platform

### Pour Windows (depuis Linux)

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

### Pour Linux (depuis macOS)

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

## Commandes Utiles

```bash
# Vérifier le code sans compiler
cargo check

# Formater le code
cargo fmt

# Linter (clippy)
cargo clippy

# Mise à jour des dépendances
cargo update

# Voir l'arbre des dépendances
cargo tree

# Statistiques du projet
cargo count
```

## Variables d'Environnement

```bash
# Port du serveur
export GEEKCRAFT_PORT=3030

# Niveau de log
export RUST_LOG=info

# Mode de développement
export GEEKCRAFT_DEV_MODE=true
```

## Next Steps

Après avoir construit GeekCraft :

1. **Lancer le serveur**
   ```bash
   cargo run --release
   ```

2. **Ouvrir le viewer exemple**
   ```bash
   cd examples/viewer
   open index.html  # ou double-cliquez sur le fichier
   ```

3. **Créer votre premier bot**
   ```bash
   cp examples/template_bot.js my_bot.js
   # Éditez my_bot.js avec votre stratégie
   ```

4. **Explorer la documentation**
   - API Reference : `examples/API_REFERENCE.md`
   - Exemples de bots : `examples/basic_bot.js` et `examples/advanced_bot.js`
   - Viewer documentation : `examples/viewer/README.md`

5. **Créer votre propre viewer** (optionnel)
   - Le moteur est headless - vous êtes libre de créer votre propre interface
   - Utilisez le viewer HTML comme référence
   - Technologies suggérées : React, Unity, Godot, terminal, etc.

## Support

En cas de problème :
- Vérifiez les issues GitHub
- Consultez la documentation Rust
- Rejoignez notre Discord

**Bon build ! 🚀**

# GeekCraft Viewer - Exemple de Client de Visualisation

Ce viewer HTML est un exemple basique de client de visualisation pour GeekCraft. Il démontre comment se connecter au serveur de jeu et afficher l'état du jeu en temps réel.

## Caractéristiques

- ✅ Connexion WebSocket au serveur
- ✅ Affichage en temps réel du monde de jeu
- ✅ Canvas HTML5 pour le rendu
- ✅ Interface utilisateur responsive
- ✅ Console de logs intégrée
- ✅ Contrôles de caméra (zoom, défilement)
- ✅ Sélection d'unités
- ✅ Informations détaillées sur les joueurs et unités

## Utilisation

### 1. Lancer le serveur GeekCraft

```bash
cd GeekCraft
cargo run --release
```

Le serveur démarre par défaut sur `ws://localhost:3030/ws`

### 2. Ouvrir le viewer

Ouvrez simplement `index.html` dans votre navigateur web :

```bash
cd examples/viewer
# Méthode 1 : Double-cliquez sur index.html
# Méthode 2 : Utilisez un serveur HTTP local
python3 -m http.server 8000
# Puis ouvrez http://localhost:8000
```

### 3. Se connecter

1. Vérifiez que l'URL du serveur est correcte (par défaut : `ws://localhost:3030/ws`)
2. Cliquez sur le bouton "Connecter"
3. Le viewer affichera l'état du jeu en temps réel

## Contrôles

### Caméra
- **Zoom +/- :** Boutons dans la barre de contrôle
- **Molette de la souris :** Zoomer/dézoomer
- **Centrer :** Réinitialiser la vue

### Sélection
- **Clic sur une unité :** Voir ses détails dans le panneau de droite
- **Clic ailleurs :** Désélectionner

### Console
- **Effacer :** Vider les logs de la console

## Structure du Code

### index.html
Interface utilisateur principale avec :
- Header (titre, statut de connexion)
- Sidebar gauche (configuration, infos du jeu, joueurs)
- Zone centrale (canvas de jeu, contrôles)
- Panneau droit (détails de sélection)
- Console en bas (logs)

### viewer.js
Logique principale du viewer :
- **GeekCraftViewer** : Classe principale
- **WebSocket** : Communication avec le serveur
- **Rendu Canvas** : Affichage du jeu
- **Gestion des événements** : Interactions utilisateur

### style.css
Styles modernes avec :
- Dark theme
- Layout responsive
- Animations subtiles
- Scrollbars personnalisées

## Protocol WebSocket

Le viewer communique avec le serveur via WebSocket en JSON :

### Messages entrants (serveur → client)

```javascript
// État complet du jeu
{
    "type": "gameState",
    "data": {
        "tick": 123,
        "players": [...],
        "units": [...],
        "buildings": [...],
        "resources": [...]
    }
}

// Mise à jour incrémentale
{
    "type": "gameUpdate",
    "data": {
        "tick": 124,
        "units": [...]
    }
}

// Événement de jeu
{
    "type": "event",
    "data": {
        "eventType": "unitCreated",
        "unitId": 42
    }
}
```

### Messages sortants (client → serveur)

```javascript
// Demander l'état du jeu
{
    "type": "getGameState"
}

// Changer la vitesse du jeu
{
    "type": "setSpeed",
    "speed": 2.0
}
```

## Personnalisation

### Modifier l'apparence

Éditez `style.css` pour changer les couleurs, tailles, etc.

```css
:root {
    --primary-color: #2196F3;  /* Couleur principale */
    --bg-dark: #1a1a1a;        /* Fond sombre */
    /* ... */
}
```

### Ajouter des fonctionnalités

Le viewer est conçu pour être facilement extensible :

1. **Nouveau type d'entité** : Ajoutez une méthode `render[Type]()` dans `viewer.js`
2. **Nouvelle commande** : Ajoutez un bouton et utilisez `sendCommand()`
3. **Nouveau panneau** : Modifiez `index.html` et `style.css`

### Exemple : Ajouter un bouton pause

```html
<!-- Dans index.html -->
<button id="pause-btn" class="btn btn-sm">⏸️ Pause</button>
```

```javascript
// Dans viewer.js, méthode setupEventListeners()
document.getElementById('pause-btn').addEventListener('click', () => {
    this.sendCommand({ type: 'pause' });
});
```

## Créer votre propre viewer

Ce viewer HTML est juste un exemple ! Vous pouvez créer votre propre client avec :

### Technologies Web
- **React/Vue/Angular** : Applications web modernes
- **Three.js** : Rendu 3D dans le navigateur
- **Phaser** : Framework de jeu 2D
- **PixiJS** : Rendu 2D haute performance

### Technologies Desktop
- **Electron** : Application desktop web
- **Unity** : Moteur de jeu 3D
- **Godot** : Moteur de jeu open-source
- **Tauri** : Alternative légère à Electron

### Technologies Terminal
- **Blessed/Ink** (Node.js) : Interface terminal
- **Rich** (Python) : Terminal avec couleurs
- **Cursive** (Rust) : TUI en Rust

### Exemple minimaliste

```javascript
// viewer_minimal.js
const ws = new WebSocket('ws://localhost:3030/ws');

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('Game state:', data);
    
    // Affichez comme vous voulez !
    // Terminal, canvas, 3D, etc.
};

ws.onopen = () => {
    ws.send(JSON.stringify({ type: 'getGameState' }));
};
```

## Troubleshooting

### Le viewer ne se connecte pas
- Vérifiez que le serveur GeekCraft est lancé
- Vérifiez l'URL WebSocket dans l'interface
- Regardez la console du navigateur (F12) pour les erreurs

### Rien ne s'affiche sur le canvas
- Vérifiez que vous recevez des données (console en bas)
- Vérifiez que le serveur envoie des données de jeu
- Ajustez le zoom si les éléments sont hors de vue

### Performance faible
- Réduisez la fréquence de mise à jour du serveur
- Optimisez le rendu (ne redessinez que ce qui change)
- Utilisez un canvas plus petit

## Ressources

- [WebSocket API MDN](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
- [Canvas API MDN](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API)
- [GeekCraft API Reference](../API_REFERENCE.md)

## Licence

Ce viewer d'exemple est fourni sous licence MIT, comme le reste du projet GeekCraft.

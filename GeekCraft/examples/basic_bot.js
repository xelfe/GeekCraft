/**
 * Basic Bot Example for GeekCraft
 * 
 * Ce bot démontre l'API de base disponible pour les joueurs.
 * Les joueurs peuvent écrire leurs propres bots en JavaScript pour contrôler
 * des unités dans le jeu.
 * 
 * @author GeekCraft Team
 * @version 1.0.0
 */

class BasicBot {
    constructor() {
        this.name = "BasicBot";
        this.version = "1.0.0";
    }

    /**
     * Appelé quand le bot est initialisé
     */
    onInit() {
        console.log(`${this.name} v${this.version} initialized!`);
    }

    /**
     * Appelé à chaque tick du jeu (boucle principale)
     * @param {GameState} gameState - L'état actuel du jeu
     */
    onTick(gameState) {
        // Récupérer toutes vos unités
        const myUnits = gameState.getMyUnits();
        
        // Stratégie simple : déplacer toutes les unités vers les ressources
        for (const unit of myUnits) {
            if (unit.isIdle()) {
                const nearestResource = gameState.findNearestResource(unit.position);
                
                if (nearestResource) {
                    unit.moveTo(nearestResource.position);
                    console.log(`Unit ${unit.id} se déplace vers une ressource à ${nearestResource.position}`);
                } else {
                    console.log(`Aucune ressource trouvée pour l'unité ${unit.id}`);
                }
            }
        }
    }

    /**
     * Appelé quand une unité est créée
     * @param {Unit} unit - L'unité nouvellement créée
     */
    onUnitCreated(unit) {
        console.log(`Nouvelle unité créée : ${unit.id} de type ${unit.type}`);
    }

    /**
     * Appelé quand une unité est détruite
     * @param {Unit} unit - L'unité détruite
     */
    onUnitDestroyed(unit) {
        console.log(`Unité détruite : ${unit.id}`);
    }

    /**
     * Appelé quand des ressources sont collectées
     * @param {Unit} unit - L'unité qui collecte
     * @param {Resource} resource - La ressource collectée
     * @param {number} amount - La quantité collectée
     */
    onResourceCollected(unit, resource, amount) {
        console.log(`Unité ${unit.id} a collecté ${amount} ${resource.type}`);
    }
}

// Exporter le bot
module.exports = BasicBot;

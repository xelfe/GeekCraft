/**
 * Bot Template for GeekCraft
 * 
 * Utilisez ce template comme point de départ pour votre propre bot.
 * Remplissez les méthodes avec votre logique personnalisée.
 * 
 * @author Votre Nom
 * @version 1.0.0
 */

class TemplateBot {
    constructor() {
        this.name = "MonBot";
        this.version = "1.0.0";
        // Ajoutez vos propriétés personnalisées ici
    }

    /**
     * Appelé une fois au démarrage du jeu
     * Utilisez cette méthode pour initialiser vos structures de données
     */
    onInit() {
        // Initialisez votre bot ici
        console.log(`${this.name} v${this.version} démarré!`);
    }

    /**
     * Boucle principale du jeu - appelée à chaque tick
     * @param {GameState} gameState - L'état actuel du jeu
     * 
     * C'est ici que vous implémentez votre stratégie principale
     */
    onTick(gameState) {
        // Implémentez la stratégie de votre bot ici
        
        // Exemple : récupérer vos unités
        // const myUnits = gameState.getMyUnits();
        
        // Exemple : récupérer vos ressources
        // const myResources = gameState.getMyResources();
        
        // Exemple : trouver les ennemis
        // const enemies = gameState.findEnemyUnits();
    }

    /**
     * Appelé quand une nouvelle unité est créée
     * @param {Unit} unit - La nouvelle unité
     */
    onUnitCreated(unit) {
        // Gérez la création d'une nouvelle unité
    }

    /**
     * Appelé quand une unité est détruite
     * @param {Unit} unit - L'unité détruite
     */
    onUnitDestroyed(unit) {
        // Gérez la destruction d'une unité
    }

    /**
     * Appelé quand des ressources sont collectées
     * @param {Unit} unit - L'unité qui collecte
     * @param {Resource} resource - La ressource collectée
     * @param {number} amount - La quantité collectée
     */
    onResourceCollected(unit, resource, amount) {
        // Gérez la collecte de ressources
    }

    /**
     * Appelé quand une structure est construite
     * @param {Structure} structure - La structure construite
     */
    onStructureBuilt(structure) {
        // Gérez la construction d'une structure
    }

    /**
     * Appelé quand votre unité attaque une autre unité
     * @param {Unit} attacker - Votre unité qui attaque
     * @param {Unit} target - La cible de l'attaque
     */
    onUnitAttack(attacker, target) {
        // Gérez les attaques
    }

    /**
     * Appelé quand une de vos unités subit des dégâts
     * @param {Unit} unit - L'unité blessée
     * @param {number} damage - Les dégâts subis
     */
    onUnitDamaged(unit, damage) {
        // Gérez les dégâts reçus
    }
}

// N'oubliez pas d'exporter votre bot !
module.exports = TemplateBot;

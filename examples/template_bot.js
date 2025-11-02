/**
 * Bot Template for GeekCraft
 * 
 * Use this template as a starting point for your own bot.
 * Fill in the methods with your custom logic.
 * 
 * @author Your Name
 * @version 1.0.0
 */

class TemplateBot {
    constructor() {
        this.name = "MyBot";
        this.version = "1.0.0";
        // Add your custom properties here
    }

    /**
     * Called once at game startup
     * Use this method to initialize your data structures
     */
    onInit() {
        // Initialize your bot here
        console.log(`${this.name} v${this.version} started!`);
    }

    /**
     * Main game loop - called every tick
     * @param {GameState} gameState - The current game state
     * 
     * This is where you implement your main strategy
     */
    onTick(gameState) {
        // Implement your bot strategy here
        
        // Example: get your units
        // const myUnits = gameState.getMyUnits();
        
        // Example: get your resources
        // const myResources = gameState.getMyResources();
        
        // Example: find enemies
        // const enemies = gameState.findEnemyUnits();
    }

    /**
     * Called when a new unit is created
     * @param {Unit} unit - The new unit
     */
    onUnitCreated(unit) {
        // Handle the creation of a new unit
    }

    /**
     * Called when a unit is destroyed
     * @param {Unit} unit - The destroyed unit
     */
    onUnitDestroyed(unit) {
        // Handle the destruction of a unit
    }

    /**
     * Called when resources are collected
     * @param {Unit} unit - The unit collecting
     * @param {Resource} resource - The resource collected
     * @param {number} amount - The amount collected
     */
    onResourceCollected(unit, resource, amount) {
        // Handle resource collection
    }

    /**
     * Called when a structure is built
     * @param {Structure} structure - The built structure
     */
    onStructureBuilt(structure) {
        // Handle structure construction
    }

    /**
     * Called when your unit attacks another unit
     * @param {Unit} attacker - Your attacking unit
     * @param {Unit} target - The attack target
     */
    onUnitAttack(attacker, target) {
        // Handle attacks
    }

    /**
     * Called when one of your units takes damage
     * @param {Unit} unit - The damaged unit
     * @param {number} damage - The damage taken
     */
    onUnitDamaged(unit, damage) {
        // Handle damage received
    }
}

// Don't forget to export your bot!
module.exports = TemplateBot;

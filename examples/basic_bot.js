/**
 * Basic Bot Example for GeekCraft
 * 
 * This bot demonstrates the basic API available to players.
 * Players can write their own bots in JavaScript to control
 * units in the game.
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
     * Called when the bot is initialized
     */
    onInit() {
        console.log(`${this.name} v${this.version} initialized!`);
    }

    /**
     * Called every game tick (main loop)
     * @param {GameState} gameState - The current game state
     */
    onTick(gameState) {
        // Get all your units
        const myUnits = gameState.getMyUnits();
        
        // Simple strategy: move all units toward resources
        for (const unit of myUnits) {
            if (unit.isIdle()) {
                const nearestResource = gameState.findNearestResource(unit.position);
                
                if (nearestResource) {
                    unit.moveTo(nearestResource.position);
                    console.log(`Unit ${unit.id} moving to resource at ${nearestResource.position}`);
                } else {
                    console.log(`No resource found for unit ${unit.id}`);
                }
            }
        }
    }

    /**
     * Called when a unit is created
     * @param {Unit} unit - The newly created unit
     */
    onUnitCreated(unit) {
        console.log(`New unit created: ${unit.id} of type ${unit.type}`);
    }

    /**
     * Called when a unit is destroyed
     * @param {Unit} unit - The destroyed unit
     */
    onUnitDestroyed(unit) {
        console.log(`Unit destroyed: ${unit.id}`);
    }

    /**
     * Called when resources are collected
     * @param {Unit} unit - The unit collecting
     * @param {Resource} resource - The resource collected
     * @param {number} amount - The amount collected
     */
    onResourceCollected(unit, resource, amount) {
        console.log(`Unit ${unit.id} collected ${amount} ${resource.type}`);
    }
}

// Export the bot
module.exports = BasicBot;

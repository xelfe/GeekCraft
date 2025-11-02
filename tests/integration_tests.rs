// Integration tests for GeekCraft.
// Note: Integration tests are compiled as a separate crate,
// so we must use the crate name as the path root.

use geekcraft::game::world::World;

#[test]
fn test_game_world_initialization() {
    // Create a new world and verify initial state
    let world = World::new();
    assert_eq!(world.get_tick(), 0, "Newly created world should start at tick 0");
}
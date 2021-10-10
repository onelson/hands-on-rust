# Rusty Roguelike

A dungeon crawler with procedurally generated levels, monsters of increasing
difficulty and turn-based movement.

## Story

The hero's hometown is suffering from a plague of monsters. Welling up from the
deep, they seem to be unstoppable. Legends tell of the Amulet of Yala (Yet
Another Lost Amulet) that can be used to stem the tide. After a long night at
the tavern, the hero promises to save the day and sets forth into the dungeon.

## Game Loops

1. Enter the dungeon level.
2. Explore, revealing the map (fog of war style).
3. Encounter enemies who the player fights or flees from.
4. Find power-ups and use them to strengthen the player.
5. Locate the exit to the level.
6. Repeat!

## Minimum Viable Product

- [ ] Create a basic dungeon map.
- [ ] Place the player and let them walk around.
- [ ] Spawn monsters, draw them, and let the player kill them by walking into them.
- [ ] Add health and a combat system that takes it into account.
- [ ] Add healing potions.
- [ ] Display a "game over" screen when the player dies.
- [ ] Add the Amulet of Yala to the level and let the player win by reaching it.

## Stretch Goals

- [ ] Add fields-of-view.
- [ ] Add more interesting dungeon designs.
- [ ] Add dungeon themes.
- [ ] Add multiple layers to the dungeon (with the Amulet on the last one).
- [ ] Add new weapons.
- [ ] Move to a data-driven design for spawning enemies.
- [ ] Consider some visual effects to make combat more visceral.
- [ ] Consider keeping score.

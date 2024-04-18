# Section 3: Project: Game Prototype with Rusty Engine

## Lesson 26: Project Overview

This section 3 is divided into 2 parts. Both parts will make use of [Rusty Engine](https://github.com/CleanCut/rusty_engine), an open source game engine around [Bevy](https://bevyengine.org/).
- Part 1 will be the tutorial on how to use Rusty Engine.
- Part 2 will run through the game scenarios.

## Lesson 27: Configuration

- First let make a new project with Cargo
```shell
cargo new project-game-prototype-with-rusty-engine
```
- then add `rusty_engine` dependencies.
- at this time, we can run `cargo run` and it will install all dependencies.
- next to download assets and the recommended way is to execute below command inside your project directory
```shell
curl -L https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz | tar -zxv --strip-components=1 rusty_engine-main/assets
```

## Lesson 28: Engine Initialization

- load `rusty_engine::prelude` into our program
- create mutable game engine
- then we can run our game via the `run(game_state)` method. Ex: `game.run(())` for now we're just passing empty tuple for game_state argument.
- now we can run our game (`cargo run`)

## Lesson 29: Game State

- we need it somewhere to store data in our game that is not part of the engine 
- and we need access to it more than a single frame,
- that "somewhere" is your game state struct.
- in this lesson we create `GameState` struct and implement `Default`
- then pass default into game.run().

## Lesson 30: Game Logic Function

- a game is divided into frames
- in most hardware, we usually get about 60 frame-per-second (fps)
- Rusty Engine tries to run game logic function once each frame.
- in this lesson we create `game_logic(engine: &mut Engine, game_state: &mut GameState)`
- 1st param is always `&mut Engine`
- 2nd param is always `&mut GameState`
- then in the main function before `game.run()` we need to add game logic by calling `game.add_logic()` with param as our game logic function
- we can have more than 1 game logic functions if needed and add them one-by-one in order we want to
 
## Lesson 31: Sprites

- Sprites are 2D images with associated data like transform (translation, rotation, scale), collider and metadata.
- They are the building blocks for game graphics.
- Sprites are created and manipulated through the `Engine` struct.
- Use `add_sprite` method with a label and image path or `SpritePreset` enum for predefined sprites.
- The method returns a mutable reference to the created sprite.
- **Translation:** Uses a `Vec2` to define the sprite's position (x, y) on screen.
  - (0.0, 0.0) is the center of the screen.
  - Positive X goes right, positive Y goes up.
  - Coordinates are in logical pixels.
- **Rotation:** Specified in radians. 0 degrees faces right, pi radians faces up, 2 pi radians is back to facing right.
  - use `UP`, `DOWN`, `LEFT`, `RIGHT` constants for convenience.
- **Scale:** default to 1.0 (100%). Values less than 1 shrink, greater than 1 enlarge the sprite.
- Sprite Layers: define the order in which sprites are spawn. Higher layer is drawn on top of the lower layers.
- Default layer is 0.0.
- Use a value up to 999.0 to ensure a sprite is drawn on top of most others.
 
## Lesson 32: Colliders

**Collisions in Rusty Engine:**

- Collisions are detected between sprites with colliders enabled.
- By default, collisions are disabled.
- A `collision` field on the sprite struct needs to set to `true` for collision detection.
- Collision events are triggered when sprites with enabled colliders begin or end overlapping.
- A `collision_events` vector stores these events.

**Collision Events:**

- Collision events represented by `CollissionEvent` struct.
- contain a `state` field (`CollisionState::Begin` or `CollisionState::End`) indicate collision start or end.
- contain a `pair` field, a tuple of string representing the label of colliding sprites.
- Order of label in tuple is non-deterministic.

**Handling Collisions:**

- we can loop through `collision_events` to process them.
- access the `state` and `pair` fields to determine collision detail.

**Colliders:**

- Convex polygons used for collision detection.
- Visualized with white lines when `engine.show_colliders` is set to `true`.
- Stored in `.collider` file next to the corresponding sprite image file.
- Preset sprites come with predefined colliders.
- Custom collider required the `Collider` example program for creation.

**Creating Colliders with Collider Example:**

- Allows creating colliders for custom sprite images.
- Click points in a clockwise direction to define the collider polygon.
- Must be convex (no inward corners).
- Hold `Shift` key while clicking to adjust the last point
- It will write the collider to a file with extension `.collider`.
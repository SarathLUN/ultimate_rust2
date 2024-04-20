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

## Lesson 33: Keyboard Input

**Keyboard Input in Rusty Engine:**

- provide access to keyboard state through the `engin.keyboard_state` field.
- focus on keyboard state for interactive action like character movement.
- keyboard events (every keystroke) are covered in the written tutorial.

**Keyboard State:**

- represents a snapshot of currently pressed/unpressed keys at the start of each frame.
- use the `pressed` method with a `KeyCode` enum value to check a specific key's state.
- `KeyCode` comes from the Bevy crate and defines all possible key codes.

**Moving a Sprite with Keyboard:**

- use the pressed state of relevant keys (e.g., `Up`, `Down`, `Left`, `Right`) to determine movement direction.
- define movement speed as a constant (e.g., `MOVEMENT_SPEED`).
- update the sprite's position based on pressed keys and movement speed.

**Handling Multiple Keys:**

- use the `press_any` method with and array of `KeyCode` values to check if any of those keys are pressed.

## Lesson 34: Mouse Input

**Mouse Input in Rusty Engine:**

- similar to keyboard input, there's mouse state and mouse events.
- focuses on mouse state for common interactions like clicking.
- mouse events (individual clicks) are covered in the written tutorial.

**Mouse State:**

- provides information about the current mouse state.
- use the `left_pressed` method of `mouse_state` to check if the left mouse button is clicked.
- the `location` method of `mouse_state` returns an `Option<Vec2>`, indicating the mouse position within the window (if available).

**Spawning Sprites on Mouse Click:**

- check if the left mouse button is pressed (engine.mouse_state.left_pressed).
- if a mouse location exists (mouse_state.location), use it to determine the spawn location for the new sprite.
- generate unique labels for each spawned sprite (e.g., using a counter).
- add the new sprite to the engine using `engine.add_sprite` with label, image path, and location.

## Lesson 35: Text

**Text in Rusty Engine:**

- similar to sprites in terms of position and transformation (translation, rotation, scale, layer).
- use a world coordinate system for placement. 
- defined by a string value, font, and font size.

**Creating Text Elements:**

- use the `add_text` method of the engine or game struct.
- provide a unique label for identification.
- specify the text string to be displayed.
- set the desired position using the `translation` field (x, y Coordinates).

**Updating Text Content:**

- get a mutable reference to the text elements using `get_mut` on the `texts` hash map.
- modify the text content using the reference.

## Lesson 36: Audio

**Audio in Rusty Engine:**

- basic audio system supporting looping music and concurrent sound effects.
- support Ogg, MP3, FLAC, WAV audio formats.
- access through the engine's audio manager.

**Music Playback:**

- use the `play_music` method of the audio manager.
- provide a `MusicPreset` enum variant or a path to a music file within the assets directory.
- set the desired volume (0.0 to 1.0).

**Sound Effects:**

- use the `play_sfx` method of the audio manager.
- provide a `SfxPreset` enum variant or a path to a sound effect file within the assets directory.
- set the desired volume (0.0 to 1.0).
- played in a "fire-and-forget" manner, terminating after playback.

## Lesson 37: Timer

**Timer in Rusty Engine:**

- leverages Bevy's `Timer` struct.
- Timers are inexpensive to create and manage.

**Creating Timer:**

- use the `from_seconds` method to create a timer.
- specify the duration (in seconds) and whether it's repeating or not.
- a repeating timer restarts counting down from the initial duration when it reaches zero.

**Updating Timers:**

- call the `tick` method with the elapsed time (`engine.delta`).
- this advances the timer's internal counter.

**Checking Timer Completion:**

- the `tick` method returns an immutable reference to the timer.
- access the `finished` field to check if the timer has recently completed its countdown.

## Lesson 38: Engine & Game Structs

**Engine Struct:**

- `should_exit`: is boolean field to trigger clean exist at the end of the frame.
- use `should_exit` by setting it to `true` to initiate a graceful exit from the game loop.
- `window_dimentions`: `Vec2` representing the window's width and height in logical pixels.
- update text or sprite positions based on window dimensions for dynamic resizing.
- `time_since_startup_f64`: 64-bit floating-point value tracking time elapsed since game start.
- useful for creating periodic animations (e.g., oscillation effects).

**Game Struct:**

- provides access to all `Engine` struct fields.
- allows setting window properties through the `WindowDescriptor` struct.
  - Common properties include `title`, `width`, `height`.
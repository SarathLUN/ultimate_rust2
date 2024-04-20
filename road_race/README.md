# Section 3: Project: Game Prototype with Rusty Engine

## Lesson 39: Game Walkthrough - Common Setup

1. **Add Rusty Engine as a Dependency**: add `rusty_engine` as a dependency in file `Cargo.toml` with the appropriate version (consult documentation for latest version).
2. **Download the Asset Pack**: Follow the instructions on the Asset Pack tutorial page (downloading via curl command is mentioned).
3. **Copy Skeleton Logic**: Copy the provided skeleton logic from the documentation and paste it into your main.rs file.
4. **Run Initial Compilation**: Use `cargo run --release` to compile the project. This step catches any errors from missing dependencies or incorrect setup.
5. **Verify Successful Setup**: A blank gray Rusty Engine window indicates a successful setup and readiness for game development.

## Lesson 40: Game Walkthrough - Road Race

**Setting Up the Game:**

1. Player Health:

   - Add health_amount (u8) and lost (bool) fields to GameState.
   - Provide initial values (e.g., health: 5, lost: false) when running the game.

2. Player Sprite:

   - Create a blue racing car sprite (player1) using add_sprite.
   - Set position (translation.x to -500), layer (10), and collision (true).

3. Background Music:

   - Play the "whimsical popsicle" music preset at 20% volume.

**Game Loop Logic:**

1. Player Movement:

   - Capture keyboard input for up/down movement.
   - Update direction (f32) based on key presses.
   - Get a mutable reference to the player sprite.
   - Adjust translation.y of the sprite based on direction and PLAYER_SPEED constant.
   - Rotate the car slightly based on direction.
   - Implement bounds checking to prevent the player from going off-screen (set health to zero if violated).

2. Road Lines:

   - Create ten road line sprites using add_sprite with SpritePreset::RacingBarrierWhite.
   - Set scale (0.1) and position them horizontally across the screen.
   - In the game loop, iterate through all sprites and move those with "road line" labels to the left using ROAD_SPEED constant and engine.delta_f32.
   - Reset the position of a road line sprite when it goes off-screen (to the left) to simulate continuity.

3. Obstacles:

   - Add rand dependency to generate random numbers.
   - Create a vector of obstacle presets (e.g., RacingBarrelBlue, RacingBarrelRed, RacingConeStraight).
   - In a loop, create obstacle sprites using add_sprite with unique labels and set their layer, collision, and random starting positions (x and y).
   - Move obstacles to the left in the game loop similar to road lines. Reset positions when they go off-screen (to the left).

4. Collisions:

   - Create health text to display remaining health.
   - Iterate through collision events in the game loop.
   - If a collision involves the player (collision includes "player1" label and isn't an ending event), decrement health and update health text.
   - Play a sound effect ("Impact3") when the player gets hurt (if health is still above zero).

5. Lose Condition:
   - Check if game_state.health_amount is zero at the end of the game loop.
   - If health is zero (lost), set game_state.lost to true.
   - Create a large "Game Over" text and play a losing sound effect ("SfxPreset::Jingle3") when the game is lost.
   - Stop the background music.
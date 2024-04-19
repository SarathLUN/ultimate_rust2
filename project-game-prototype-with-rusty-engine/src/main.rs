use rand::prelude::*;
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    high_score: u32,
    score: u32,
    ferris_index: i32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            ferris_index: 0,
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

fn main() {
    let mut game = Game::new();

    // background music audio
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);

    // get your game stuff ready here
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = 0.0; // = player.rotation = RIGHT;
                           // player.rotation = std::f32::consts::FRAC_PI_2; // = player.rotation = UP;
    player.scale = 1.0; // this is the default size 100%
                        // player.layer = 0.1; // 0.0 is the default layer, and the highest layer is 999.0
    player.collision = true;

    // create a temporary sprite to demonstrate with layer
    let car1 = game.add_sprite("car1", "sprite/cuddlyferris.png");
    car1.translation = Vec2::new(300.0, 0.0);
    // car1.rotation = LEFT;
    // car1.layer = 0.0;
    car1.scale = 0.3;
    car1.collision = true;

    // add text for our game
    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);

    // add game logic here
    game.add_logic(game_logic);

    // game start here
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // engine.show_colliders = true;
    for event in engine.collision_events.drain(..) {
        // println!("{:#?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            // play sound effect on hit
            engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.3);

            // remove the sprite the player collided with
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            game_state.score += 1;
            // println!("Current score: {}", game_state.score);
            // instead of print score in console, we display on user's screen
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", game_state.score);
            // handle high score
            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High Score: {}", game_state.high_score);
            }
        }
    }
    // handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 500.0;
    if player.translation.y > -325.0 && player.translation.y < 325.0 {
        if engine
            .keyboard_state
            .pressed_any(&[KeyCode::Up, KeyCode::W])
        {
            player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
        }
        if engine
            .keyboard_state
            .pressed_any(&[KeyCode::Down, KeyCode::S])
        {
            player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
        }
    } else {
        player.translation.y = 0.0;
    }

    if player.translation.x > -550.0 && player.translation.x < 550.0 {
        if engine
            .keyboard_state
            .pressed_any(&[KeyCode::Right, KeyCode::D])
        {
            player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
        }
        if engine
            .keyboard_state
            .pressed_any(&[KeyCode::Left, KeyCode::A])
        {
            player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
        }
    } else {
        player.translation.x = 0.0;
    }
    // handle mouse input
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            // create a temporary sprite as obstacle
            let label = format!("ferris{}", game_state.ferris_index);
            game_state.ferris_index += 1;
            let ferris = engine.add_sprite(label.clone(), "sprite/cuddlyferris.png");
            ferris.translation = mouse_location;
            // ferris.rotation = LEFT;
            // ferris.layer = 0.0;
            ferris.scale = 0.3;
            ferris.collision = true;
        }
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("ferris{}", game_state.ferris_index);
        game_state.ferris_index += 1;
        let ferris = engine.add_sprite(label.clone(), "sprite/cuddlyferris.png");
        ferris.translation.x = thread_rng().gen_range(-550.0..550.0);
        ferris.translation.y = thread_rng().gen_range(-325.0..325.0);
        ferris.scale = 0.2;
        ferris.collision = true;
    }

    // Reset score
    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();
        // player.translation = Vec2::new(0.0, 0.0);
    }
}

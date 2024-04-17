use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

fn main() {
    let mut game = Game::new();

    // get your game stuff ready here
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    // player.rotation = 0.0; // = player.rotation = RIGHT;
    player.rotation = std::f32::consts::FRAC_PI_2; // = player.rotation = UP;
    player.scale = 1.0; // this is the default size 100%
    player.layer = 0.1; // 0.0 is the default layer, and the highest layer is 999.0

    // create a temporary sprite to demonstrate with layer
    let temporary = game.add_sprite("temporary", SpritePreset::RacingCarRed);
    temporary.translation = Vec2::new(30.0, 0.0);
    temporary.layer = 0.0;
    // add game logic here
    game.add_logic(game_logic);

    // game start here
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game_state.current_score += 1;
    // println!("Current score: {}",game_state.current_score);
}
use log::{debug, warn};
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    // board tilting in some direction
    tilting_vec: Vec2,
    // marble have own move direction
    marble_vec: Vec2,
    marble_center: Vec2,
}

fn direction_from_vector(vec: Vec2) -> f32 {
    vec.y.atan2(vec.x)
}

fn translate_titling_to_marble(tilting_vec: Vec2, marble_center: Vec2) -> Vec2 {
    let translation = marble_center;
    tilting_vec + translation
}

// linear interpolation, of begin and end of vector
fn lerp(old_position: Vec2, new_position: Vec2, delta_time: f32) -> Vec2 {
    old_position + delta_time * (new_position - old_position)
}

const MAX_TITLING: f32 = 500.0;
const MIN_TITLING: f32 = 50.0;

fn main() {
    env_logger::init();
    let mut game = Game::new();
    let game_state = GameState {
        tilting_vec: Vec2::new(0.0, 0.0),
        marble_vec: Vec2::new(0.0, 0.0),
        marble_center: Vec2::new(200.0, 300.0),
    };

    let _ = game.add_sprite("marble", SpritePreset::RollingBallBlue);
    let marble = game.sprites.get_mut("marble").unwrap();
    marble.translation = game_state.marble_vec;

    // hole to show the middle - safe zone for mouse
    let _ = game.add_sprite("hole", SpritePreset::RollingHoleStart);
    let center = game.sprites.get_mut("hole").unwrap();
    center.translation = Vec2::new(0.0, 0.0);

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let marble = engine.sprites.get_mut("marble").unwrap();

    // limit tilting vector max and min
    if let Some(location) = engine.mouse_state.location() {
        if location.length() < MIN_TITLING {
            game_state.tilting_vec = Vec2::new(0.0, 0.0);
        } else if location.x > MAX_TITLING {
            game_state.tilting_vec.x = location.x / location.x.abs() * MAX_TITLING;
        } else if location.y > MAX_TITLING {
            game_state.tilting_vec.y = location.y / location.y.abs() * MAX_TITLING;
        } else {
            game_state.tilting_vec = location;
        }
    }

    // translate titling board vector to new marble center vector
    game_state.marble_vec =
        translate_titling_to_marble(game_state.tilting_vec, game_state.marble_center);

    let rotate_to = direction_from_vector(game_state.tilting_vec);
    marble.rotation = if rotate_to != 0.0 {
        rotate_to
    } else {
        marble.rotation
    };

    game_state.marble_center = lerp(
        game_state.marble_center,
        game_state.marble_vec,
        engine.delta_f32,
    );
    marble.translation = game_state.marble_center;

    if marble.translation.length() > engine.window_dimensions.length() / 2.0 {
        game_state.marble_center = Vec2::new(100.0, 200.0);
    }
}

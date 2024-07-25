use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();
    let game_state = GameState {};

    game.add_logic(game_logic); // Don't forget to add the logic function to the game!
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {}

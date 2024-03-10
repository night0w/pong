mod game_state;
mod game;
mod entity;

use game::Game;

/**
 * Main entrypoint for the game
 */
fn main() -> tetra::Result {
    Game::start("Pong", 640.0, 480.0)
}



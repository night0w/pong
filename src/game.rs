use tetra::ContextBuilder;

use crate::game_state::GameState;

/**
 * Game Executer Struct
 */
pub struct Game {}

/**
 * Game Executer Implementation
 */
impl Game {

    /**
     * Start game method. It begins the main game loop
     */
    pub fn start(title: &str, width: f32, height: f32) -> tetra::Result{
        // Main Game Loop
        ContextBuilder::new(title, width as i32, height as i32)
        .quit_on_escape(true)
        .build()?
        .run(|ctx| {
            GameState::new(ctx, width, height)
        })
    }
}
use tetra::{graphics::{self, Color, Texture}, input, math::Vec2, window, Context, State};
use crate::entity::Entity;

// Game config (to-be moved as dedicated config class?)
const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACCEL: f32 = 0.05;

/**
 * Game state class
 * Handles all the frame updates and implements Tetra State
 */
pub struct GameState {
    pub player1: Entity,
    pub player2: Entity,
    pub ball: Entity,
    width: f32,
    height: f32
}

/**
 * Tetra state implementation
 */
impl State for GameState {

    /**
     * Draw override: Draws all the game entities
     */
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.ball.texture.draw(ctx, self.ball.position);
        Ok(())
    }

    /**
     * Method for the frame update. 
     * It changes entities positions according to game dynamics
     * It will also handles collision and win condition
     */
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        // Move player 1 up when pressing W
        if input::is_key_down(ctx, input::Key::W) {
            self.player1.position.y -= PADDLE_SPEED;
        }

        // Move player 1 udown when pressing S
        if input::is_key_down(ctx, input::Key::S) {
            self.player1.position.y += PADDLE_SPEED;
        }

        // Move player 2 up when pressing Up arrow
        if input::is_key_down(ctx, input::Key::Up) {
            self.player2.position.y -= PADDLE_SPEED;
        }
        
        // Move player 2 down when pressing Down arrow
        if input::is_key_down(ctx, input::Key::Down) {
            self.player2.position.y += PADDLE_SPEED;
        }

        // Move ball according with its velocity
        self.ball.position += self.ball.velocity;

        // Collision handling 
        let p1_bounds = self.player1.bounds();
        let p2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();
        let paddle_hit = if ball_bounds.intersects(&p1_bounds) { 
            Some(&self.player1) 
        } else if ball_bounds.intersects(&p2_bounds) { 
            Some(&self.player2) 
        } else { 
            None
        };

        if let Some(paddle) = paddle_hit {
            // Increase the ball's velocity, then flip it.
            self.ball.velocity.x =
                -(self.ball.velocity.x + (BALL_ACCEL * self.ball.velocity.x.signum()));
        
            // Calculate the offset between the paddle and the ball, as a number between
            // -1.0 and 1.0.
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();
        
            // Apply the spin to the ball.
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        // Invert the direction of the ball when up-down boundaries collide
        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= self.height {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // If the ball goes out of the window on the left Player 2 wins
        if self.ball.position.x < 0.0 {
            window::quit(ctx);
            println!("Player 2 wins!");
        }
        // If the ball goes out of the window on the right Player 1 wins
        if self.ball.position.x > self.width {
            window::quit(ctx);
            println!("Player 1 wins!");
        }
        Ok(())
    }
}

/**
 * Game state implementation
 */
impl GameState {

    /**
     * Instantiate a new GameState
     */
    pub fn new(ctx: &mut Context, width: f32, height: f32) -> tetra::Result<GameState> {
        
        // Initialize player 1 texture and position
        let p1_texture = Texture::new(ctx, "./resources/BlueBar.png")?;
        let p1_position = Vec2::new(16.0, (height - p1_texture.height() as f32) / 2.0);

        // Initialize player 2 texture and position
        let p2_texture = Texture::new(ctx, "./resources/RedBar.png")?;
        let p2_position = Vec2::new(
            width - p2_texture.width() as f32 - 16.0,
            (height - p2_texture.height() as f32) / 2.0
        );

        // Initialize ball texture, position and velocity
        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position = Vec2::new(
            width / 2.0 - ball_texture.width() as f32 / 2.0,
            height / 2.0 - ball_texture.height() as f32 / 2.0
        );
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);

        // Instantiate the game state with the config provided
        Ok(GameState {
            player1: Entity::new(p1_texture, p1_position),
            player2: Entity::new(p2_texture, p2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            width,
            height
        })
    }
}
use tetra::{graphics::{Rectangle, Texture}, math::Vec2};

/**
 * Main entity class represent any entity in the game
 */
pub struct Entity {
    pub texture: Texture,
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
}

/**
 * Entity implementation
 */
impl Entity {

    /**
     * Instantiate a new Entity (Velocity defaulted to 0)
     */
    pub fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    /**
     * Instantiate a new Entity considering a velocity
     */
    pub fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            velocity
        }
    }

    /**
     * Return entity texture width as floating number
     */
    pub fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    /**
     * Return entity texture height as floating number
     */
    pub fn height(&self) -> f32 {
        self.texture.height() as f32
    }
    
    /**
     * Return entity Rectangle (used for bounds collision)
     */
    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(), 
            self.height()
        )
    }

    /**
     * Centre the texture
     */
    pub fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
}
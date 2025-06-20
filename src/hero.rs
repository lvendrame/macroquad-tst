use macroquad::{
    color::{WHITE, YELLOW},
    input::{is_key_down, KeyCode},
    math::{clamp, Vec2},
    prelude::animation::AnimatedSprite,
    texture::{draw_texture_ex, DrawTextureParams},
    window::{screen_height, screen_width}
};

use crate::{
    sprites_config::SpritesConfig, collidable::Collidable, constants::MOVEMENT_SPEED, shaders::StarfieldShader, shape::{Shape, ShapeType}
};

pub struct Hero {
    shape: Shape,
    sprite: AnimatedSprite,
}

impl Default for Hero {
    fn default() -> Self {
        Self::new()
    }
}

impl Collidable for Hero {
    fn shape_type(&self) -> ShapeType {
        self.shape.shape_type()
    }

    fn position(&self) -> Vec2 {
        self.shape.position()
    }

    fn size(&self) -> f32 {
        self.shape.size()
    }
}

impl Hero {

    pub fn new() -> Hero {
        Hero {
            shape: Self::create_shape(),
            sprite: SpritesConfig::get_ship_sprite(),
        }
    }

    fn create_shape() -> Shape {
        Shape {
            shape_type: ShapeType::Circle,
            size: 32.0,
            speed: MOVEMENT_SPEED,
            position: Vec2 {
                x: screen_width() / 2.0,
                y: screen_height() / 2.0,
            },
            color: YELLOW,
            collided: false,
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.shape.speed
    }

    pub fn restart(&mut self) {
        self.shape = Self::create_shape();
    }

    pub fn check_inputs(&mut self, delta_time: f32, shader: &mut StarfieldShader) {
        let speed = self.shape.speed * delta_time;
        let radius = self.shape.size / 2.0;

        self.sprite.set_animation(0);
        if is_key_down(KeyCode::Right) {
            self.shape.position.x += speed;
            shader.inc_by(0.05 * delta_time);
            self.sprite.set_animation(2);
        }
        if is_key_down(KeyCode::Left) {
            self.shape.position.x -= speed;
            shader.dec_by(0.05 * delta_time);
            self.sprite.set_animation(1);
        }
        if is_key_down(KeyCode::Down) {
            self.shape.position.y += speed;
        }
        if is_key_down(KeyCode::Up) {
            self.shape.position.y -= speed;
        }

        self.sprite.update();

        self.shape.position.x = clamp(self.shape.position.x, radius, screen_width() - radius);
        self.shape.position.y = clamp(self.shape.position.y, radius, screen_height() - radius);

    }

    pub fn draw(&self, sprites_config: &SpritesConfig) {
        let ship_frame = self.sprite.frame();
        draw_texture_ex(
            &sprites_config.ship_texture,
            self.shape.position.x - ship_frame.dest_size.x,
            self.shape.position.y - ship_frame.dest_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(ship_frame.dest_size * 2.0),
                source: Some(ship_frame.source_rect),
                ..Default::default()
            },
        );
    }

}
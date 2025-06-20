use macroquad::{color::{RED, WHITE}, math::{vec2}, prelude::animation::AnimatedSprite, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{sprites_config::SpritesConfig, collidable::Collidable, hero::Hero, shape::{Shape, ShapeType}};

pub struct Bullet {
    pub shape: Shape,
    sprite: AnimatedSprite,
}

impl Collidable for Bullet {
    fn shape_type(&self) -> ShapeType {
        self.shape.shape_type()
    }

    fn position(&self) -> macroquad::prelude::Vec2 {
        self.shape.position()
    }

    fn size(&self) -> f32 {
        self.shape.size()
    }
}

impl Bullet {

    pub fn new(hero: &Hero) -> Self {
        let mut shape = Shape {
            shape_type: ShapeType::Square,
            size: 32.0,
            speed: hero.get_speed() * 2.,
            position: hero.position(),
            color: RED,
            collided: false,
        };
        shape.position.y -= 24.; // Adjust bullet position to be above the hero

        Bullet {
            shape,
            sprite: SpritesConfig::get_bullet_sprite(),
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.shape.speed
    }

    pub fn get_collided(&self) -> bool {
        self.shape.collided
    }

    pub fn set_collided(&mut self, collided: bool) {
        self.shape.collided = collided;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.shape.position.y -= self.shape.speed * delta_time;
    }

    pub fn draw(&self, sprites_config: &SpritesConfig) {
        let bullet_frame = self.sprite.frame();

        let size = self.size();

        draw_texture_ex(
            &sprites_config.bullet_texture,
            self.shape.position.x - size / 2.0,
            self.shape.position.y - size / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                source: Some(bullet_frame.source_rect),
                ..Default::default()
            },
        );
    }
}
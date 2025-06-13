use macroquad::{color::{RED, WHITE}, math::vec2, prelude::animation::AnimatedSprite, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{assets_config::AssetsConfig, hero::Hero, point_2d::Point2d, shape::{Shape, ShapeType}};

pub struct Bullet{
    pub shape: Shape,
    sprite: AnimatedSprite,
}
impl Bullet {

    pub fn new(hero: &Hero) -> Self {
        let mut shape = Shape {
            shape_type: ShapeType::Square,
            size: 32.0,
            speed: hero.get_speed() * 2.,
            position: hero.get_position(),
            color: RED,
            collided: false,
        };
        shape.position.y -= 24.; // Adjust bullet position to be above the hero

        Bullet {
            shape,
            sprite: AssetsConfig::get_bullet_sprite(),
        }
    }

    pub fn get_size(&self) -> f32 {
        self.shape.size
    }

    pub fn get_speed(&self) -> f32 {
        self.shape.speed
    }

    pub fn get_position(&self) -> Point2d {
        self.shape.position
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

    pub fn collides_with(&self, other: &Shape) -> bool {
        self.shape.collides_with(other)
    }
    pub fn draw(&self, assets_config: &AssetsConfig) {
        // self.shape.draw();

        let bullet_frame = self.sprite.frame();

        let size = self.get_size();

        draw_texture_ex(
            &assets_config.bullet_texture,
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
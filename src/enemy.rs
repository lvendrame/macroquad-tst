use macroquad::{
    color::WHITE, math::{vec2, Vec2},
    prelude::animation::AnimationFrame,
    rand::{self, ChooseRandom},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
    window::screen_width
};

use crate::{collidable::Collidable, constants::COLORS, shape::{Shape, ShapeType}
};

#[derive(Debug, Default)]
pub struct Enemy {
    pub shape: Shape,
}

impl Collidable for Enemy {
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

impl Enemy {
    pub fn new() -> Self {
        Enemy {
            shape: Self::create_shape(),
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.shape.speed
    }

    pub fn create_shape() -> Shape {
        let size = rand::gen_range(16.0, 64.0);
        let half = size / 2.0;

        Shape {
            shape_type: ShapeType::Square,
            size,
            speed: rand::gen_range(50.0, 150.0),
            position: Vec2 {
                x: rand::gen_range(half, screen_width() - half),
                y: -size,
            },
            color: *COLORS.choose().unwrap(),
            collided: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.shape.position.y += self.shape.speed * delta_time;
    }

    pub fn draw(&self, texture: &Texture2D, frame: &AnimationFrame) {
        draw_texture_ex(
            texture,
            self.shape.position.x - self.shape.size / 2.,
            self.shape.position.y - self.shape.size / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.shape.size, self.shape.size)),
                source: Some(frame.source_rect),
                ..Default::default()
            },
        );
    }
}
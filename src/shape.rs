use macroquad::{color::Color, math::{Vec2}, shapes::{draw_circle, draw_rectangle}};

use crate::{collidable::Collidable};

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub enum ShapeType {
    #[default]
    Square,
    Circle
}

#[derive(Default, Debug)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub size: f32,
    pub speed: f32,
    pub position: Vec2,
    pub color: Color,
    pub collided: bool,
}

impl Collidable for Shape {

    fn shape_type(&self) -> ShapeType {
        self.shape_type
    }

    fn position(&self) -> macroquad::prelude::Vec2 {
        self.position
    }

    fn size(&self) -> f32 {
        self.size
    }
}

impl Shape {
    pub fn draw(&self) {
        if self.shape_type == ShapeType::Circle {
            draw_circle(self.position.x, self.position.y, self.size / 2.0, self.color);
            return;
        }

        draw_rectangle(
            self.position.x - self.size / 2.0,
            self.position.y - self.size / 2.0,
            self.size,
            self.size,
            self.color,
        );
    }
}
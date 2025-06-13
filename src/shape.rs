use macroquad::{color::Color, math::Rect, shapes::{draw_circle, draw_rectangle}};

use crate::point_2d::Point2d;

#[derive(PartialEq)]
pub enum ShapeType {
    Square,
    Circle
}

pub struct Shape {
    pub shape_type: ShapeType,
    pub size: f32,
    pub speed: f32,
    pub position: Point2d,
    pub color: Color,
    pub collided: bool,
}

impl Shape {
    pub fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.position.x - self.size / 2.0,
            y: self.position.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }

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
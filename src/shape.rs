use macroquad::{color::Color, math::Rect, shapes::{draw_circle, draw_rectangle}};

#[derive(PartialEq)]
pub enum ShapeType {
    Square,
    Circle
}

pub struct Shape {
    pub shape_type: ShapeType,
    pub size: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub collided: bool,
}

impl Shape {
    pub fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }

    pub fn draw(&self) {
        if self.shape_type == ShapeType::Circle {
            draw_circle(self.x, self.y, self.size / 2.0, self.color);
            return;
        }

        draw_rectangle(
            self.x - self.size / 2.0,
            self.y - self.size / 2.0,
            self.size,
            self.size,
            self.color,
        );
    }
}
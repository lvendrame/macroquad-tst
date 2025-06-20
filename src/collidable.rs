use macroquad::prelude::Rect;

use crate::shape::ShapeType;

pub trait Collidable {

    fn shape_type(&self) -> ShapeType;
    fn position(&self) -> macroquad::prelude::Vec2;
    fn size(&self) -> f32;

    fn rect(&self) -> Rect {
        let pos = self.position();
        let size = self.size();
        let shape_type = self.shape_type();

        match shape_type {
            ShapeType::Circle => Rect {
                x: pos.x - size / 2.0,
                y: pos.y - size / 2.0,
                w: size,
                h: size,
            },
            _ => Rect {
                x: pos.x,
                y: pos.y,
                w: size,
                h: size,
            },
        }
    }

    fn collides_with<T: Collidable>(&self, other: &T) -> bool {
        self.rect().overlaps(&other.rect())
    }
}
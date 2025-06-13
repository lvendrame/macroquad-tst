use macroquad::{color::YELLOW, input::{is_key_down, KeyCode}, math::clamp, miniquad::native::apple::frameworks::Sel, window::{screen_height, screen_width}};

use crate::{constants::MOVEMENT_SPEED, point_2d::Point2D, shaders::StarfieldShader, shape::{Shape, ShapeType}};

pub struct Hero {
    shape: Shape
}

impl Default for Hero {
    fn default() -> Self {
        Self::new()
    }
}

impl Hero {

    pub fn new() -> Hero {
        Hero {
            shape: Self::create_shape(),
        }
    }

    fn create_shape() -> Shape {
        Shape {
            shape_type: ShapeType::Circle,
            size: 32.0,
            speed: MOVEMENT_SPEED,
            position: Point2D {
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

    pub fn get_position(&self) -> Point2D {
        self.shape.position.clone()
    }

    pub fn restart(&mut self) {
        self.shape = Self::create_shape();
    }

    pub fn collides_with(&self, other: &Shape) -> bool {
        self.shape.collides_with(other)
    }

    pub fn check_inputs(&mut self, delta_time: f32, shader: &mut StarfieldShader) {
        let speed = self.shape.speed * delta_time;
        let radius = self.shape.size / 2.0;

        if is_key_down(KeyCode::Right) {
            self.shape.position.x += speed;
            shader.inc_by(0.05 * delta_time);
        }
        if is_key_down(KeyCode::Left) {
            self.shape.position.x -= speed;
            shader.dec_by(0.05 * delta_time);
        }
        if is_key_down(KeyCode::Down) {
            self.shape.position.y += speed;
        }
        if is_key_down(KeyCode::Up) {
            self.shape.position.y -= speed;
        }

        self.shape.position.x = clamp(self.shape.position.x, radius, screen_width() - radius);
        self.shape.position.y = clamp(self.shape.position.y, radius, screen_height() - radius);

    }

    pub fn draw(&self) {
        self.shape.draw();
    }

}
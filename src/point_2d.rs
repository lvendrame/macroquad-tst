#[derive(Default, Clone, Copy)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

impl Point2d {
    pub fn new(x: f32, y: f32) -> Point2d {
        Self { x, y }
    }
}
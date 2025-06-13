use macroquad::color::*;

pub const MOVEMENT_SPEED: f32 = 200.0;
pub const INITIAL_LIVES: u32 = 5;

pub const COLORS: [Color; 11] = [
    DARKPURPLE, BLUE, GREEN, RED, PURPLE, ORANGE, PINK, BROWN, GRAY,
    DARKBLUE, DARKGREEN,
];

pub const ASSETS_PATH: &str = "assets";
pub const HIGH_SCORE_FILE_PATH: &str = "high_score.dat";
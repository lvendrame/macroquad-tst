pub mod collidable;
pub mod shape;
pub mod enemy;
pub mod enemies;
pub mod bullet;
pub mod hero;
pub mod game_state;
pub mod game;
pub mod constants;
pub mod scores;
pub mod shaders;
pub mod particles;
pub mod sprites_config;
pub mod sound_config;
pub mod menu;

use std::process::exit;

use macroquad::{prelude::*};

use crate::{game::Game, game_state::GameState, scores::Scores};

fn draw_game_over() {
    const FONT_SIZE: u16 = 50;
    let text = "GAME OVER! Press ENTER to restart";
    let text_dimensions = measure_text(text, None, FONT_SIZE, 1.0);
    let x = screen_width() / 2.0 - text_dimensions.width / 2.0;
    let y = screen_height() / 2.0 - text_dimensions.height / 2.0;

    draw_rectangle(
        x,
        y - text_dimensions.height,
        text_dimensions.width,
        text_dimensions.height,
        BLACK);

    draw_text(
        text,
        x,
        y,
        FONT_SIZE.into(),
        RED,
    );
}

fn draw_values(lives: u32, scores: &Scores) {
    const FONT_SIZE: f32 = 25.0;
    const SCREEN_VERTICAL_POSITION: f32 = 35.0;

    let lives_text = format!("Lives: {}", lives);
    let text_dimensions = measure_text(lives_text.as_str(), None, 25, 1.0);
    draw_text(
        lives_text.as_str(),
        30.0,
        SCREEN_VERTICAL_POSITION,
        FONT_SIZE,
        WHITE,
    );

    draw_text(
        format!("Score: {}", scores.score).as_str(),
        text_dimensions.width + 50.0,
        SCREEN_VERTICAL_POSITION,
        FONT_SIZE,
        WHITE,
    );

    let high_score_text = format!("High score: {}", scores.high_score);
    let text_dimensions = measure_text(high_score_text.as_str(), None, 25, 1.0);
    draw_text(
        high_score_text.as_str(),
        screen_width() - text_dimensions.width - 10.0,
        SCREEN_VERTICAL_POSITION,
        FONT_SIZE,
        WHITE,
    );
}

#[macroquad::main("My game")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(BLACK);

        match game.game_state {
            GameState::MainMenu => {
                game.main_menu();
            }
            GameState::Playing => {
                game.playing();
            }
            GameState::Paused => {
                game.paused();
            }
            GameState::GameOver => {
                draw_game_over();
                if is_key_pressed(KeyCode::Enter) {
                    game.restart();
                }

                if is_key_down(KeyCode::Escape) {
                    exit(0);
                }
            }
        }

        draw_values(game.lives, &game.scores);

        next_frame().await
    }
}
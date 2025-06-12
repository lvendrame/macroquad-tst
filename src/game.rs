use macroquad::{color::{self, *}, input::{is_key_down, is_key_pressed, KeyCode}, math::clamp, rand::{self, ChooseRandom}, text::{draw_text, measure_text}, time::get_frame_time, window::{screen_height, screen_width}};

use crate::{constants::*, game_state::GameState, particles::Particles, scores::Scores, shaders::{self, StarfieldShader}, shape::*};

pub struct Game {
    pub game_state: GameState,

    pub lives: u32,
    pub hero: Shape,
    pub enemies: Vec<Shape>,
    pub bullets: Vec<Shape>,

    pub scores: Scores,

    pub shaders: StarfieldShader,

    pub particles: Particles
}

impl Default for Game {
    fn default() -> Self {
        Self {
            game_state: GameState::MainMenu,
            lives: INITIAL_LIVES,
            hero: Self::get_hero(),
            enemies: Default::default(),
            bullets: Default::default(),
            scores: Scores::new(),
            shaders: shaders::StarfieldShader::default(),
            particles: Particles::new()
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    fn get_hero() -> Shape {
        Shape {
            shape_type: ShapeType::Circle,
            size: 32.0,
            speed: MOVEMENT_SPEED,
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            color: color::YELLOW,
            collided: false,
        }
    }

    pub fn get_enemy() -> Shape {
        let size = rand::gen_range(16.0, 64.0);
        let half = size / 2.0;

        Shape {
            shape_type: ShapeType::Square,
            size,
            speed: rand::gen_range(50.0, 150.0),
            x: rand::gen_range(half, screen_width() - half),
            y: -size,
            color: *COLORS.choose().unwrap(),
            collided: false,
        }
    }

    fn get_bullet(&self) -> Shape {
        Shape {
            shape_type: ShapeType::Square,
            size: 5.0,
            speed: self.hero.speed * 2.0,
            x: self.hero.x,
            y: self.hero.y,
            color: RED,
            collided: false,
        }
    }

    fn add_bullet(&mut self) {
        self.bullets.push(self.get_bullet());
    }

    pub fn restart(&mut self) {
        self.lives = INITIAL_LIVES;
        self.scores.score = 0;
        self.hero = Self::get_hero();
        self.enemies.clear();
        self.bullets.clear();
        self.particles.clear();
        self.game_state = GameState::Playing;
    }

    fn try_add_enemy(&mut self) {
        if rand::gen_range(0, 99) >= 95 {
            self.enemies.push(Game::get_enemy());
        }
    }

    fn update_enemies(&mut self, delta_time: f32) {
        for enemy in self.enemies.iter_mut() {
            enemy.y += enemy.speed * delta_time;
        }

        self.enemies.retain(|enemy| enemy.y < screen_height() + enemy.size);
    }

    fn update_bullets(&mut self, delta_time: f32) {
        for bullet in self.bullets.iter_mut() {
            bullet.y -= bullet.speed * delta_time;
        }

        self.bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);
    }

    fn update_playing(&mut self, delta_time: f32) {
        self.try_add_enemy();

        self.update_enemies(delta_time);
        self.update_bullets(delta_time);
    }

    fn check_playing_inputs(&mut self, delta_time: f32) {
        let speed = self.hero.speed * delta_time;
        let radius = self.hero.size / 2.0;

        if is_key_down(KeyCode::Right) {
            self.hero.x += speed;
            self.shaders.inc_by(0.05 * delta_time);
        }
        if is_key_down(KeyCode::Left) {
            self.hero.x -= speed;
            self.shaders.dec_by(0.05 * delta_time);
        }
        if is_key_down(KeyCode::Down) {
            self.hero.y += speed;
        }
        if is_key_down(KeyCode::Up) {
            self.hero.y -= speed;
        }
        if is_key_pressed(KeyCode::Space) {
            self.add_bullet();
        }

        if is_key_down(KeyCode::Escape) {
            self.game_state = GameState::Paused;
        }

        self.hero.x = clamp(self.hero.x, radius, screen_width() - radius);
        self.hero.y = clamp(self.hero.y, radius, screen_height() - radius);
    }

    fn check_hero_collisions(&mut self) -> bool {
        let mut has_collision = false;
        //check if hero collides with any enemy and remove the enemy if it does
        for enemy in self.enemies.iter_mut() {
            if self.hero.collides_with(enemy) {
                enemy.collided = true;
                has_collision = true;
            }
        }

        self.enemies.retain(|enemy| !enemy.collided);

        has_collision
    }

    fn check_bullets_collisions(&mut self) {
        for bullet in self.bullets.iter_mut() {
            for enemy in self.enemies.iter_mut() {
                if bullet.collides_with(&enemy) {
                    bullet.collided = true;
                    enemy.collided = true;

                    self.scores.score += enemy.size.round() as u32;
                    self.particles.create_explosion(enemy.x, enemy.y, enemy.size);
                }
            }
        }

        self.bullets.retain(|bullet| !bullet.collided);
        self.enemies.retain(|enemy| !enemy.collided);
        self.particles.clean();
    }

    fn check_collisions(&mut self) {
        if self.check_hero_collisions() {
            self.lives = self.lives.saturating_sub(1);

            if self.lives < 1
            {
                self.scores.check_score_vs_high_score();
                self.game_state = GameState::GameOver;
            }
        }

        self.check_bullets_collisions();
    }

    fn draw_enemies(&self) {
        for enemy in self.enemies.iter() {
            enemy.draw();
        }
    }

    fn draw_bullets(&self) {
        for bullet in self.bullets.iter() {
            bullet.draw();
        }
    }

    fn draw_playing(&mut self) {
        self.shaders.draw();

        self.hero.draw();
        self.draw_bullets();
        self.draw_enemies();
        self.particles.draw();
    }

    pub fn playing(&mut self) {
        let delta_time = get_frame_time();

        self.update_playing(delta_time);

        self.check_playing_inputs(delta_time);

        self.check_collisions();

        self.draw_playing();

    }

    pub fn main_menu(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
        if is_key_pressed(KeyCode::Enter) {
            self.restart();
        }

        let text = "Press ENTER";
        let text_dimensions = measure_text(text, None, 50, 1.0);
        draw_text(
            text,
            screen_width() / 2.0 - text_dimensions.width / 2.0,
            screen_height() / 2.0,
            50.0,
            WHITE,
        );

    }

    pub fn paused(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.game_state = GameState::Playing;
        }
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        let text = "Paused";
        let text_dimensions = measure_text(text, None, 50, 1.0);
        draw_text(
            text,
            screen_width() / 2.0 - text_dimensions.width / 2.0,
            screen_height() / 2.0,
            50.0,
            WHITE,
        );
    }
}
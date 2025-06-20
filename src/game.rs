use macroquad::{color::*, input::{is_key_down, is_key_pressed, KeyCode}, text::{draw_text, measure_text}, time::get_frame_time, window::{screen_height, screen_width}};

use crate::{sprites_config::SpritesConfig, bullet::Bullet, collidable::Collidable, constants::*, enemies::Enemies, game_state::GameState, hero::Hero, particles::Particles, scores::Scores, shaders::{self, StarfieldShader}};

pub struct Game {
    pub game_state: GameState,

    pub lives: u32,
    pub hero: Hero,
    pub enemies: Enemies,
    pub bullets: Vec<Bullet>,

    pub scores: Scores,

    pub shaders: StarfieldShader,

    pub particles: Particles,

    pub sprites_config: SpritesConfig,
}

impl Game {
    pub async fn new() -> Self {
        Self {
            game_state: GameState::MainMenu,
            lives: INITIAL_LIVES,
            hero: Hero::new(),
            enemies: Default::default(),
            bullets: Default::default(),
            scores: Scores::new(),
            shaders: shaders::StarfieldShader::default(),
            particles: Particles::new(),
            sprites_config: SpritesConfig::new().await,
        }
    }

    fn add_bullet(&mut self) {
        self.bullets.push(Bullet::new(&self.hero));
    }

    pub fn restart(&mut self) {
        self.lives = INITIAL_LIVES;
        self.scores.score = 0;
        self.hero.restart();
        self.enemies.clear();
        self.bullets.clear();
        self.particles.clear();
        self.game_state = GameState::Playing;
    }

    fn update_bullets(&mut self, delta_time: f32) {
        for bullet in self.bullets.iter_mut() {
            bullet.update(delta_time);
        }

        self.bullets.retain(|bullet| bullet.position().y > 0.0 - bullet.size() / 2.0);
    }

    fn update_playing(&mut self, delta_time: f32) {
        self.enemies.update(delta_time);
        self.update_bullets(delta_time);
    }

    fn check_playing_inputs(&mut self, delta_time: f32) {
        self.hero.check_inputs(delta_time, &mut self.shaders);

        if is_key_pressed(KeyCode::Space) {
            self.add_bullet();
        }

        if is_key_down(KeyCode::Escape) {
            self.game_state = GameState::Paused;
        }
    }

    fn check_hero_collisions(&mut self) -> bool {
        self.enemies.collides_with(&self.hero, |enemy| {
            self.scores.score += enemy.size().round() as u32;
            self.particles.create_explosion(enemy.position().x, enemy.position().y, enemy.size(), &self.sprites_config.explosion_texture);
        })
    }

    fn check_bullets_collisions(&mut self) {
        for bullet in self.bullets.iter_mut() {
            if self.enemies.collides_with(bullet, |enemy| {
                self.scores.score += enemy.size().round() as u32;
                self.particles.create_explosion(enemy.position().x, enemy.position().y, enemy.size(), &self.sprites_config.explosion_texture);
            }) {
                bullet.set_collided(true);
            };
        }

        self.bullets.retain(|bullet| !bullet.get_collided());
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

    fn draw_bullets(&self) {
        for bullet in self.bullets.iter() {
            bullet.draw(&self.sprites_config);
        }
    }

    fn draw_playing(&mut self) {
        self.shaders.draw();

        self.hero.draw(&self.sprites_config);
        self.draw_bullets();
        self.enemies.draw(&self.sprites_config);
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

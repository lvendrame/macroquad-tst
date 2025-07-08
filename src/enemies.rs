use macroquad::{prelude::animation::AnimatedSprite, rand, window::screen_height};

use crate::{sprites_config::SpritesConfig, collidable::Collidable, enemy::Enemy};

pub struct Enemies {
    list: Vec<Enemy>,
    sprite_small: AnimatedSprite,
    sprite_medium: AnimatedSprite,
    sprite_big: AnimatedSprite,
}

impl Default for Enemies {
    fn default() -> Self {
        Self::new()
    }
}

impl Enemies {
    pub fn new() -> Self {
        Enemies {
            list: Vec::new(),
            sprite_small: SpritesConfig::get_enemy_small_sprite(),
            sprite_medium: SpritesConfig::get_enemy_medium_sprite(),
            sprite_big: SpritesConfig::get_enemy_big_sprite(),
        }
    }

    fn try_add_enemy(&mut self) {
        if rand::gen_range(0, 99) >= 95 {
            self.list.push(Enemy::new());
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.try_add_enemy();

        for enemy in self.list.iter_mut() {
            enemy.update(delta_time);
        }

        self.list.retain(|enemy| enemy.position().y < screen_height() + enemy.size());

        self.sprite_small.update();
        self.sprite_medium.update();
        self.sprite_big.update();
    }

    pub fn collides_with<T: Collidable, F: FnMut(&Enemy)>(&mut self, other: &T, mut on_collision: F) -> bool {
        let mut has_collision = false;

        for enemy in self.list.iter_mut() {
            if enemy.collides_with(other) {
                enemy.shape.collided = true;
                has_collision = true;

                on_collision(&enemy);
            }
        }

        self.list.retain(|enemy| !enemy.shape.collided);

        has_collision
    }

    pub fn draw(&self, sprites_config: &SpritesConfig) {
        let small_frame = self.sprite_small.frame();
        let medium_frame = self.sprite_medium.frame();
        let big_frame = self.sprite_big.frame();

        for enemy in self.list.iter() {
            let size = enemy.size();
            if size < 32. {
                enemy.draw(&sprites_config.enemy_small_texture, &small_frame);
            } else if size < 48. {
                enemy.draw(&sprites_config.enemy_medium_texture, &medium_frame);
            } else {
                enemy.draw(&sprites_config.enemy_big_texture, &big_frame);
            }
        }
    }

    pub fn clear(&mut self) {
        self.list = Vec::new();
    }
}
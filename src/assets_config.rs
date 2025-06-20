use macroquad::{file::set_pc_assets_folder, prelude::animation::{AnimatedSprite, Animation}, texture::{build_textures_atlas, load_texture, FilterMode, Texture2D}};

use crate::constants::ASSETS_PATH;

pub struct AssetsConfig {
    pub ship_texture: Texture2D,
    pub bullet_texture: Texture2D,
    pub explosion_texture: Texture2D,
    pub enemy_small_texture: Texture2D,
    pub enemy_medium_texture: Texture2D,
    pub enemy_big_texture: Texture2D,
}


impl AssetsConfig {
    pub async fn new() -> AssetsConfig {
        set_pc_assets_folder(ASSETS_PATH);

        let ship_texture = AssetsConfig::load_asset_texture("ship.png").await;
        let bullet_texture = AssetsConfig::load_asset_texture("laser-bolts.png").await;
        let explosion_texture = AssetsConfig::load_asset_texture("explosion.png").await;
        let enemy_small_texture = AssetsConfig::load_asset_texture("enemy-small.png").await;
        let enemy_medium_texture = AssetsConfig::load_asset_texture("enemy-medium.png").await;
        let enemy_big_texture = AssetsConfig::load_asset_texture("enemy-big.png").await;

        let config = AssetsConfig {
            ship_texture,
            bullet_texture,
            explosion_texture,
            enemy_small_texture,
            enemy_medium_texture,
            enemy_big_texture,
        };

        build_textures_atlas();

        config
    }

    async fn load_asset_texture(filename: &str) -> Texture2D {
        let texture = load_texture(filename)
            .await
            .expect(&format!("Couldn't load file {}", filename));

        texture.set_filter(FilterMode::Nearest);

        texture
    }

    pub fn get_bullet_sprite() -> AnimatedSprite {
        let mut bullet_sprite = AnimatedSprite::new(
            16,
            16,
            &[
                Animation {
                    name: "bullet".into(),
                    row: 0,
                    frames: 2,
                    fps: 12,
                },
                Animation {
                    name: "bolt".into(),
                    row: 1,
                    frames: 2,
                    fps: 12,
                },
            ],
            true
        );

        bullet_sprite.set_animation(1);

        bullet_sprite
    }

    pub fn get_ship_sprite() -> AnimatedSprite {
        AnimatedSprite::new(
            16,
            24,
            &[
                Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 2,
                    fps: 12,
                },
                Animation {
                    name: "left".to_string(),
                    row: 2,
                    frames: 2,
                    fps: 12,
                },
                Animation {
                    name: "right".to_string(),
                    row: 4,
                    frames: 2,
                    fps: 12,
                },
            ],
            true,
        )
    }

    pub fn get_enemy_small_sprite() -> AnimatedSprite {
        AnimatedSprite::new(
            17,
            16,
            &[
                Animation {
                    name: "enemy_small".to_string(),
                    row: 0,
                    frames: 2,
                    fps: 12,
                },
            ],
            true,
        )
    }

    pub fn get_enemy_medium_sprite() -> AnimatedSprite {
        AnimatedSprite::new(
            32,
            16,
            &[
                Animation {
                    name: "enemy_medium".to_string(),
                    row: 0,
                    frames: 2,
                    fps: 12,
                },
            ],
            true,
        )
    }

    pub fn get_enemy_big_sprite() -> AnimatedSprite {
        AnimatedSprite::new(
            32,
            32,
            &[
                Animation {
                    name: "enemy_big".to_string(),
                    row: 0,
                    frames: 2,
                    fps: 12,
                },
            ],
            true,
        )
    }

}
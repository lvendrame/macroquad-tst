use macroquad::{color::{ORANGE, RED}, math::{vec2, Vec2}};
use macroquad_particles::{ColorCurve, Emitter, EmitterConfig};

use crate::particles;

pub struct Particles {
    pub explosions: Vec<(Emitter, Vec2)>,
}

impl Default for Particles {
    fn default() -> Self {
        Self::new()
    }
}

impl Particles {

    pub fn new() -> Particles {
        Particles {
            explosions: vec![],
        }
    }

    fn get_explosion_emitter() -> EmitterConfig {
        EmitterConfig {
            local_coords: false,
            one_shot: true,
            emitting: true,
            lifetime: 0.6,
            lifetime_randomness: 0.3,
            explosiveness: 0.65,
            initial_direction_spread: 2.0 * std::f32::consts::PI,
            initial_velocity: 300.0,
            initial_velocity_randomness: 0.8,
            size: 3.0,
            size_randomness: 0.3,
            colors_curve: ColorCurve {
                start: RED,
                mid: ORANGE,
                end: RED,
            },
            ..Default::default()
        }

    }

    pub fn create_explosion(&mut self, x: f32, y: f32, size: f32) {
        self.explosions.push((
            Emitter::new(EmitterConfig {
                amount: size.round() as u32 * 2,
                ..Self::get_explosion_emitter()
            }),
            vec2(x, y)
        ));
    }

    pub fn clear(&mut self) {
        self.explosions.clear();
    }

    pub fn clean(&mut self) {
        self.explosions.retain(|(explosion, _)| explosion.config.emitting);
    }

    pub fn draw(&mut self) {
        for (explosion, coords) in self.explosions.iter_mut() {
            explosion.draw(*coords);
        }
    }

}
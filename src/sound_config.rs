use macroquad::audio::{load_sound, play_sound, play_sound_once, PlaySoundParams, Sound};

pub struct SoundConfig {
    pub theme_music: Sound,
    pub sound_explosion: Sound,
    pub sound_laser: Sound,
}

impl SoundConfig {
    pub async fn new() -> Self {
        let theme_music = load_sound("assets/8bit-spaceshooter.ogg").await.unwrap();
        let sound_explosion = load_sound("assets/explosion.wav").await.unwrap();
        let sound_laser = load_sound("assets/laser.wav").await.unwrap();

        SoundConfig {
            theme_music,
            sound_explosion,
            sound_laser,
        }
    }

    pub fn play_theme_music(&self) {
        play_sound(
            &self.theme_music,
            PlaySoundParams {
                looped: true,
                volume: 1.,
            },
        );
    }

    pub fn play_sound_explosion(&self) {
        play_sound_once(&self.sound_explosion);
    }

    pub fn play_sound_laser(&self) {
        play_sound_once(&self.sound_laser);
    }
}
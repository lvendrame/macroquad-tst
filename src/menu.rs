use macroquad::{
    color::WHITE, file::load_file,
    input::{is_key_pressed, KeyCode},
    math::{vec2, RectOffset},
    texture::load_image,
    ui::{hash, root_ui, Skin, Style},
    window::{screen_height, screen_width}
};

pub struct Menu;

impl Menu {
    async fn create_window_style() -> Style {
        let window_background = load_image("assets/window_background.png").await.unwrap();

        root_ui().style_builder()
            .background(window_background)
            .background_margin(RectOffset::new(32., 76., 44., 20.))
            .margin(RectOffset::new(0., -40., 0., 0.))
            .build()
    }

    async fn create_button_style(font: &[u8]) -> Style {
        let button_background = load_image("assets/button_background.png").await.unwrap();
        let button_clicked_background = load_image("assets/button_clicked_background.png").await.unwrap();

        root_ui().style_builder()
            .background(button_background)
            .background_hovered(button_clicked_background)
            .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
            .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
            .font(font)
            .unwrap()
            .text_color(WHITE)
            .font_size(64)
            .build()
    }

    fn create_label_style(font: &[u8]) -> Style {
        root_ui().style_builder()
            .font(font)
            .unwrap()
            .text_color(WHITE)
            .font_size(28)
            .build()
    }

    pub async fn initialize() {
        let font = load_file("assets/atari_games.ttf").await.unwrap();

        let window_style = Self::create_window_style().await;
        let button_style = Self::create_button_style(&font).await;
        let label_style = Self::create_label_style(&font);

        let ui_skin = Skin {
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        };

        root_ui().push_skin(&ui_skin);
    }

    pub fn main_menu<C: FnMut(), P: FnMut()>(mut on_close_click: C, mut on_play_click: P) {
        let window_size = vec2(370.0, 320.0);

        root_ui().window(
            hash!(),
                vec2(
                    screen_width() / 2.0 - window_size.x / 2.0,
                    screen_height() / 2.0 - window_size.y / 2.0,
                ),
                window_size,
                |ui| {
                    ui.label(vec2(80.0, -34.0), "Main Menu");
                    if ui.button(vec2(65.0, 25.0), "(P)lay") ||
                        is_key_pressed(KeyCode::P) {
                        on_play_click();
                    }
                    if ui.button(vec2(65.0, 125.0), "(Q)uit") ||
                        is_key_pressed(KeyCode::Q) {
                        on_close_click();
                    }
                },
            );
    }
}
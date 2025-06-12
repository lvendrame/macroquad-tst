use macroquad::{color::WHITE, math::vec2, prelude::{gl_use_default_material, gl_use_material, load_material, Material, MaterialParams, ShaderSource, UniformDesc, UniformType}, texture::{draw_texture_ex, render_target, DrawTextureParams, FilterMode, RenderTarget}, window::{screen_height, screen_width}};

const FRAGMENT_SHADER: &str = include_str!("starfield-shader.glsl");

const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";

pub struct StarfieldShader {
    pub material: Material,
    pub direction_modifier: f32,
    pub render_target: RenderTarget,
}

impl Default for StarfieldShader {
    fn default() -> Self {
        Self::new()
    }
}

impl StarfieldShader {
    pub fn new() -> Self {
        let render_target = Self::create_render_target();
        let material = Self::load_shader_material();
        StarfieldShader {
            render_target,
            material,
            direction_modifier: 0.,
        }
    }

    pub fn draw(&mut self) {
        let material = &mut self.material;

        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", self.direction_modifier);

        gl_use_material(material);

        draw_texture_ex(
            &self.render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }

    fn create_render_target() -> RenderTarget {
        let render_target = render_target(320, 150);
        render_target.texture.set_filter(FilterMode::Nearest);
        render_target
    }

    fn load_shader_material() -> Material {
        load_material(
            ShaderSource::Glsl {
                vertex: VERTEX_SHADER,
                fragment: FRAGMENT_SHADER
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("iResolution", UniformType::Float2),
                    UniformDesc::new("direction_modifier", UniformType::Float1),
                ],
                ..Default::default()
            })
            .unwrap()
    }

    pub fn inc_by(&mut self, value: f32) {
        self.direction_modifier += value;
    }

    pub fn dec_by(&mut self, value: f32) {
        self.direction_modifier -= value;
    }

}
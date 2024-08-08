use macroquad::camera::{Camera2D, set_camera, set_default_camera};
use macroquad::color::{BLACK, BLUE};
use macroquad::math::Vec2;
use macroquad::prelude::{RenderTarget, Texture2D};
use macroquad::shapes::{draw_line, draw_rectangle};
use macroquad::texture::render_target;
use macroquad::window::clear_background;

pub struct Habitat {
    // virtual screen which camera2d will draw to
    render_target: RenderTarget,
    camera_target: Vec2,
    camera_zoom: Vec2,
    habitat_size: (u32, u32),
}

impl Habitat {
    pub fn new(habitat_size: (u32, u32)) -> Self {
        let render_target = render_target(habitat_size.0, habitat_size.1);
        let camera_zoom = Vec2::new(5.0 / &render_target.texture.width(), 5.0 / &render_target.texture.height());

        Self {
            render_target,
            camera_target: Vec2::new(habitat_size.0 as f32/2f32, habitat_size.1 as f32/2f32),
            camera_zoom,
            habitat_size,
        }
    }

    pub fn draw(&mut self) {
        set_camera(&Camera2D {
            zoom: self.camera_zoom,
            target: self.camera_target,
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        clear_background(BLACK);
        draw_rectangle(0f32, 0f32, self.habitat_size.0 as f32, self.habitat_size.1 as f32, BLUE);
        draw_line(0f32, 0f32, self.habitat_size.0 as f32, self.habitat_size.1 as f32, 2f32, BLACK);

        set_default_camera();
    }

    pub fn get_texture(&self) -> &Texture2D {
        &self.render_target.texture
    }

    pub fn move_target(&mut self, direction: Vec2) {
        self.camera_target += direction * (0.1f32/self.camera_zoom.length());
        if self.camera_target.x < 0f32 {self.camera_target.x = 0f32}
        if self.camera_target.x > self.habitat_size.0 as f32 { self.camera_target.x = self.habitat_size.0 as f32}
        if self.camera_target.y < 0f32 {self.camera_target.y = 0f32}
        if self.camera_target.y > self.habitat_size.1 as f32 { self.camera_target.y = self.habitat_size.1 as f32}

    }
    pub fn zoom(&mut self, factor: f32) {
        self.camera_zoom.x *= factor;
        self.camera_zoom.y *= factor;
    }
}
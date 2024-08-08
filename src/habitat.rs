use macroquad::camera::{Camera2D, set_camera, set_default_camera};
use macroquad::color::BLACK;
use macroquad::prelude::{Rect, RenderTarget, Texture2D};
use macroquad::shapes::draw_line;
use macroquad::texture::render_target;

pub struct Habitat {
    // camera which will draw to part of the screen
    camera2d: Camera2D,
    // virtual screen which camera2d will draw to
    render_target: RenderTarget,
    habitat_size: (u32, u32),
}

impl Habitat {
    pub fn new(habitat_size: (u32, u32), position: (f32, f32), width: f32, height: f32) -> Self {
        let render_target = render_target(habitat_size.0, habitat_size.1);
        let mut render_target_cam = Camera2D::from_display_rect(Rect::new(position.0, position.1, width, height));
        render_target_cam.render_target = Some(render_target.clone());
        Self {
            camera2d: render_target_cam,
            render_target,
            habitat_size,
        }
    }

    pub fn draw_test(&mut self) {
        set_camera(&self.camera2d);

        draw_line(0f32, 0f32, self.habitat_size.0 as f32, self.habitat_size.1 as f32, 2f32, BLACK);

        set_default_camera();
    }

    pub fn get_texture(&self) -> &Texture2D {
        &self.render_target.texture
    }
}
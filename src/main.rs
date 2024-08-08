
mod habitat;

use macroquad::prelude::*;
use macroquad::prelude::KeyCode::*;
use simulation2d_library::noise::NoiseFlowField;
use crate::habitat::Habitat;

const PADDING: f32 = 20f32;

fn get_config() -> Conf {
    Conf {
        window_title: "".to_string(),
        window_width: 800,
        window_height: 500,
        high_dpi: false,
        fullscreen: false,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(get_config())]
async fn main() {
    let width: usize = 20;
    let height: usize = 20;
    let spacing = (screen_width()/(width+1) as f32, screen_height()/(height + 1) as f32);
    let line_length = 15f32;
    let mut flow_field = NoiseFlowField::new(width, height,20f32, 0.01f32, 1);
    let mut buffer: &Vec<Vec2>;

    let mut habitat = Habitat::new((1000,1000));


    loop {
        habitat.draw();
        buffer = flow_field.get_next();
        if is_key_down(Escape) {break}
        if is_key_down(Left) {habitat.move_target(Vec2::new(-1f32, 0f32))}
        if is_key_down(Right) {habitat.move_target(Vec2::new(1f32, 0f32))}
        if is_key_down(Up) {habitat.move_target(Vec2::new(0f32, -1f32))}
        if is_key_down(Down) {habitat.move_target(Vec2::new(0f32, 1f32))}
        if is_key_down(V) {habitat.zoom(1.02)}
        if is_key_down(B) {habitat.zoom(0.98)}
        clear_background(WHITE);
        draw_texture_ex(habitat.get_texture(), screen_width() - screen_height() + PADDING, PADDING, WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(screen_height() - 2f32*PADDING, screen_height() - 2f32*PADDING)),
            ..Default::default()
        });
        draw_rectangle_lines(screen_width() - screen_height() + PADDING, PADDING, screen_height() - 2f32*PADDING, screen_height() - 2f32*PADDING, 10f32, BLACK);

/*
        for x in 0..width {
            for y in 0..height {
                let noise_vec = buffer[x + y*width];
                let (pos_x, pos_y) = (x as f32 * spacing.0 + spacing.0, y as f32 * spacing.1 + spacing.1);
                draw_line(pos_x, pos_y,
                          pos_x + noise_vec.x * line_length,
                          pos_y + noise_vec.y * line_length,
                          1f32,
                          BLACK);
            }
        }
*/


        next_frame().await;
    }
}


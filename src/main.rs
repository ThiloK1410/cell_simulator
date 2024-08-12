
mod habitat;
mod cell;

use std::num::Wrapping;
use macroquad::prelude::*;
use macroquad::prelude::KeyCode::*;
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
        window_resizable: true,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(get_config())]
async fn main() {

    let mut habitat = Habitat::new((2000,2000));


    // frame counter with turned off overflow protection
    let mut frame_counter: Wrapping<usize> = Wrapping(0) ;
    let mut fps: usize = 0;


    loop {
        frame_counter += 1;

        habitat.update(frame_counter);

        // drawing habitat to its internal render target
        habitat.draw();


        //checking inputs
        if is_key_down(Escape) {break}
        if is_key_down(Left) {habitat.move_target(Vec2::new(-1f32, 0f32))}
        if is_key_down(Right) {habitat.move_target(Vec2::new(1f32, 0f32))}
        if is_key_down(Up) {habitat.move_target(Vec2::new(0f32, -1f32))}
        if is_key_down(Down) {habitat.move_target(Vec2::new(0f32, 1f32))}
        if is_key_down(V) {habitat.zoom(1.02)}
        if is_key_down(B) {habitat.zoom(0.98)}
        if is_key_pressed(Space) {}
        if is_key_pressed(D) {habitat.draw_flow_field_bool = !habitat.draw_flow_field_bool}
        if is_key_pressed(X) {
            let n = 10;
            let spacing_x = (habitat.get_size().0) / (n+1);
            let spacing_y = (habitat.get_size().1) / (n+1);
            for i in 1..=n {
                for j in 1..=n {
                    habitat.spawn_cell(Vec2::new((i*spacing_x) as f32, (j*spacing_y) as f32))
                }
            }

        }

        //drawing screen
        clear_background(WHITE);
        // drawing the texture provided by habitat
        draw_texture_ex(habitat.get_texture(), screen_width() - screen_height() + PADDING, PADDING, WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(screen_height() - 2f32*PADDING, screen_height() - 2f32*PADDING)),
            ..Default::default()
        });
        draw_rectangle_lines(screen_width() - screen_height() + PADDING, PADDING, screen_height() - 2f32*PADDING, screen_height() - 2f32*PADDING, 10f32, BLACK);
        let text_params = TextParams {
            font: None,
            font_size: 16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color: BLACK,
        };
        //drawing debug information
        draw_text_ex(&format!("FPS: {}", fps), 5f32, 10f32, text_params.clone());
        draw_text_ex(&format!("Cell Count: {}", habitat.cells.len()), 5f32, 10f32 + text_params.font_size as f32, text_params.clone());

        // update fps every 10 frames
        if frame_counter % Wrapping(10) == Wrapping(0) {
            //taking average from current and last fps
            fps = fps/2 + (0.5f32/get_frame_time()) as usize;
        }

        next_frame().await;
    }
}


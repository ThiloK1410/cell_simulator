mod habitat;
mod cell;

use std::num::Wrapping;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::prelude::KeyCode::*;
use macroquad::ui::{root_ui};
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
    let mut habitat = Habitat::new((500, 500));
    let habitat_size = Vec2::new(habitat.get_size().0 as f32, habitat.get_size().1 as f32);


    // frame counter with turned off overflow protection
    let mut frame_counter: Wrapping<usize> = Wrapping(0);
    let mut fps: usize = 0;

    let mut flow_speed_update_val: f32 = 0.1f32;
    let mut flow_change_update_val: f32 = 1f32;
    let mut physics_iterations: usize = 1;

    let mut ui_tab: usize = 1;

    loop {
        frame_counter += 1;

        let texture_pos: Vec2 = Vec2::new(screen_width() - screen_height() + PADDING, PADDING);
        let texture_dim: Vec2 = Vec2::new(screen_height() - 2f32 * PADDING, screen_height() - 2f32 * PADDING);

        habitat.update(frame_counter, flow_speed_update_val.powi(2), physics_iterations);


        root_ui().window(hash!(), vec2(PADDING, 100.0), vec2(screen_width() - screen_height() - PADDING, 200.0), |ui| {
            ui.label(None, "settings");
            ui.separator();
            ui.same_line(20f32);
            if ui.button(None, "general") {
                ui_tab = 1;
            }
            ui.same_line(100f32);
            if ui.button(None, "flow") {
                ui_tab = 2;
            }
            match ui_tab {
                1 => {
                    ui.label(None, &format!("physics iterations: {}", physics_iterations));
                    ui.same_line(160f32);
                    if ui.button(None, "+") {
                        physics_iterations += 1;
                    }
                    ui.same_line(180f32);
                    if ui.button(None, "-") {
                        physics_iterations = (physics_iterations-1).max(0);
                    }
                    ui.label(None, "scale collision force:");
                    ui.same_line(-50f32);
                    ui.checkbox(hash!(), "", &mut habitat.scale_collision_force);
                }
                2 => {
                    // Create a button
                    ui.checkbox(hash!(), "show", &mut habitat.draw_flow_field_bool);
                    ui.slider(hash!(), "flow power", 0.1f32..2f32, &mut flow_speed_update_val);
                    ui.slider(hash!(), "flow change", 1f32..100f32, &mut flow_change_update_val);
                }
                _ => ()
            }
        });
        habitat.set_z_offset(0.001 * flow_change_update_val);

        // drawing habitat to its internal render target
        habitat.draw();


        //checking inputs
        if is_key_down(Escape) { break; }
        if is_key_down(Left) { habitat.move_target(Vec2::new(-1f32, 0f32)) }
        if is_key_down(Right) { habitat.move_target(Vec2::new(1f32, 0f32)) }
        if is_key_down(Up) { habitat.move_target(Vec2::new(0f32, -1f32)) }
        if is_key_down(Down) { habitat.move_target(Vec2::new(0f32, 1f32)) }
        if is_key_down(V) { habitat.zoom(1.02) }
        if is_key_down(B) { habitat.zoom(0.98) }
        if is_key_pressed(Space) {}
        if is_key_pressed(D) { habitat.draw_flow_field_bool = !habitat.draw_flow_field_bool }
        if is_key_pressed(X) {
            let n = 10;
            let spacing_x = (habitat.get_size().0) / (n + 1);
            let spacing_y = (habitat.get_size().1) / (n + 1);
            for i in 1..=n {
                for j in 1..=n {
                    habitat.spawn_cell(Vec2::new((i * spacing_x) as f32, (j * spacing_y) as f32))
                }
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            habitat.set_focused_cell(get_texture_pos(Vec2::from(mouse_position()), texture_pos, texture_dim, habitat.camera_zoom, habitat.camera_target, habitat_size));
        }

        //drawing screen
        clear_background(WHITE);
        // drawing the texture provided by habitat
        draw_texture_ex(habitat.get_texture(), texture_pos.x, texture_pos.y, WHITE,
                        DrawTextureParams {
                            dest_size: Some(texture_dim),
                            ..Default::default()
                        });
        draw_rectangle_lines(screen_width() - screen_height() + PADDING, PADDING, screen_height() - 2f32 * PADDING, screen_height() - 2f32 * PADDING, 10f32, BLACK);
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
        draw_text_ex(&format!("current idx {}", if let Some(x) = habitat.focused_cell_idx {x as i32} else {-1}), 5f32, 10f32 + 2.0 * text_params.font_size as f32, text_params.clone());
        draw_text_ex(&format!("current target: {}", habitat.camera_target), 5f32, 10f32 + 3.0 * text_params.font_size as f32, text_params.clone());

        // update fps every 10 frames
        if frame_counter % Wrapping(10) == Wrapping(0) {
            //taking average from current and last fps
            fps = fps / 2 + (0.5f32 / get_frame_time()) as usize;
        }

        next_frame().await;
    }

    //takes screen position and gives the relative texture position, only if position is inside it
    pub fn get_texture_pos(pos: Vec2, tex_pos: Vec2, tex_dim: Vec2, zoom: Vec2, target: Vec2, habitat_size: Vec2) -> Option<Vec2> {
        //checks if pos coordinates are inside texture
        if pos.cmpge(tex_pos).all() && pos.cmple(tex_pos + tex_dim).all() {
            let center = tex_pos + tex_dim / 2f32;
            let abs_pos = (2f32 * ((pos - center) / tex_dim)) / zoom + target;
            if abs_pos.cmpge(Vec2::ZERO).all() && abs_pos.cmple(habitat_size).all() {
                return Some(abs_pos);
            }
        }
        return None;
    }
}


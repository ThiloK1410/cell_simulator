mod canvas_test;

use macroquad::prelude::*;
use macroquad::prelude::KeyCode::Escape;
use simulation2d_library::noise::NoiseHandler;

fn get_config() -> Conf {
    Conf {
        window_title: "".to_string(),
        window_width: 500,
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
    let mut noise_handler_x = NoiseHandler::new(
        width, height, 20f64, 0.001, 1);
    let mut noise_handler_y = NoiseHandler::new(
        width, height, 20f64, 0.001, 2);
    let mut buffer_x;
    let mut buffer_y;


    loop {
        buffer_x = noise_handler_x.get_next();
        buffer_y = noise_handler_y.get_next();
        if is_key_down(Escape) {break};
        clear_background(WHITE);


        for x in 0..width {
            for y in 0..height {
                let x_noise = buffer_x[x + y*width] as f32;
                let y_noise = buffer_y[x + y*width] as f32;
                let noise_vec = Vec2::new(x_noise, y_noise).normalize_or_zero();
                let (pos_x, pos_y) = (x as f32 * spacing.0 + spacing.0, y as f32 * spacing.1 + spacing.1);
                draw_line(pos_x, pos_y,
                          pos_x + noise_vec.x * line_length,
                          pos_y + noise_vec.y * line_length,
                          1f32,
                          BLACK);
            }
        }

        next_frame().await;
    }
}


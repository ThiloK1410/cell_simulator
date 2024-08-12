use std::cell::RefCell;
use std::num::Wrapping;
use std::rc::Rc;
use macroquad::camera::{Camera2D, set_camera, set_default_camera};
use macroquad::color::{BLACK, BLUE};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_line, RenderTarget, Texture2D};
use macroquad::shapes::{draw_circle, draw_rectangle};
use macroquad::texture::render_target;
use macroquad::window::clear_background;
use simulation2d_library::noise::NoiseFlowField;
use crate::cell::Cell;
use rstar::RTree;

pub struct Habitat {
    // virtual screen which camera2d will draw to
    render_target: RenderTarget,
    camera_target: Vec2,
    camera_zoom: Vec2,
    habitat_size: (u32, u32),
    pub cells: Vec<Cell>,
    flow_field: NoiseFlowField,
    flow_field_buffer: Vec<Vec2>,
    pub draw_flow_field_bool: bool,
    r_tree: RTree<Cell>
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
            cells: Vec::new(),
            flow_field: NoiseFlowField::new((habitat_size.0/30) as usize, (habitat_size.1/30) as usize, 10f32, 0.003, 1),
            flow_field_buffer: Vec::new(),
            draw_flow_field_bool: false,
            r_tree: RTree::new(),
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.habitat_size
    }

    pub fn draw(&mut self) {
        // set camera to draw on own render target
        set_camera(&Camera2D {
            zoom: self.camera_zoom,
            target: self.camera_target,
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        clear_background(BLACK);
        draw_rectangle(0f32, 0f32, self.habitat_size.0 as f32, self.habitat_size.1 as f32, BLUE);

        for cell in &self.cells {
            draw_circle(
                cell.pos.x,
                cell.pos.y,
                cell.size,
                BLACK
            )
        }

        if self.draw_flow_field_bool {self.draw_flow_field()}

        set_default_camera();
    }

    pub fn update(&mut self, frame_id: Wrapping<usize>) {
        // update flow field
        if frame_id % Wrapping(10) == Wrapping(0) {
            self.flow_field_buffer = self.flow_field.get_next().clone();
        }

        // update velocity
        for cell in self.cells.iter_mut() {
            let (rel_x, rel_y) = (cell.pos.x/self.habitat_size.0 as f32, cell.pos.y/self.habitat_size.1 as f32);
            cell.vel = cell.vel*0.9 + self.flow_field.get_pos(rel_x, rel_y) * 0.01;

        }
        // apply velocity
        for cell in self.cells.iter_mut() {
            cell.pos += cell.vel
        }



        // collision checks between cells
        let mut collision_update: Vec<Vec2> = vec![Vec2::ZERO; self.cells.len()];
        let mut collision_counter: Vec<usize> = vec![0; self.cells.len()];
        for (idx, cell1) in self.cells.iter().enumerate() {
            for cell2 in self.cells.iter() {
                let min_dist = cell1.size + cell2.size;
                let dist_vec = cell2.pos - cell1.pos;
                let dist = dist_vec.length();
                if dist < min_dist {
                    collision_update[idx] -= dist_vec.normalize_or_zero() * (min_dist-dist);
                    collision_counter[idx] += 1
                }
            }
        }
        // check boundary breaks, then apply collision update
        for (idx, cell) in self.cells.iter_mut().enumerate() {
            let new_pos = cell.pos + collision_update[idx] / (collision_counter[idx] as f32).max(1f32);
            match new_pos {
                _ if new_pos.x < 0f32 + cell.size => {
                    cell.pos.x = cell.size;
                    cell.pos.y = new_pos.y;
                }
                _ => {}
            }
            if cell.pos.x < 0f32 + cell.size {
                cell.pos.x = cell.size;
                collision_update[idx].x = collision_update[idx].x.max(0f32)
            }
            if cell.pos.x > self.habitat_size.0 as f32-cell.size {
                cell.pos.x = self.habitat_size.0 as f32-cell.size;
                collision_update[idx].x = collision_update[idx].x.min(0f32)
            }
            if cell.pos.y < 0f32 + cell.size {
                cell.pos.y = 0f32 + cell.size;
                collision_update[idx].y = collision_update[idx].y.max(0f32)
            }
            if cell.pos.y > self.habitat_size.1 as f32 - cell.size {
                cell.pos.y = self.habitat_size.1 as f32-cell.size;
                collision_update[idx].y = collision_update[idx].y.min(0f32)
            }
            // collision update applied here
            cell.pos += collision_update[idx] / (collision_counter[idx] as f32).max(1f32);
        }
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

    pub fn spawn_cell(&mut self, pos: Vec2) {
        self.cells.push(Cell::new(Vec2::new(pos.x, pos.y), 5f32))
    }

    pub fn draw_flow_field(&self) {
        let line_length = 20f32;
        let (width, height) = self.flow_field.get_dim();
        let spacing = (self.habitat_size.0 as f32/(width+1) as f32, self.habitat_size.0 as f32/(height + 1) as f32);
        for x in 0..width {
            for y in 0..height {
                let noise_vec = self.flow_field_buffer[x + y*width];
                let (pos_x, pos_y) = (x as f32 * spacing.0 + spacing.0, y as f32 * spacing.1 + spacing.1);
                draw_line(pos_x, pos_y,
                          pos_x + noise_vec.x * line_length,
                          pos_y + noise_vec.y * line_length,
                          1f32,
                          BLACK);
            }
        }
    }
}
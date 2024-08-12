use macroquad::math::Vec2;
use rstar::{AABB, PointDistance, RTreeObject};

pub struct Cell {
    pub pos: Vec2,
    pub size: f32,
    pub vel: Vec2,
}

impl Cell {
    pub fn new(pos: Vec2, size: f32) -> Self {
        Self {
            pos,
            size,
            vel: Vec2::default()
        }
    }
}
impl RTreeObject for &Cell {
    type Envelope = AABB<(f32, f32)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners((self.pos.x-self.size, self.pos.y-self.size), (self.pos.x+self.size, self.pos.y+self.size))
    }
}
impl PointDistance for &Cell {
    fn distance_2(&self, point: &(f32, f32)) -> f32 {
        let d_x = self.pos.x - point.0;
        let d_y = self.pos.y - point.1;
        let distance_to_origin = (d_x * d_x + d_y * d_y).sqrt();
        let distance_to_ring = distance_to_origin - self.size;
        let distance_to_circle = f32::max(0.0, distance_to_ring);
        // We must return the squared distance!
        distance_to_circle * distance_to_circle
    }
    fn contains_point(&self, point: &(f32, f32)) -> bool
    {
        let d_x = self.pos.x - point.0;
        let d_y = self.pos.y - point.1;
        let distance_to_origin_2 = d_x * d_x + d_y * d_y;
        let radius_2 = self.size * self.size;
        distance_to_origin_2 <= radius_2
    }
}
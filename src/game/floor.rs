use super::EDGE_SIZE;
use cgmath::Point2;

#[derive(Debug)]
pub struct Floor {
    pub pos: Point2<f32>,
    pub edge_size: f32,
//    pub mass: f32,
//    pub inv_mass: f32,
}

impl Floor {
    pub fn new(pos: Point2<f32>) -> Floor {
        Floor {
            pos,
            edge_size: EDGE_SIZE,
//            mass: 0.0,
//            inv_mass: 0.0,
        }
    }

    pub fn get_center(&self) -> Point2<f32> {
        Point2::new(
            self.pos.x + self.edge_size / 2.0,
            self.pos.y + self.edge_size / 2.0
        )
    }
}
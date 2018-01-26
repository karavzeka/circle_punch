use super::EDGE_SIZE;
use super::command::{SpikeCmd, Position, Line};

use json_struct::SpikeTemplate;
use cgmath::{Point2, Vector2, Zero, InnerSpace};

pub const SPIKE_HEIGHT:f32 = 16.0;
pub const NEEDLE_HALF_WIDTH:f32 = 8.0;

/// Use spike templates from config to produce spike models.
pub struct SpikeFabric {
}

impl SpikeFabric {
    pub fn create(template: SpikeTemplate) -> Spike {
        let normal = Vector2::new(template.normal.x, template.normal.y);

        let draw_point_1 = match (template.certain_point_1, template.tile_point_1) {
            // If a certain position is set, take it.
            (Some(pos), _) => Point2::new(pos.x, pos.y),
            // Else take a position defined as tile.
            (None, Some(pos)) => {
                let mut x = pos.x * EDGE_SIZE;
                if normal.x < 0.0 {
                    x += EDGE_SIZE;
                }
                let mut y = pos.y * EDGE_SIZE;
                if normal.y < 0.0 {
                    y += EDGE_SIZE;
                }
                Point2::new(x, y)
            },
            _ => {
                panic!("Spike point_1 is undefined")
            }
        };
        let draw_point_2 = match (template.certain_point_2, template.tile_point_2) {
            // If a certain position is set, take it.
            (Some(pos), _) => Point2::new(pos.x, pos.y),
            // Else take a position defined as tile.
            (None, Some(pos)) => {
                let mut x = pos.x * EDGE_SIZE;
                if normal.x < 0.0 {
                    x += EDGE_SIZE;
                }
                let mut y = pos.y * EDGE_SIZE;
                if normal.y < 0.0 {
                    y += EDGE_SIZE;
                }
                Point2::new(x, y)
            },
            _ => {
                panic!("Spike point_2 is undefined")
            }
        };


        let vec_along_spike: Vector2<f32> = (draw_point_2 - draw_point_1).normalize();
        if !vec_along_spike.is_perpendicular(normal) {
            panic!("Incorrect spike normal");
        }

        let danger_point_1 = draw_point_1 + vec_along_spike * NEEDLE_HALF_WIDTH + normal * SPIKE_HEIGHT;
        let danger_point_2 = draw_point_2 - vec_along_spike * NEEDLE_HALF_WIDTH + normal * SPIKE_HEIGHT;

        Spike {
            draw_body: SpikeBody {
                point_1: draw_point_1,
                point_2: draw_point_2,
            },
            danger_body: SpikeBody {
                point_1: danger_point_1,
                point_2: danger_point_2,
            },
            normal,
            vec_along_spike,
        }
    }
}

#[derive(Debug)]
/// Spike model
pub struct Spike {
    pub draw_body: SpikeBody,
    pub danger_body: SpikeBody,
    pub normal: Vector2<f32>,
    pub vec_along_spike: Vector2<f32>,
}

impl Spike {
    pub fn generate_cmd(&self) -> SpikeCmd {
        SpikeCmd {
            draw_body: Line {
                point_1: Position { x: self.draw_body.point_1.x, y: self.draw_body.point_1.y },
                point_2: Position { x: self.draw_body.point_2.x, y: self.draw_body.point_2.y },
            },
            danger_body: Line {
                point_1: Position { x: self.danger_body.point_1.x, y: self.danger_body.point_1.y },
                point_2: Position { x: self.danger_body.point_2.x, y: self.danger_body.point_2.y },
            },
            normal: Position {x: self.normal.x, y: self.normal.y},
            vec_along_spike: Position {x: self.vec_along_spike.x, y: self.vec_along_spike.y},
            height: SPIKE_HEIGHT,
            needle_half_width: NEEDLE_HALF_WIDTH,
        }
    }
}

#[derive(Debug)]
pub struct SpikeBody {
    pub point_1: Point2<f32>,
    pub point_2: Point2<f32>,
}
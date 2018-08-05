use super::Player;
use super::command::{WaveCmd};

use cgmath::{Point2};
use chrono::prelude::Utc;

const MAX_RADIUS: f32 = 75.0;

#[derive(Clone, Debug)]
pub struct Wave {
    id: i64,
    pub r: f32,
    pub pos: Point2<f32>,
    pub owner_id: String,
    pub is_dead: bool,
    /// List of player ids
    pub collided_with: Vec<String>
}

impl Wave {
    pub fn new(owner: &Player) -> Wave {
        let now = Utc::now();
        let id = now.timestamp() * 1_000_000 + now.timestamp_subsec_micros() as i64;
        Wave{
            id,
            r: owner.body.r,
            pos: owner.body.pos.clone(),
            owner_id: owner.id.clone(),
            is_dead: false,
            collided_with: Vec::with_capacity(2),
        }
    }

    pub fn update(&mut self) {
        self.r += 5.0;
        if self.r >= MAX_RADIUS {
            self.is_dead = true;
        }
    }

    pub fn generate_cmd(&self) -> WaveCmd {
        WaveCmd::new(self.id, self.pos.x, self.pos.y, self.r, self.is_dead)
    }
}
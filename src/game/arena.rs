use super::Player;
use super::player::PLAYER_RADIUS;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rand::{thread_rng, Rng};

pub struct Arena {
    width: i32,
    height: i32,
}

impl Arena {
    pub fn new(width: i32, height: i32) -> Arena {
        Arena {
            width: width,
            height: height,
        }
    }

    /// Generate random position to spawn
    pub fn get_spawn_pos(&self, players: &HashMap<String, Player>) -> (f32, f32) {
        let mut rng = thread_rng();
        let pos_x = rng.gen_range(0.0, 1.0) * (self.width as f32 - 4.0 * PLAYER_RADIUS) + 2.0 * PLAYER_RADIUS;
        let pos_y = rng.gen_range(0.0, 1.0) * (self.height as f32 - 4.0 * PLAYER_RADIUS) + 2.0 * PLAYER_RADIUS;
        //TODO check collision with another objects
        (pos_x, pos_y)
    }
}
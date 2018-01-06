use super::Player;
use super::player::DEFAULT_RADIUS as PLAYER_RADIUS;
use super::{Wall, EDGE_SIZE};
use super::command::{MapCmd, WallCmd};

use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use cgmath::Point2;

pub struct Arena {
    width: i32,
    height: i32,

    pub walls: Vec<Wall>
}

impl Arena {
    pub fn new(width: i32, height: i32) -> Arena {
        Arena {
            width,
            height,
            walls: Vec::new(),
        }
    }

    /// Generates random position to spawn
    pub fn get_spawn_pos(&self, players: &HashMap<String, Player>) -> (f32, f32) {
        let mut rng = thread_rng();
        let pos_x = rng.gen_range(0.0, 1.0) * (self.width as f32 - 4.0 * PLAYER_RADIUS) + 2.0 * PLAYER_RADIUS;
        let pos_y = rng.gen_range(0.0, 1.0) * (self.height as f32 - 4.0 * PLAYER_RADIUS) + 2.0 * PLAYER_RADIUS;
        //TODO check collision with another objects
        (pos_x, pos_y)
    }

    /// Loads map structure from file
    pub fn load_map(&mut self, path: &str) {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        for (row, line) in reader.lines().enumerate() {
            for (col, character) in line.unwrap().chars().enumerate() {
                if character.eq(&'1') {
                    let position = Point2::new(col as f32 * EDGE_SIZE, row as f32 * EDGE_SIZE);
                    self.walls.push(Wall::new(position));
                }
            }
        }
    }

    pub fn generate_map_cmd(&self) -> MapCmd {
        let mut map_cmd = MapCmd::new();
        for wall in self.walls.iter() {
            map_cmd.walls.push(WallCmd::new(wall.pos.x as f32, wall.pos.y as f32));
        }
        map_cmd
    }
}
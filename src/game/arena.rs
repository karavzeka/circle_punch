use super::Player;
use super::player::DEFAULT_RADIUS as PLAYER_RADIUS;
use super::{Wall, Floor, EDGE_SIZE};
use super::command::{MapCmd, WallCmd};

use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use cgmath::Point2;

pub struct Arena {
    width: u32,
    height: u32,

    pub walls: Vec<Wall>,
    pub floors: Vec<Floor>,
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            width: 0,
            height: 0,
            walls: Vec::new(),
            floors: Vec::new(),
        }
    }

    /// Generates random position to spawn
    pub fn get_spawn_pos(&self, players: &HashMap<String, Player>) -> (f32, f32) {
        let mut rng = thread_rng();
        let index = rng.gen_range(0, self.floors.len());
        let rand_point = self.floors.get(index).unwrap().get_center();
        //TODO check collision with another objects
        (rand_point.x, rand_point.y)
    }

    /// Loads map structure from file
    pub fn load_map(&mut self, path: &str) {
        let mut rows: u32 = 0;
        let mut cols: u32 = 0;

        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        for (row, line) in reader.lines().enumerate() {
            rows += 1;

            let string = line.unwrap();
            let numbers_in_line: Vec<&str> = string.trim().split(" ").collect();

            if row == 0 {
                cols = numbers_in_line.len() as u32;
            } else if numbers_in_line.len() as u32 != cols {
                panic!("Map error! Different count of cols in different rows.");
            }

            for (col, el) in numbers_in_line.iter().enumerate() {
                let position = Point2::new(col as f32 * EDGE_SIZE, row as f32 * EDGE_SIZE);

                let number: u32 = el.parse().unwrap();
                match number {
                    1 => {
                        self.walls.push(Wall::new(position));
                    }
                    _ => {
                        self.floors.push(Floor::new(position));
                    }
                }
            }
        }

        self.width = cols * EDGE_SIZE as u32;
        self.height = rows * EDGE_SIZE as u32;
    }

    pub fn generate_map_cmd(&self) -> MapCmd {
        let mut map_cmd = MapCmd::new(self.width, self.height);
        for wall in self.walls.iter() {
            map_cmd.walls.push(WallCmd::new(wall.pos.x as f32, wall.pos.y as f32));
        }
        map_cmd
    }
}
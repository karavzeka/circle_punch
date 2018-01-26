use super::Player;
use super::player::DEFAULT_RADIUS as PLAYER_RADIUS;
use super::{Wall, Floor, EDGE_SIZE};
use super::command::{MapCmd, WallCmd, SpikeCmd};
use super::{SpikeFabric, Spike};

use json_struct::MapScheme;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use rand::{thread_rng, Rng};
use cgmath::Point2;
use serde_json;

pub struct Arena {
    width: u32,
    height: u32,

    pub walls: Vec<Wall>,
    pub floors: Vec<Floor>,
    pub spikes: Vec<Spike>,
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            width: 0,
            height: 0,
            walls: Vec::new(),
            floors: Vec::new(),
            spikes: Vec::new(),
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
    pub fn load_map(&mut self, map_dir: &Path) {
        self.load_landscape(map_dir);
        self.load_map_objects(map_dir);
    }

    /// Loads landscape (walls, floor)
    fn load_landscape(&mut self, map_dir: &Path) {
        let landscape = map_dir.join("landscape.txt");

        let f = File::open(landscape).unwrap();
        let reader = BufReader::new(f);

        let mut cols: u32 = 0;
        let mut row = 0;
        for line in reader.lines() {
            let string = line.unwrap();
            if string == "" {
                continue;
            }

            let elements: Vec<&str> = string.trim().split(" ").collect();
            if elements.is_empty() || elements[0] == "//" || elements[0] == "#" {
                continue;
            }

            if row == 0 {
                // First row with data
                cols = elements.len() as u32;
            } else if elements.len() as u32 != cols {
                panic!("Map error! Different count of cols in different rows.");
            }

            for (col, el) in elements.iter().enumerate() {
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

            row += 1;
        }

        self.width = cols * EDGE_SIZE as u32;
        self.height = row * EDGE_SIZE as u32;
    }

    /// Loads map objects
    fn load_map_objects(&mut self, map_dir: &Path) {
        let objects_config = map_dir.join("objects.json");
        let f = File::open(objects_config).unwrap();

        let map_scheme: MapScheme = serde_json::from_reader(f).unwrap();

        for spike_template in map_scheme.spike_templates {
            let spike = SpikeFabric::create(spike_template);
            self.spikes.push(spike);
        }
    }

    pub fn generate_map_cmd(&self) -> MapCmd {
        let mut map_cmd = MapCmd::new(self.width, self.height);
        for wall in self.walls.iter() {
            map_cmd.walls.push(WallCmd::new(wall.pos.x as f32, wall.pos.y as f32));
        }
        for spike in self.spikes.iter() {
            let spike_cmd = spike.generate_cmd();
            map_cmd.spikes.push(spike_cmd);
        }
        map_cmd
    }
}
use super::Player;
use std::sync::{Arc, Mutex};

pub struct Arena {
    width: i32,
    height: i32,
//    pub players: Vec<Player>
    pub players: Arc<Mutex<Vec<Player>>>
}

impl Arena {
    pub fn new(width: i32, height: i32) -> Arena {
        Arena {
            width: width,
            height: height,
            players: Arc::new(Mutex::new(Vec::new())),
        }
    }

//    pub fn add_player(&mut self, player: Player) {
//        self.players.push(player);
//    }

    pub fn test_method(&mut self) -> i32 {
        67
    }
}
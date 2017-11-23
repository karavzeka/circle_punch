extern crate websocket;

use super::Arena;
use super::command::{PlayerCmd, MoveTo};

use std::net::TcpStream;

pub const PLAYER_RADIUS: f32 = 16.0;

pub struct Player {
    pub id: String,
//    pub command: Option<PlayerCmd>,
    pub sender: websocket::sender::Writer<TcpStream>,

    pub x: f32,
    pub y: f32,
    pub r: f32,
}

impl Player {
    pub fn new(id: String, sender: websocket::sender::Writer<TcpStream>) -> Player {
        Player {
            id: id,
//            command: None,
            sender: sender,
            x: 0.0,
            y: 0.0,
            r: PLAYER_RADIUS,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn generate_cmd(&self) -> PlayerCmd {
        PlayerCmd {
            player_id: self.id.clone(),
            it_is_you: false,
            move_vector: MoveTo {
                x: self.x,
                y: self.y,
            }
        }
    }
}
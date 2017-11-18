extern crate websocket;

use super::Arena;
use super::command::PlayerCmd;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};


pub struct Player {
    id: String,
    command: Option<PlayerCmd>,
    pub sender: websocket::sender::Writer<TcpStream>,

//    arena: &'a Arena,
    arena: Arc<Mutex<Option<Arena>>>,

    pub x: i32,
    y: i32,
}

impl Player {
    pub fn new(id: String, sender: websocket::sender::Writer<TcpStream>, arena: Arc<Mutex<Option<Arena>>>) -> Player {
//        let arena_inst = arena.lock().unwrap().as_mut().unwrap();
        let i = arena.lock().unwrap().as_mut().unwrap().test_method();
        let player = Player {
            id: id,
            command: None,
            sender: sender,
//            arena: Arc::new(Mutex::new(None)),
            arena: arena,
            x: i,
            y: 0,
        };
//        arena.lock().unwrap().as_mut().unwrap().players.lock().unwrap().push(player);
        player
    }
}
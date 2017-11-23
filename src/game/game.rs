extern crate websocket;
extern crate serde_json;

use super::{Arena, Player};

use GAME_CONF_PATH;
use config::Config;
use std::net::TcpStream;
use std::collections::HashMap;
use game::command::{CommandIn, CommandOut, MoveTo};
use chrono::prelude::Utc;
use websocket::{OwnedMessage};

pub struct Game {
    pub arena: Option<Arena>,
    pub players: HashMap<String, Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            arena: None,
            players: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        let config = Config::load(GAME_CONF_PATH);
        let arena = Arena::new(
            config.get_raw("width").unwrap().as_i64().unwrap() as i32,
            config.get_raw("height").unwrap().as_i64().unwrap() as i32
        );
        self.arena = Some(arena);
    }

    /// Create new player
    pub fn make_player(&mut self, player_id: String, sender: websocket::sender::Writer<TcpStream>) {
        let (spawn_x, spawn_y) = self.arena.as_ref().unwrap().get_spawn_pos(&self.players);

        let mut player = Player::new(player_id.clone(), sender);
        player.set_position(spawn_x, spawn_y);
        self.players.insert(player_id.clone(), player);
    }

    /// Remove palayer
    pub fn remove_player(&mut self, player_id: String) {
        self.players.remove(&player_id);
    }

    /// Process player command
    pub fn process_command(&mut self, cmd: CommandIn) {
        let mut player = self.players.get_mut(&cmd.player_id).unwrap();
        player.x += cmd.move_vector.x;
        player.y += cmd.move_vector.y;
    }

    /// Send game state to all players
    pub fn update(&mut self) {
        let now = Utc::now();
        let ts = (now.timestamp() * 1_000) as u64 + now.timestamp_subsec_millis() as u64;
        let mut cmd = CommandOut {
            time: ts,
            players: vec![]
        };

        for (_, player) in self.players.iter_mut() {
            let player_cmd = player.generate_cmd();
            cmd.players.push(player_cmd);

//            let json = serde_json::to_string(&cmd).unwrap();
//            player.sender.send_message(&OwnedMessage::Text(json)).unwrap();
        }

        for i in 0..cmd.players.len() {
            cmd.players[i].it_is_you = true;
            let player = self.players.get_mut(&cmd.players[i].player_id).unwrap();

            let json = serde_json::to_string(&cmd).unwrap();
            player.sender.send_message(&OwnedMessage::Text(json)).unwrap();

            cmd.players[i].it_is_you = false;
        }
    }
}
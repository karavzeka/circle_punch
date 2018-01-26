extern crate websocket;
extern crate serde_json;

use super::{Arena, Player};

use config::Config;
use std::net::TcpStream;
use std::collections::HashMap;
use std::path::Path;
use game::command::{ClientCmd, ServerCmd};
use controller::CollisionController;
use websocket::{OwnedMessage};

const MAP_CONF_DIR: &str = "config-rs/map";

pub struct Game {
    pub arena: Option<Arena>,
    pub players: HashMap<String, Player>,
    pub disconnected_players: Vec<String>,
    pub collision_controller: CollisionController,
}

impl Game {
    pub fn new() -> Game {
        Game {
            arena: None,
            players: HashMap::new(),
            disconnected_players: Vec::new(),
            collision_controller: CollisionController::new(),
        }
    }

    pub fn init(&mut self) {
        let mut arena = Arena::new();
        arena.load_map(Path::new(MAP_CONF_DIR));
        self.arena = Some(arena);
    }

    /// Create new player
    pub fn make_player(&mut self, player_id: String, sender: websocket::sender::Writer<TcpStream>) {
        let (spawn_x, spawn_y) = self.arena.as_ref().unwrap().get_spawn_pos(&self.players);

        let mut player = Player::new(player_id.clone(), sender);
        player.set_position(spawn_x, spawn_y);
        self.send_map(&mut player);
        self.players.insert(player_id.clone(), player);

    }

    /// Remove palayer
    pub fn remove_player(&mut self, player_id: String) {
        self.players.remove(&player_id);
        self.disconnected_players.push(player_id);
    }

    /// Process player command
    pub fn process_command(&mut self, cmd: ClientCmd) {
        let player = match self.players.get_mut(&cmd.player_id) {
            Some(player) => player,
            None => {
                panic!("Player '{}' not found in stack.", &cmd.player_id);
            }
        };

        player.set_move_to(cmd.move_vector.x, cmd.move_vector.y);
    }

    /// Send game state to all players
    pub fn update(&mut self, dt: f32) {
        let mut cmd = ServerCmd::new();
        cmd.disconnected_players = self.disconnected_players.clone();

        let player_ids: Vec<String> = self.players.keys().map(|key| key.clone()).collect();
        for (i, player_id_i) in player_ids.iter().enumerate() {
            // Updating player state
            {
                let player_i = self.players.get_mut(player_id_i).unwrap();
                player_i.update(dt);
            }
            // Finding player-player collisions
            {
                let player_i = self.players.get(player_id_i).unwrap();
                for (_, player_id_j) in player_ids.iter().skip(i + 1).enumerate() {
                    let player_j = self.players.get(player_id_j).unwrap();

                    self.collision_controller.detect_player_vs_player(player_i, player_j);
                }
                self.collision_controller.detect_player_vs_wall(player_i, self.arena.as_ref().unwrap().walls.as_ref());
                // Immutable borrow of player_i ends here
            }

            // Applying collisions
            let player_i = self.players.get_mut(player_id_i).unwrap();
            self.collision_controller.apply_for_player(player_i);

            // Generation player command
            let player_cmd = player_i.generate_cmd();
            cmd.players.push(player_cmd);
        }

        // Individual modification and sending command
        for i in 0..cmd.players.len() {
            cmd.players[i].it_is_you = true;
            let player = self.players.get_mut(&cmd.players[i].player_id).unwrap();

            let json = serde_json::to_string(&cmd).unwrap();
            player.sender.send_message(&OwnedMessage::Text(json)).unwrap();

            cmd.players[i].it_is_you = false;
        }
    }

    /// Send map state
    fn send_map(&self, player: &mut Player) {
        let arena = self.arena.as_ref().unwrap();
        let map_cmd = arena.generate_map_cmd();
        let json = serde_json::to_string(&map_cmd).unwrap();
        player.sender.send_message(&OwnedMessage::Text(json)).unwrap();
    }

    pub fn pong(&mut self, player_id: String, data: Vec<u8>) {
        let player = self.players.get_mut(&player_id).unwrap();
        player.sender.send_message(&OwnedMessage::Pong(data)).unwrap();
    }
}
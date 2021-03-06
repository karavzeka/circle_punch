extern crate websocket;
extern crate serde_json;

use super::{Arena, Player, Wave};

use std::net::TcpStream;
use std::collections::HashMap;
use std::path::Path;
use std::error::Error;
use game::command::{ClientCmd, ServerCmd, IncomingCmdType, ScoreCmd};
use controller::CollisionController;
use websocket::{OwnedMessage};

const MAP_CONF_DIR: &str = "config-rs/map";

pub struct Game {
    pub arena: Option<Arena>,
    pub guests: HashMap<String, Player>,
    pub players: HashMap<String, Player>,
    pub disconnected_players: Vec<String>,
    pub collision_controller: CollisionController,
    pub waves: Vec<Wave>,

    pub update_score: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            arena: None,
            guests: HashMap::new(),
            players: HashMap::new(),
            disconnected_players: Vec::new(),
            collision_controller: CollisionController::new(),
            waves: Vec::new(),
            update_score: false,
        }
    }

    pub fn init(&mut self) {
        let mut arena = Arena::new();
        arena.load_map(Path::new(MAP_CONF_DIR));
        self.arena = Some(arena);
    }

    /// Create new player
    pub fn make_player(&mut self, player_id: String, sender: websocket::sender::Writer<TcpStream>) -> Result<(), Box<Error>> {
        let mut player = Player::new(player_id.clone(), sender);
        self.send_map(&mut player)?;
        self.guests.insert(player_id.clone(), player);

        Ok(())
    }

    /// Respawn player
    fn respawn_player(&mut self, player_id: &str) {
        let (spawn_x, spawn_y) = self.arena.as_ref().unwrap().get_spawn_pos(&self.players);

        let player = self.players.get_mut(player_id).unwrap();
        player.set_position(spawn_x, spawn_y);
        player.body.velocity.x = 0.0;
        player.body.velocity.y = 0.0;
        player.body.acceleration.x = 0.0;
        player.body.acceleration.y = 0.0;
        player.health = 100.0;
        player.is_dead = false;
        player.is_health_changed = true;

        self.update_score = true;
    }

    /// Remove palayer
    pub fn remove_player(&mut self, player_id: String) {
        self.players.remove(&player_id);
        self.guests.remove(&player_id);
        self.disconnected_players.push(player_id);

        self.update_score = true;
    }

    /// Process player command
    pub fn process_command(&mut self, client_cmd: ClientCmd) {
        // Register command
        match client_cmd.cmd {
            IncomingCmdType::RegisterPlayer {ref nickname} => {
                for player in self.players.values() {
                    if player.nickname == *nickname {
                        self.guests.get_mut(&client_cmd.player_id).unwrap().bad_registration_answer();
                        return;
                    }
                }

                match self.guests.remove(&client_cmd.player_id) {
                    Some(mut player) => {
                        player.nickname = nickname.clone();
                        let (spawn_x, spawn_y) = self.arena.as_ref().unwrap().get_spawn_pos(&self.players);
                        player.set_position(spawn_x, spawn_y);
                        self.players.insert(client_cmd.player_id.clone(), player);

                        // To show health of all players for new player
                        for player in self.players.values_mut() {
                            player.is_health_changed = true;
                        }

                        self.update_score = true;
                    },
                    None => {
                        panic!("Player '{}' not found in guest stack.", &client_cmd.player_id);
                    }
                }
            }
            _ => ()
        }

        // Any other command
        let player = match self.players.get_mut(&client_cmd.player_id) {
            Some(player) => player,
            None => {
                panic!("Player '{}' not found in stack.", &client_cmd.player_id);
            }
        };

        match client_cmd.cmd {
            IncomingCmdType::Move {x, y} => player.set_move_to(x, y),
            IncomingCmdType::Attack => player.attack(),
            _ => ()
        }
    }

    /// Send game state to all players
    pub fn update(&mut self, dt: f32) -> Result<(), Box<Error>> {
        let mut cmd = ServerCmd::new();
        cmd.disconnected_players = self.disconnected_players.clone();

        let player_ids: Vec<String> = self.players.keys().map(|key| key.clone()).collect();
        for (i, player_id_i) in player_ids.iter().enumerate() {
            let mut is_dead = false;
            let mut update_score_for = None;
            // Updating player state
            {
                let player_i = self.players.get_mut(player_id_i).unwrap();
                if player_i.is_dead {
                    is_dead = player_i.is_dead;
                    update_score_for = player_i.last_hit_by.clone();
                }

                if player_i.attack {
                    self.waves.push(Wave::new(&player_i));
                }

                player_i.update(dt);
            }
            if is_dead {
                self.respawn_player(player_id_i);
            }
            // Update score for killer
            match update_score_for {
                Some(hit_player_id) => {
                    self.players.get_mut(&hit_player_id).unwrap().score += 1;
                },
                None => ()
            }
            // Finding player-player collisions
            {
                let player_i = self.players.get(player_id_i).unwrap();
                for (_, player_id_j) in player_ids.iter().skip(i + 1).enumerate() {
                    let player_j = self.players.get(player_id_j).unwrap();

                    self.collision_controller.detect_player_vs_player(player_i, player_j);
                }
                self.collision_controller.detect_player_vs_wall(player_i, self.arena.as_ref().unwrap().walls.as_ref());
                self.collision_controller.detect_player_vs_spike(player_i, self.arena.as_ref().unwrap().spikes.as_ref());
                // Immutable borrow of player_i ends here
            }
            {
                let mut player_i = self.players.get_mut(player_id_i).unwrap();
                self.collision_controller.detect_player_vs_wave(player_i, self.waves.as_mut());
            }

            // Applying collisions
            let player_i = self.players.get_mut(player_id_i).unwrap();
            self.collision_controller.apply_for_player(player_i);

            // Generation player command
            let player_cmd = player_i.generate_cmd();
            cmd.players.push(player_cmd);
        }

        // Waves update
        self.waves = self.waves.clone().into_iter()
            .map(|mut wave| {
                wave.update();
                cmd.waves.push(wave.generate_cmd());
                wave
            })
            .filter(|wave| !wave.is_dead)
            .collect();

        cmd.score_list = self.generate_score_list();

        // Individual modification and sending command
        for i in 0..cmd.players.len() {
            cmd.players[i].it_is_you = true;
            let player = self.players.get_mut(&cmd.players[i].player_id).unwrap();

            let json = serde_json::to_string(&cmd)?;
            player.sender.send_message(&OwnedMessage::Text(json))?;

            cmd.players[i].it_is_you = false;
        }

        self.update_score = false;

        Ok(())
    }

    /// Send map state
    fn send_map(&self, player: &mut Player) -> Result<(), Box<Error>> {
        let arena = self.arena.as_ref().unwrap();
        let map_cmd = arena.generate_map_cmd();
        let json = serde_json::to_string(&map_cmd)?;
        player.sender.send_message(&OwnedMessage::Text(json))?;

        Ok(())
    }

    pub fn pong(&mut self, player_id: String, data: Vec<u8>) {
        let player = self.players.get_mut(&player_id).unwrap();
        player.sender.send_message(&OwnedMessage::Pong(data)).unwrap();
    }

    pub fn generate_score_list(&self) -> Vec<ScoreCmd> {
        let mut score_list = Vec::new();
        if self.update_score {
            let mut players_vec: Vec<(&String, &Player)> = self.players.iter().collect();
            players_vec.sort_by(|a, b| b.1.score.cmp(&a.1.score));
            score_list = players_vec.iter().map(|cortege| {
                ScoreCmd::new(cortege.1.nickname.clone(), cortege.1.score)
            }).collect();
        }

        score_list
    }
}
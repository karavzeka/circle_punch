#[macro_use]
extern crate jconfig;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate websocket;
extern crate chrono;
extern crate rand;

mod game;
mod config;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::ops::DerefMut;
use websocket::{Message, OwnedMessage};
use websocket::sync::Server;
use chrono::prelude::Utc;
use rand::{thread_rng, Rng};

use game::{Game};
use game::command::{ClientCmd, ServerCmd, MoveVector};

const GAME_CONF_PATH: &str = "config-rs/game.json";
const FPS: u64 = 60u64;
const TICK_MS: u64 = (1000u64 / FPS);

fn main() {
    let mut game = Game::new();
    game.init();
    let mut game_glob = Arc::new(Mutex::new(game));

    let (command_tx, command_rx) = mpsc::channel();

    // Network processing
    let server = Server::bind("0.0.0.0:9002").unwrap();

    let now = Instant::now();

    // Send result commands to users
    let game_glob_copy = game_glob.clone();
    let handler = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(TICK_MS));
            game_glob_copy.lock().unwrap().update();
        }
    });

    // Process incoming commands
    let mut game_glob_copy = game_glob.clone();
    let handler = thread::spawn(move || {
        loop {
            let cmd: ClientCmd = command_rx.recv().unwrap();
            game_glob_copy.lock().unwrap().process_command(cmd);
//            println!("Recieved x: {}, y: {}", cmd.move_vector.x, cmd.move_vector.y);
        }
    });

    // Process user connections and incoming messages
    for connection in server.filter_map(Result::ok) {
        // Open new socket connection
        let client = connection.accept().unwrap();

        let player_id = format!("{}", client.peer_addr().unwrap());
        println!("Client is connected: {:?}", player_id);

        let (mut receiver, mut sender) = client.split().unwrap();

        // Creating new player
        game_glob.lock().unwrap().make_player(player_id.clone(), sender);

        // Listen connections and process incoming messages
        let game_glob_copy = game_glob.clone();
//        let commands_in_queue_copy = commands_in_queue.clone();
//        let queue_counter_copy = queue_counter.clone();
        let command_tx = command_tx.clone();
        thread::spawn(move || {
            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(message) => message,
                    Err(e) => {
                        println!("{:?}", e);
//                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                };

                match message {
                    OwnedMessage::Text(txt) => {
                        println!("Input message: {:?}", txt);
                        //TODO catch error
                        let mut cmd_in: ClientCmd = serde_json::from_str(&txt).unwrap();
                        cmd_in.player_id = player_id.clone();
//                        println!("{:?}", cmd_in);
                        command_tx.send(cmd_in);
                    }
                    OwnedMessage::Close(_) => {
                        println!("Client closed connection: {}", player_id);
                        game_glob_copy.lock().unwrap().remove_player(player_id.clone());
                        //TODO complete
//                        sender.send_message(&OwnedMessage::Close(None)).ok();
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        //TODO complete
//                        sender.send_message(&OwnedMessage::Pong(data)).unwrap();
                    }
                    _ => (),
                };
            }
        });
    }
}

#[macro_use]
extern crate jconfig;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate websocket;
extern crate chrono;
extern crate rand;
extern crate cgmath;

mod game;
mod config;
mod controller;
mod json_struct;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::{Duration};
use websocket::OwnedMessage;
use websocket::sync::Server;

use game::{Game};
use game::command::{ClientCmd, IncomingCmdType};

const FPS: u64 = 60u64;
const TICK_MS: u64 = (1000u64 / FPS);

fn main() {
    let mut game = Game::new();
    game.init();
    let game_glob = Arc::new(Mutex::new(game));

    let (command_tx, command_rx) = mpsc::channel();

    // Network processing
    let server = Server::bind("0.0.0.0:9002").unwrap();

    // Send result commands to users
    let game_glob_copy = game_glob.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(TICK_MS));
            game_glob_copy.lock().unwrap().update(TICK_MS as f32 / 1000.0);
        }
    });

    // Process incoming commands
    let game_glob_copy = game_glob.clone();
    thread::spawn(move || {
        loop {
            let cmd: ClientCmd = command_rx.recv().unwrap();
            game_glob_copy.lock().unwrap().process_command(cmd);
        }
    });

    // Process user connections and incoming messages
    for connection in server.filter_map(Result::ok) {
        // Open new socket connection
        let client = connection.accept().unwrap();

        let player_id = format!("{}", client.peer_addr().unwrap());
        println!("Client is connected: {:?}", player_id);

        let (mut receiver, sender) = client.split().unwrap();

        // Creating new player
        game_glob.lock().unwrap().make_player(player_id.clone(), sender);

        // Listen connections and process incoming messages
        let game_glob_copy = game_glob.clone();

        let command_tx = command_tx.clone();
        thread::spawn(move || {
            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(message) => message,
                    Err(e) => {
                        println!("Incoming message error: {:?}", e);
                        return;
                    }
                };

                match message {
                    OwnedMessage::Text(txt) => {
                        //TODO возможно надо все команды запихнуть в enum (https://serde.rs/enum-representations.html)
                        let mut cmd_in: IncomingCmdType;
                        let client_cmd: ClientCmd = match serde_json::from_str(&txt) {
                            Ok(cmd_in) => {
//                                println!("{:?}", cmd_in);
                                ClientCmd {player_id: player_id.clone(), cmd: cmd_in}
                            }
                            Err(e) => {
                                println!("Input message parse err: {:?}", e);
                                return;
                            }
                        };
                        command_tx.send(client_cmd).unwrap();
                    }
                    OwnedMessage::Close(_) => {
                        println!("Client closed connection: {}", player_id);
                        game_glob_copy.lock().unwrap().remove_player(player_id.clone());
                        //TODO complete
//                        sender.send_message(&OwnedMessage::Close(None)).ok();
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        game_glob_copy.lock().unwrap().pong(player_id.clone(), data);
                    }
                    _ => (),
                };
            }
        });
    }
}

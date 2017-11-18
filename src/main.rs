#[macro_use]
extern crate jconfig;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate websocket;
extern crate chrono;

mod game;
mod config;

use std::env;
use std::thread;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use websocket::{Message, OwnedMessage};
use websocket::sync::Server;

use game::{Game, Player};
use game::command::Command;

const GAME_CONF_PATH: &str = "config-rs/game.json";
const FPS: u64 = 60u64;
const TICK_MS: u64 = (1000u64 / FPS);

fn main() {
    let mut game = Game::new();
    game.init();

    let mut players = Arc::new(Mutex::new(Vec::new() as Vec<Player>));
    let arena = Arc::new(Mutex::new(game.arena));

//    let player = Player {};
//    let arena = Arc::new(game.get_arena().unwrap().players);
//    let mut hm: HashMap<String, websocket::sender::Writer<_>> = HashMap::new();
//    let sender_map = Arc::new(Mutex::new(hm));

    // Network processing
    let server = Server::bind("127.0.0.1:9002").unwrap();

    let now = Instant::now();

    let players_for_send = players.clone();

//    let mut cmd = Command::new();
//    let j = serde_json::to_string(&cmd).unwrap();
//    println!("{}", j);

    let handler = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(TICK_MS));
            let mut players = players_for_send.lock().unwrap();
            let cmd = Command::new();
//            cmd.add_player_cmd(Player)
            for player in players.iter_mut() {
                player.sender.send_message(&OwnedMessage::Text("zxczcx".to_string())).unwrap();
                println!("Unix ts: {} ms", cmd.time);
//                println!("Current time: {} + {}", now.elapsed().as_secs(), now.elapsed().subsec_nanos() / 1_000_000);
            }
        }
    });

    for connection in server.filter_map(Result::ok) {
//        let arena = arena.clone();
        let client = connection.accept().unwrap();

        let player_id = format!("{}", client.peer_addr().unwrap());
        println!("Client is connected: {:?}", format!("{}", player_id));

        let (mut receiver, mut sender) = client.split().unwrap();

        let mut player = Player::new(player_id.clone(), sender, arena.clone());
//        arena.lock().unwrap().as_mut().unwrap().players.lock().unwrap().push(player);
//        player.arena = arena.clone();
//        players.lock().unwrap().push(player);
        println!("x: {}", player.x);

        thread::spawn(move || {

            // Create new player and put his into arena
//            let a = arena.lock().unwrap().as_mut().unwrap().test_method();
//            println!("{}", a);

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
//                        sender.send_message(&OwnedMessage::Text(txt)).unwrap();
                    }
                    OwnedMessage::Binary(bin) => {
//                        sender.send_message(&OwnedMessage::Binary(bin)).unwrap();
                    }
                    OwnedMessage::Close(_) => {
                        println!("Client closed connection: {:?}", player_id);
//                        sender.send_message(&OwnedMessage::Close(None)).ok();
                        return;
                    }
                    OwnedMessage::Ping(data) => {
//                        sender.send_message(&OwnedMessage::Pong(data)).unwrap();
                    }
                    _ => (),
                };
            }

        });
    }
}

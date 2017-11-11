#[macro_use]
extern crate jconfig;
extern crate websocket;

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

const GAME_CONF_PATH: &str = "config-rs/game.json";
const FPS: u64 = 60u64;
const TICK_MS: u64 = (1000u64 / FPS);

fn main() {
    let mut game = Game::new();
    game.init();
//    let mut players = game.arena.unwrap().players;
//    let mut players = Arc::new(Mutex::new(players));
    let arena = Arc::new(Mutex::new(game.arena));

    let player = Player {};
//    let arena = Arc::new(game.get_arena().unwrap().players);
    let mut hm: HashMap<String, websocket::sender::Writer<_>> = HashMap::new();
    let sender_map = Arc::new(Mutex::new(hm));

    // Network processing
    let server = Server::bind("127.0.0.1:9002").unwrap();



    let now = Instant::now();
    let sm = sender_map.clone();

    let handler = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(TICK_MS));
            let mut map = sm.lock().unwrap();

            for (user_id, sender) in map.iter_mut() {
                sender.send_message(&OwnedMessage::Text("zxczcx".to_string())).unwrap();
            }


//            thread::sleep(Duration::from_millis(2000));
//            let mut map = sm.lock().unwrap();
//            for (user_id, sender) in map.iter_mut() {
//                sender.send_message(&OwnedMessage::Text(format!("Your id is '{}'", user_id))).unwrap();
//                println!("Current time: {} + {}. Sended to {}", now.elapsed().as_secs(), now.elapsed().subsec_nanos() / 1_000_000, user_id);
//            }
        }
    });

    for connection in server.filter_map(Result::ok) {
        let arena = arena.clone();
        let client = connection.accept().unwrap();
        let client_addr = client.peer_addr().unwrap();
        println!("Client is connected: {:?}", format!("{}", client_addr));
        let (mut receiver, mut sender) = client.split().unwrap();

        let player_id = format!("{}", client_addr);
        sender_map.lock().unwrap().insert(player_id, sender);

        thread::spawn(move || {

            // Create new player and put his into arena
            let a = arena.lock().unwrap().as_mut().unwrap().test_method();
            println!("{}", a);

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
                        println!("Client closed connection: {:?}", client_addr);
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

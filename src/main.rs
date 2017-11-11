extern crate websocket;

use std::thread;
use websocket::{Message, OwnedMessage};
use websocket::sync::Server;

fn main() {
    let game =

    // Network processing
    let server = Server::bind("127.0.0.1:9002").unwrap();

    for connection in server.filter_map(Result::ok) {
        thread::spawn(move || {
            let client = connection.accept().unwrap();
            let client_addr = client.peer_addr().unwrap();
            println!("Client is connected: {:?}", client_addr);

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(message) => message,
                    Err(e) => {
                        println!("{:?}", e);
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                };

                match message {
                    OwnedMessage::Text(txt) => {
                        sender.send_message(&OwnedMessage::Text(txt)).unwrap();
                    }
                    OwnedMessage::Binary(bin) => {
                        sender.send_message(&OwnedMessage::Binary(bin)).unwrap();
                    }
                    OwnedMessage::Close(_) => {
                        println!("Client closed connection: {:?}", client_addr);
                        sender.send_message(&OwnedMessage::Close(None)).ok();
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        sender.send_message(&OwnedMessage::Pong(data)).unwrap();
                    }
                    _ => (),
                };
            }
        });
    }
}

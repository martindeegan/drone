extern crate websocket;

use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::string::String;

use websocket::Message;
use websocket::sync::Server;

const DEBUG_PORT: &str = "0.0.0.0:27070";

#[derive(Debug)]
pub struct DebugInfo {
    pub time: i64,
    pub power: f32,
    pub p: f32,
    pub i: f32,
    pub d: f32
}

pub enum Signal {
    Log(DebugInfo),
    Stop
}

pub fn init_debug_port() -> Sender<Signal> {
    let (tx, rx): (Sender<Signal>, Receiver<Signal>) = channel();

    thread::spawn(move || {
        let mut server = Server::bind(DEBUG_PORT).unwrap();
        loop {
            println!("Debug port waiting for a connection");
            match server.accept() {
                Ok(upgrade) => {
                    let mut client = upgrade.use_protocol("rust-websocket").accept().unwrap();

                    let ip = client.peer_addr().unwrap();
                    println!("Debug connection from {}", ip);

                    loop {
                        match rx.recv() {
                            Ok(Signal::Log(debug_info)) => {
                                let msg_str: String = format!("{},{},{},{},{}", debug_info.time, debug_info.power, debug_info.p, debug_info.i, debug_info.d);
                                client.send_message(&Message::text(msg_str.as_ref()));
                            },
                            Ok(Signal::Stop) => {
                                client.shutdown();
                                break;
                            }
                            _ => {}
                        }
                    }
                },
                _ => { println!("Bad connection."); }
            };
        }

    });
    tx.clone()
}
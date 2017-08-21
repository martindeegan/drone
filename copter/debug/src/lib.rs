extern crate websocket;

use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::string::String;

use websocket::Message;
use websocket::sync::Server;
use websocket::sync::Client;
use websocket::stream::sync::TcpStream;
use websocket::server::NoTlsAcceptor;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::result::Result;


#[derive(Debug, Serialize, Deserialize)]
pub struct Axis {
    pub power: f32,
    pub p: f32,
    pub i: f32,
    pub d: f32,
    pub power_y: f32,
    pub p_y: f32,
    pub i_y: f32,
    pub d_y: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugInfo {
    pub time: i64,
    pub power: f32,
    pub pidaxes: Axis
}

pub enum Signal {
    Log(DebugInfo),
    Stop
}

const DEBUG_PROTOCOL: &str = "drone-debug";

fn shutdown_port(client: &mut Client<TcpStream>) -> Result<(),()> {
    match client.shutdown() {
        Ok(()) => {
            println!("[Debug]: Debug client successfully shutdown.");
            Ok(())
        },
        Err(e) => {
            println!("[Debug]: Error shutting down debug client: {:?}", e);
            Err(())
        }
    }
}

fn start_port(server: &mut Server<NoTlsAcceptor>) -> Result<Client<TcpStream>,()> {
    println!("[Debug]: Debug port waiting for a connection.");
    'search: loop {
        match server.accept() {
            Ok(upgrade) => {
                if !upgrade.protocols().contains(&String::from(DEBUG_PROTOCOL)) {
                    continue 'search;
                }
                match upgrade.use_protocol(DEBUG_PROTOCOL).accept() {
                    Ok(client) => {
                        let ip = client.peer_addr().unwrap();
                        println!("[Debug]: Debug connection from {}", ip);
                        return Ok(client);
                    },
                    _ => { }
                }
            },
            _ => { }
        }
    }
}

pub fn init_debug_port(port : i32) -> Sender<Signal> {
    let (tx, rx): (Sender<Signal>, Receiver<Signal>) = channel();

    thread::Builder::new().name("Debug Port".to_string()).spawn(move || {
        let mut server = Server::bind(format!("0.0.0.0:{}", port)).unwrap();
        let mut client: Client<TcpStream> = start_port(&mut server).unwrap();

        //Debug loop
        loop {
            match rx.recv() {
                Ok(Signal::Log(debug_info)) => {
                    let msg_str = serde_json::to_string(&debug_info).unwrap();

                    match client.send_message(&Message::text(msg_str.as_ref())) {
                        Ok(()) => {},
                        _ => {
                            match start_port(&mut server) {
                                Ok(c) => {
                                    client = c;
                                },
                                Err(()) => {}
                            }
                        }
                    }
                },
                Ok(Signal::Stop) => {
                    shutdown_port(&mut client).unwrap();
                    break;
                }
                _ => {}
            }
        }
    });
    tx.clone()
}
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

// Moved

//
// fn stat(values: [f32;40]) -> (f32, f32) {
//     let mut average = 0.0;
//     for i in 0..40 {
//         average += values[i];
//     }
//     average /= 40.0;
//
//     let mut std = 0.0;
//     for i in 0..40 {
//         std += (values[i] - average).powi(2);
//     }
//     std /= 40.0;
//
//     (average, std)
// }
//
// struct Log {
//     pub t: i64,
//     pub m1: u32,
//     pub m2: u32,
//     pub m3: u32,
//     pub m4: u32,
//     pub x_ang: f32,
//     pub y_ang: f32,
//     pub z_ang: f32,
//     pub x_p: f32,
//     pub x_i: f32,
//     pub x_d: f32,
//     pub y_p: f32,
//     pub y_i: f32,
//     pub y_d: f32
// }
//
// struct Logger {}
//
// impl Logger {
//     pub fn new(on: bool) -> Sender<Log> {
//         let (tx,rx): (Sender<Log>,Receiver<Log>) = channel();
//         if on {
//             thread::spawn(move || {
//                 let log_file_name = format!("logs/{}_{}_{}_{}_{}_{}_data.csv",
//                                             time::now().tm_year, time::now().tm_mon, time::now().tm_yday,
//                                             time::now().tm_hour, time::now().tm_min, time::now().tm_sec);
//                 let mut log_file = OpenOptions::new()
//                     .read(true)
//                     .write(true)
//                     .create(true)
//                     .open(log_file_name).expect("Couldn't open or create new file");
//                 loop {
//                     match rx.try_recv() {
//                         Ok(log) => {
//                             let out = format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n", log.t,
//                                               log.m1, log.m2, log.m3, log.m4,
//                                               format!("{:.*}", 2, log.x_ang), format!("{:.*}", 2, log.y_ang), format!("{:.*}", 2, log.z_ang),
//                                               format!("{:.*}", 2, log.x_p), format!("{:.*}", 2, log.x_i), format!("{:.*}", 2, log.x_d),
//                                               format!("{:.*}", 2, log.y_p), format!("{:.*}", 2, log.y_i), format!("{:.*}", 2, log.y_d));
//                             log_file.write_all(out.as_bytes());
//                         },
//                         Err(e) => {}
//                     }
//                 }
//             });
//         }
//         tx
//     }
// }

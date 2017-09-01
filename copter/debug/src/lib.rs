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
extern crate time;

use std::result::Result;
use std::fs::{OpenOptions};
use std::io::Write;

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
    pub t: f32,
    pub m1: u32,
    pub m2: u32,
    pub m3: u32,
    pub m4: u32,
    pub x_ang: f32,
    pub y_ang: f32,
    pub z_ang: f32,
    pub x_p: f32,
    pub x_i: f32,
    pub x_d: f32,
    pub y_p: f32,
    pub y_i: f32,
    pub y_d: f32,
    pub x_ang_rate: f32,
    pub y_ang_rate: f32,
    pub z_ang_rate: f32,
    pub x_accel: f32,
    pub y_accel: f32,
    pub z_accel: f32,
    pub x_mag: f32,
    pub y_mag: f32,
    pub z_mag: f32,
}

pub enum Signal {
    Log(DebugInfo),
    LogString(String),
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

pub struct Logger {}

impl Logger {
    //Return PID channel, Motor_channel, IMU channel, time channel
    pub fn new(on: bool) -> Sender<Signal> {
        let (tx,rx): (Sender<Signal>,Receiver<Signal>) = channel();
        if on {
            thread::spawn(move || {
                let log_file_name = format!("logs/{}_{}_{}_{}_{}_{}_data.csv",
                                            time::now().tm_year, time::now().tm_mon, time::now().tm_yday,
                                            time::now().tm_hour, time::now().tm_min, time::now().tm_sec);
                let mut log_file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(log_file_name).expect("Couldn't open or create new file");
                'logloop: loop {
                    match rx.try_recv() {
                        Ok(debug_info) => {
                            match debug_info {
                                Signal::Log(log) => {
                                    let out = format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n", format!("{:.*}", 4, log.t),
                                                      log.m1, log.m2, log.m3, log.m4,
                                                      format!("{:.*}", 2, log.x_ang), format!("{:.*}", 2, log.y_ang), format!("{:.*}", 2, log.z_ang),
                                                      format!("{:.*}", 2, log.x_p), format!("{:.*}", 2, log.x_i), format!("{:.*}", 2, log.x_d),
                                                      format!("{:.*}", 2, log.y_p), format!("{:.*}", 2, log.y_i), format!("{:.*}", 2, log.y_d),
                                                      format!("{:.*}", 2, log.x_ang_rate), format!("{:.*}", 2, log.y_ang_rate), format!("{:.*}", 2, log.z_ang_rate),
                                                      format!("{:.*}", 2, log.x_accel), format!("{:.*}", 2, log.y_accel), format!("{:.*}", 2, log.z_accel),
                                                      format!("{:.*}", 2, log.x_mag), format!("{:.*}", 2, log.y_mag), format!("{:.*}", 2, log.z_mag));
                                    log_file.write_all(out.as_bytes());
                                },
                                Signal::LogString(s) => {
                                    log_file.write_all(s.as_bytes());
                                },
                                Signal::Stop => {
                                    break 'logloop;
                                }   
                            }
                        },
                        Err(e) => {}
                    }
                }
            });
        }
        tx
    }
}

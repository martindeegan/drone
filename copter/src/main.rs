//Drone project
extern crate time;

extern crate rust_pigpio;
extern crate sensors;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate debug_server;
extern crate protobuf;
extern crate protos;

extern crate ansi_term;

use std::io::stdin;
use std::string::String;
use std::thread;

mod motor;
use motor::MotorManager;

mod connection;

mod config;
use config::Config;

use connection::Peer;

use ansi_term::Colour::*;

fn main() {
    println!("{}", Green.paint("[Input]: Press enter to start motors or type 'calibrate' to calibrate."));

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    match input.trim() {
        "calibrate" => {
            motor::calibrate();
        }
        _ => {
            start();
        }
    }

}

fn start() {
    let config = Config::new();

    let debug_pipe = debug_server::init_debug_port(config.debug_websocket_port);
    let mut motor_manager = MotorManager::new();
    for motor in config.motors.clone() {
        motor_manager.new_motor(motor);
    }

    let mut peer = Peer::new();

    println!("{}", Green.paint("[Input]: Press enter to self control."));
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    motor_manager.start_pid_loop(config, &mut peer, debug_pipe.clone());
    let clone = debug_pipe.clone();
    thread::spawn(move || {
        peer.connect_to_server();
        peer.start_connection_loop(clone);
    });

    println!("{}", Green.paint("[Input]: Press enter to stop."));

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    match input {
        _ => {
            motor::terminate_all_motors(debug_pipe.clone());
            std::process::exit(0);
        }
    }
}



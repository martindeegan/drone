extern crate time;

extern crate rust_pigpio;
extern crate sensors;

extern crate debug_server;
extern crate protobuf;
extern crate protos;
extern crate config;

extern crate ansi_term;

use std::io::stdin;
use std::string::String;
use std::thread;

mod motor;
use motor::MotorManager;

mod connection;

use config::Config;

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

    let stream = connection::connect_via_server(debug_pipe.clone());

    println!("{}", Green.paint("[Input]: Press enter to self control."));
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    motor_manager.start_pid_loop(config, stream, debug_pipe.clone());

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



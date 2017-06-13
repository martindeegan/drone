extern crate rust_pigpio;
extern crate protobuf;
extern crate time;
extern crate sensors;
extern crate simple_signal;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::stdin;
use std::string::String;

mod motor;
use motor::MotorManager;

mod proto {
    pub mod position;
}

use protobuf::Message;
use proto::position::Position;

mod connection;
use connection::Peer;

mod config;
use config::Config;

fn main() {

    let config = Config::new();

    simple_signal::set_handler(&[simple_signal::Signal::Int], |signals| {
        println!("Ctrl C!");
        motor::TERMINATE_ALL_MOTORS();
        std::process::exit(0);
    });

//    let mut peer = Peer::new();
    let mut manager = MotorManager::new();
    manager.new_motor(config.motors[0]);
    manager.new_motor(config.motors[1]);
    manager.new_motor(config.motors[2]);
    manager.new_motor(config.motors[3]);

    println!("Press enter to self control.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

//    manager.arm();
    manager.start_pid_loop(config);

    'input: loop {
        println!("Enter 'stop'");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error");

        match input.trim() {
            "stop" => {
                motor::TERMINATE_ALL_MOTORS();
                std::process::exit(0);
                break 'input;
            },
            _ => {
                println!("unrecognized input...");
                motor::TERMINATE_ALL_MOTORS();
                std::process::exit(0);
                break 'input;
            }
        }
    }
}

const FLOATING_POWER : u32 = 1100;
const MAX_POWER : u32 = 1400;


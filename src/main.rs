extern crate rust_pigpio;
extern crate protobuf;
extern crate time;
extern crate sensors;
extern crate simple_signal;

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

const MOTOR_1 : u32 = 18;
const MOTOR_2 : u32 = 19;
const MOTOR_3 : u32 = 20;
const MOTOR_4 : u32 = 21;

fn main() {

    simple_signal::set_handler(&[simple_signal::Signal::Int], |signals| {
        println!("Ctrl C!");
        motor::TERMINATE_ALL_MOTORS();
        std::process::exit(0);
    });

//    let mut peer = Peer::new();
    let mut manager = MotorManager::new();
    manager.new_motor(MOTOR_1);
    manager.new_motor(MOTOR_2);
    manager.new_motor(MOTOR_3);
    manager.new_motor(MOTOR_4);

    println!("Press enter to self control.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    manager.arm();
    manager.start_pid_loop();

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
                motor::TERMINATE_ALL_MOTORS();
                std::process::exit(0);
                break 'input;
            }
        }
    }
}

const FLOATING_POWER : u32 = 1100;
const MAX_POWER : u32 = 1400;


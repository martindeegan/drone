#[cfg(rpi)]
extern crate rust_pigpio;
extern crate protobuf;
extern crate time;
#[cfg(rpi)]
extern crate sensors;
extern crate time;
extern crate simple_signal;

use std::io::stdin;
use std::string::String;

#[cfg(rpi)] //Add to .bashrc: export RUST_PI_COMPILATION="rpi"
mod motor;
#[cfg(rpi)]
use motor::MotorManager;
#[cfg(rpi)]
use motor::Motor;

mod proto {
    pub mod position;
}

use protobuf::Message;
use proto::position::Position;

mod connection;
use connection::Peer;

#[cfg(rpi)]
const MOTOR_1 : u32 = 18;
#[cfg(rpi)]
const MOTOR_2 : u32 = 19;
#[cfg(rpi)]
const MOTOR_3 : u32 = 20;
#[cfg(rpi)]
const MOTOR_4 : u32 = 21;

fn main() {

    simple_signal::set_handler(&[simple_signal::Signal::Int], |signals| {
        println!("Ctrl C!");
        motor::TERMINATE_ALL_MOTORS();
        std::process::exit(0);
    });

    balance();

}

const FLOATING_POWER : u32 = 1100;
const MAX_POWER : u32 = 1400;

fn balance() {
    let mut manager = MotorManager::new();
    manager.new_motor(MOTOR_1);
    manager.new_motor(MOTOR_2);
    manager.new_motor(MOTOR_3);
    manager.new_motor(MOTOR_4);

    println!("Press enter to self control.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    manager.arm();
//    manager.start_pid_loop();
    
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


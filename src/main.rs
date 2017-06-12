#[cfg(rpi)]
extern crate rust_pigpio;
extern crate protobuf;
extern crate time;
#[cfg(rpi)]
extern crate sensors;
extern crate time;

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

    balance();
//    let mut manager = MotorManager::new();
//    manager.new_motor(MOTOR_1);
//    manager.new_motor(MOTOR_2);
//    manager.new_motor(MOTOR_3);
//    manager.new_motor(MOTOR_4);
//
//    println!("arm, balance, or calibrate");
//    let mut input = String::new();
//    stdin().read_line(&mut input).expect("Error");
//
//    match input.trim() {
//        "arm" => {
//            manager.arm();
//        },
//        "calibrate" => {
//            manager.calibrate();
//        },
//        "balance" => {
//            balance(manager);
//        }
//        _ => { }
//    }
//
//    'input: loop {
//        println!("Enter power between 1000-2000 or 'stop'");
//        let mut input = String::new();
//        stdin().read_line(&mut input).expect("Error");
//
//        match input.trim() {
//            "stop" => {
//                manager.terminate();
//                break 'input
//            },
//            _ => {
//                let x: u32 = input.trim().parse().unwrap_or(1100);
//
//                manager.set_power(0, x);
//                manager.set_power(1, x);
//                manager.set_power(2, x);
//                manager.set_power(3 , x);
//            }
//        }
//    }
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
    manager.start_pid_loop();

    'input: loop {
        println!("Enter 'stop'");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error");

        match input.trim() {
            "stop" => {
                motor::TERMINATE_ALL_MOTORS();
                std::process::exit(0);
                break 'input
            },
            _ => {}
        }
    }
}


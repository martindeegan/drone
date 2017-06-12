extern crate sensors;
extern crate time;

use std::io::stdin;
use std::string::String;

mod motor;
use motor::MotorManager;

const MOTOR_1 : u32 = 19;
const MOTOR_2 : u32 = 20;
const MOTOR_3 : u32 = 21;
const MOTOR_4 : u32 = 26;

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
    };
}
extern crate sensors;

use std::io::stdin;
use std::string::String;

mod motor;

use motor::MotorManager;



const MOTOR_1 : u32 = 19;
const MOTOR_2 : u32 = 20;
const MOTOR_3 : u32 = 21;
const MOTOR_4 : u32 = 26;

fn main() {
    let mut manager = MotorManager::new();
    manager.new_motor(MOTOR_1);
    manager.new_motor(MOTOR_2);
    manager.new_motor(MOTOR_3);
    manager.new_motor(MOTOR_4);

    println!("arm, balance, or calibrate");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    match input.trim() {
        "arm" => {
            manager.arm();
        },
        "calibrate" => {
            manager.calibrate();
        },
        "balance" => {
            balance(manager);
        }
        _ => { }
    }
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


fn balance(mut manager: MotorManager) {
    manager.arm();
    let thread = std::thread::spawn(move || {
        let errors_channel = sensors::start().unwrap();
        manager.set_power(0, FLOATING_POWER);
        manager.set_power(1, FLOATING_POWER);
        manager.set_power(2, FLOATING_POWER);
        manager.set_power(3 , FLOATING_POWER);
        loop {
            match errors_channel.recv() {
                Ok(components) => {
                    if (components.y.abs() > 25.0) {
                        manager.terminate();
                    }
                    // - 2,3 go up
                    // + 0, 1 go up
                    let power = FLOATING_POWER + ((MAX_POWER - FLOATING_POWER) as f32 * (components.y.abs() / 45.0)) as u32;
                    if (components.y < 0.0) {
                        manager.set_power(0, FLOATING_POWER);
                        manager.set_power(1, power);
                        manager.set_power(2, power);
                        manager.set_power(3, FLOATING_POWER);
                    }
                    if (components.y > 0.0) {
                        manager.set_power(0, power);
                        manager.set_power(1, FLOATING_POWER);
                        manager.set_power(2, FLOATING_POWER);
                        manager.set_power(3, power);
                    }
                }
                Err(_) => {
                    println!("Channel closed...");
                    break;
                }
            }
        }
    });

    let mut manager2 = MotorManager::new();
    manager2.new_motor(MOTOR_1);
    manager2.new_motor(MOTOR_2);
    manager2.new_motor(MOTOR_3);
    manager2.new_motor(MOTOR_4);
    'input: loop {
        println!("Enter 'stop'");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error");

        match input.trim() {
            "stop" => {
                manager2.terminate();
                std::process::exit(0);
                break 'input
            },
            _ => {}
        }
    };
}
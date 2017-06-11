#[cfg(rpi)]
extern crate rust_pigpio;
extern crate protobuf;
extern crate time;
#[cfg(rpi)]
extern crate sensors;

use std::io::stdin;
use std::string::String;

//use std::io::stdin;
//use std::string::String;

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
    let peer = Peer::new();
    peer.connect_to_server();

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
        _ => {}
    }
}

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
    }
}


extern crate protobuf;

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
use connection::Connection;


#[cfg(rpi)]
const MOTOR_1 : u32 = 18;
#[cfg(rpi)]
const MOTOR_2 : u32 = 19;
#[cfg(rpi)]
const MOTOR_3 : u32 = 20;
#[cfg(rpi)]
const MOTOR_4 : u32 = 21;

fn main() {
    let mut position = Position::new();
    position.set_time(1000);
    position.set_x(1);
    position.set_y(2323);
    position.set_z(3434);
    let bytes = position.write_to_bytes().unwrap();
    println!("{:?}", bytes);

    let read_pos: Position = protobuf::parse_from_bytes(bytes.as_ref()).unwrap();

    println!("Deserialized: time: {}, x: {}, y: {}, z: {}", read_pos.time, read_pos.x, read_pos.y, read_pos.z);

    let conn = Connection::new();
    conn.connect_to_server();

//    let mut manager = MotorManager::new();
//    manager.new_motor(MOTOR_1);
//    manager.new_motor(MOTOR_2);
//    manager.new_motor(MOTOR_3);
//    manager.new_motor(MOTOR_4);
//
//    'question: loop {
//        println!("arm or calibrate");
//        let mut input = String::new();
//        stdin().read_line(&mut input).expect("Error");
//
//        match input.trim() {
//            "arm" => {
//                manager.arm();
//                break 'question;
//            },
//            "calibrate" => {
//                manager.calibrate();
//            },
//            _ => { }
//        }
//    }
//
//    'input: loop {
//        println!("Enter power between 1000-2000 or 'stop'");
//        let mut input = String::new();
//        stdin().read_line(&mut input).expect("Error");
//
//        match input.trim() {
//            "stop" => break 'input,
//            _ => {
//                let x: u32 = input.trim().parse().unwrap_or(1300);
//
//                manager.set_power(0, x);
//                manager.set_power(1, x);
//                manager.set_power(2, x);
//                manager.set_power(3, x);
//            }
//        }
//    }
}


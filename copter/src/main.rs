extern crate time;
extern crate sensors;
extern crate debug_server;
extern crate protobuf;
extern crate protos;
extern crate config;
extern crate ansi_term;

use config::Config;

mod hardware;
// mod flight;
mod connection;

use hardware::gps::get_gps;

// use flight::{FlightMode,start_flight};
// use hardware::motors::MotorManager;

use std::io::stdin;
use std::string::String;
use std::thread;

// use sensor_manager::{start_sensors, calibrate_sensors};

use ansi_term::Colour::*;

fn main() {
    println!("{}", Green.paint("[Input]: Press enter to start motors or type 'calibrate' to calibrate or 'calibrate sensors'."));

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    match input.trim() {
        "calibrate" => {
            hardware::motors::calibrate();
        },
        "calibrate sensors" => {
            println!("Place drone on a completely level surface. Then press enter.");
            input = String::new();
            stdin().read_line(&mut input).expect("Error");

            // calibrate_sensors();
        }
        _ => {
            start();
        }
    }

}

fn start() {

    let mut gps = get_gps();
    for i in 0..2000 {
        match gps.try_recv() {
            Ok(data) => {
                println!("{:?}", data);
            },
            Err(e) => {}
        }
        thread::sleep_ms(10);
    }

    let ()
    // let motor_manager = MotorManager::new();

    //
    // let debug_pipe = debug_server::init_debug_port(config.debug_websocket_port);
    // let mut motor_manager = MotorManager::new();
    // for motor in config.motors.clone() {
    //     motor_manager.new_motor(motor);
    // }
    //
    // let stream = connection::connect_via_server(debug_pipe.clone());
    //
    // println!("{}", Green.paint("[Input]: Press enter to self control."));
    // let mut input = String::new();
    // stdin().read_line(&mut input).expect("Error");
    // let (mut orientation_rx, mut location_rx) = start_sensors();
    // motor_manager.start_pid_loop(config, stream, orientation_rx, debug_pipe.clone());
    //
    // println!("{}", Green.paint("[Input]: Press enter to stop."));
    //
    // let mut input = String::new();
    // stdin().read_line(&mut input).expect("Error");
    // match input {
    //     _ => {
    //         motor::terminate_all_motors(debug_pipe.clone());
    //         std::process::exit(0);
    //     }
    // }
}

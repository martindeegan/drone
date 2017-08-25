extern crate time;
extern crate sensors;
extern crate debug_server;
extern crate protobuf;
extern crate protos;
extern crate config;
extern crate ansi_term;

use config::Config;

mod hardware;
mod flight;
mod connection;

use flight::{FlightMode,start_flight};
use hardware::sensors::calibrate_sensors;

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

            calibrate_sensors();
        }
        _ => {
            start();
        }
    }

}

fn start() {

    let flight_mode_controller = start_flight();
    println!("{}", Green.paint("[Input]: Press enter to start the flight."));
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    flight_mode_controller.send(FlightMode::TakeOff);

    println!("{}", Green.paint("[Input]: Press enter to stop the flight."));
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    flight_mode_controller.send(FlightMode::Landing);

    println!("{}", Green.paint("[Input]: Press enter to shutdown."));
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
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

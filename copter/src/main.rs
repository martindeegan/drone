extern crate time;
extern crate debug_server;
extern crate config;
extern crate ansi_term;
extern crate nalgebra as na;
extern crate typenum;

use config::Config;

mod hardware;
mod flight;
// mod networking;

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
        "run_motors" => {

        },
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

    let (flight_mode_controller, control_thread_handler) = start_flight();

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

    flight_mode_controller.send(FlightMode::Shutdown);
    control_thread_handler.join();
}

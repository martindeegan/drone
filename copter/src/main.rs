extern crate time;
extern crate debug_server;
extern crate typenum;
extern crate logger;
extern crate configurations;
#[macro_use]
extern crate rulinalg;

use logger::ModuleLogger;

use configurations::Config;

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

fn main() {
    println!("Enter one of the following options:")
    println!("1: Fly");
    println!("2: Calibrate");

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
    let logger = ModuleLogger::new("Input", None);
    let (flight_mode_controller, control_thread_handler) = start_flight();

    logger.log("Press enter to start the flight.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    flight_mode_controller.send(FlightMode::TakeOff);

    logger.log("Press enter to stop the flight.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    flight_mode_controller.send(FlightMode::Landing);

    logger.log("Press enter to shutdown.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    flight_mode_controller.send(FlightMode::Shutdown);
    control_thread_handler.join();
}

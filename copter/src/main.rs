#![feature(custom_attribute)]
#![feature(const_fn)]
// Common Imports
extern crate configurations;
extern crate debug_server;
extern crate logger;
extern crate time;

// Hardware
extern crate ads111x;
extern crate i2cdev;
extern crate i2cdev_bmp180;
extern crate i2cdev_bmp280;
extern crate i2cdev_l3gd20;
extern crate i2cdev_lsm303d;
extern crate i2cdev_lsm9ds0;
extern crate i2csensors;
extern crate pca9685;
extern crate unbounded_gpsd;
extern crate wifilocation;
// extern crate rust_pigpio;

// Math Imports
extern crate alga;
extern crate nalgebra as na;
extern crate num;
extern crate typenum;

use std::io;
use std::io::Read;
use std::process::exit;

use na::{Matrix3, MatrixN, Vector3};
use na::U20;

type Matrix100 = MatrixN<f32, U20>;

use logger::ModuleLogger;

use configurations::Calibrations;

mod hardware;
mod flight;

use hardware::{initialize_hardware, MotorCommand};
use flight::start_flight_controller;

pub type PredictionReading = hardware::PredictionReading;
pub type UpdateReading = hardware::UpdateReading;



// mod networking;

pub enum Control {
    Terminate,
}

fn main() {
    let logger = ModuleLogger::new("Main", None);

    logger.log("Enter: Start flight.");
    logger.log("sensors.");
    logger.log("motors.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().as_ref() {
        "sensors" => {
            hardware::calibrate_sensors();
        }
        "motors" => {
            hardware::calibrate_motors();
        }
        _ => {
            start_flight();
        }
    };
}

fn start_flight() {
    let logger = ModuleLogger::new("Main", None);

    let (hardware_join_handle, pred_rx, update_rx, motor_tx, hardware_control_tx) =
        initialize_hardware();
    start_flight_controller(pred_rx, update_rx, motor_tx);


    logger.log("Press enter to terminate.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    hardware_control_tx.send(()).unwrap();

    hardware_join_handle.join().unwrap();
}

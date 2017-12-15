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
extern crate nalgebra as na;
extern crate num;
extern crate typenum;

use std::io;
use std::io::Read;
use std::process::exit;

use na::{Matrix3, Vector3};

use logger::ModuleLogger;

use configurations::Calibrations;

mod hardware;
use hardware::initialize_hardware;

pub type PredictionReading = hardware::PredictionReading;
pub type UpdateReading = hardware::UpdateReading;

use hardware::MotorCommand;

// mod flight;
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


    // for i in 0..200 {
    //     println!("{:?}", pred_rx.recv().unwrap());
    //     println!("{:?}", update_rx.recv().unwrap());

    //     motor_tx.send(MotorCommand::SetPower(0.0, 0.0, 0.0, 0.0));
    // }
}

fn start_flight() {
    let logger = ModuleLogger::new("Main", None);

    let (hardware_join_handle, pred_rx, update_rx, motor_tx, hardware_control_tx) =
        initialize_hardware();


    let calibs = Calibrations::new().unwrap();
    let mag = calibs.magnetometer.unwrap();
    let offsets: Vector3<f32> = mag.get_offsets();
    let rotation: Matrix3<f32> = mag.get_rotation();
    let gains: Vector3<f32> = mag.get_gains();

    for i in 0..10000 {
        let pred = pred_rx.recv().unwrap();
        let update = update_rx.recv().unwrap();

        let mag_reading = update.magnetic_reading;
        if mag_reading.is_some() {
            println!(
                "Un-corrected: x:{:.2},y:{:.2},z:{:.2},m:{:.2}",
                mag_reading.clone().unwrap().x,
                mag_reading.clone().unwrap().y,
                mag_reading.clone().unwrap().z,
                mag_reading.clone().unwrap().norm()
            );

            let offset_corrected: Vector3<f32> = mag_reading.unwrap() - offsets;
            let rotation_corrected: Vector3<f32> = rotation * offset_corrected;
            let gain_corrected: Vector3<f32> = rotation_corrected.component_div(&gains);

            println!(
                "Corrected: x:{:.2},y:{:.2},z:{:.2},m:{:.2}",
                gain_corrected.x,
                gain_corrected.y,
                gain_corrected.z,
                gain_corrected.norm()
            );
        }

        motor_tx.send(MotorCommand::SetPower(0.0, 0.0, 0.0, 0.0));
    }

    logger.log("Press enter to terminate.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    hardware_control_tx.send(()).unwrap();
    motor_tx.send(MotorCommand::PowerDown);

    hardware_join_handle.join().unwrap();
}

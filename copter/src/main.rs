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

use std::io::Read;
use std::io;
use std::thread::sleep;
use std::time::Duration;

use logger::ModuleLogger;

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

    let (hardware_join_handle, pred_rx, update_rx, motor_tx, hardware_control_tx) =
        initialize_hardware();

    for i in 0..200 {
        println!("{:?}", pred_rx.recv().unwrap());
        println!("{:?}", update_rx.recv().unwrap());

        motor_tx.send(MotorCommand::SetPower(0.0, 0.0, 0.0, 0.0));
    }

    let mut input = String::new();

    logger.log("Press enter to terminate.");
    io::stdin().read_line(&mut input).unwrap();
    hardware_control_tx.send(()).unwrap();
    motor_tx.send(MotorCommand::PowerDown);

    hardware_join_handle.join().unwrap();
}

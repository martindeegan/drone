extern crate i2cdev;
extern crate i2cdev_bmp180;
extern crate i2cdev_bmp280;
extern crate i2cdev_l3gd20;
extern crate i2cdev_lsm303d;
extern crate i2cdev_lsm9ds0;
extern crate i2csensors;
extern crate ads111x;
extern crate pca9685;
extern crate rust_pigpio;
extern crate unbounded_gpsd;
extern crate wifilocation;

pub mod gps;
pub mod motors;
pub mod sensors;

use logger::{FlightLogger, ModuleLogger};
use configurations::Config;
use std::sync::mpsc::{channel,Sender,Receiver}
use std::thread;
use std::thread::JoinHandle;


pub fn initialize_hardware() -> (Sender<motors::MotorOutput>, Receiver<sensors::SensorInput>, JoinHandle<()>) {
    let (sensor_tx, sensor_rx): (Sender<sensors::SensorInput>, Receiver<sensors::SensorInput>) = channel();
    let (motor_tx, motor_rx): (Sender<motors::MotorOutput>, Receiver<motors::MotorOutput>) = channel();

    thread::spawn(move || {
        
    });

    (motor_tx, sensor_rx)
}

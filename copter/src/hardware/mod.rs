extern crate i2csensors;
//extern crate i2cdev_bmp180;
extern crate i2cdev_bmp280;
//extern crate i2cdev_l3gd20;
//extern crate i2cdev_lsm303d;
extern crate i2cdev_lsm9ds0;
extern crate i2cdev;
extern crate unbounded_gpsd;
extern crate rust_pigpio;

// pub mod gps;
pub mod motors;
// pub mod sensors;
//
// use config::Config;
// use config::SensorCalibrations;
//
// use std::io::stdin;
// use std::cell::{RefCell, RefMut};
// use std::rc::Rc;
// use std::marker::Sync;
// use std::sync::mpsc::channel;
// use std::sync::mpsc::{Sender, Receiver};
// use std::thread;
// use std::error::Error;
// use std::time::Duration as _Duration;
// use time::{PreciseTime, Duration};
//
// use self::i2csensors::{Barometer, Thermometer, Gyroscope, Accelerometer, Magnetometer, Vec3};
// use self::i2cdev::linux::LinuxI2CError;
// mod sensors;
// mod gps;
// use self::sensors::*;
// use self::gps::GPS;
//
// const LOCATION_SAMPLING_RATE_MS: f32 = 50.0;
//
// const RADIAN_DEGREES: f32 = 180.0 / 3.14159265;
//
// pub type MultiSensorData = Vec3;
//
// #[derive(Copy,Clone)]
// pub struct InertialMeasurement {
//     pub angles: MultiSensorData,
//     pub rotation_rate: MultiSensorData,
//     pub altitude: f32
// }
//
// #[derive(Copy,Clone)]
// pub struct Location {
//     pub latitude: f32,
//     pub longitude: f32,
//     pub altitude: f32
// }
//
//
//
//
// pub fn get_gps() {
//
// }

extern crate i2csensors;
extern crate i2cdev_bmp180;
extern crate i2cdev_bmp280;
extern crate i2cdev_l3gd20;
extern crate i2cdev_lsm303d;
extern crate i2cdev_lsm9ds0;

use config::Config;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

use i2csensors::{Barometer, Thermometer, Gyroscope, Accelerometer, Magnetometer};
use i2cdev_bmp180::*;
use i2cdev_bmp280::*;
use i2cdev_l3gd20::*;
use i2cdev_lsm303d::*;
use i2cdev_lsm9ds0::*;

pub struct SensorOutput {

}

pub fn get_sensor_manager() {
    let config = Config::new();


    for sensor in config.sensors {
        match sensor {
            "BMP180" => {

            },
            "BMP280" => {

            },
            "L3GD20" => {

            },
            "LSM303D" => {

            },
            "LSM9DS0" => {

            },
            _ => {
                return Err("Undefined sensor.");
            }
        }
    }
}

pub struct SensorManager<'a> {
    barometer: Rc<RefCell<Barometer>>,
    thermometer: Rc<RefCell<Thermometer>>,
    gyroscope: Rc<RefCell<Gyroscope>>,
    accelerometer: Rc<RefCell<Accelerometer>>,
    magnetometer: Rc<RefCell<Magnetometer>>
}

impl SensorManager{
    pub fn new(barometer: Rc<RefCell<Barometer>>,
               thermometer: Rc<RefCell<Thermometer>>,
               gyroscope: Rc<RefCell<Gyroscope>>,
               accelerometer: Rc<RefCell<Accelerometer>>,
               magnetometer: Rc<RefCell<Magnetometer>>) -> SensorManager {

        Self {
            barometer: barometer,
            thermometer: thermometer,
            gyroscope: gyroscope,
            accelerometer: accelerometer,
            magnetometer: magnetometer
        }
    }

    pub fn start(&mut self) -> Receiver{
        let (tx, rx): (Sender<SensorOutput>, Receiver<SensorOutput>) = channel();

    }
}


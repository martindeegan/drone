use i2cdev_bmp180::*;
use i2cdev_bmp280::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use i2cdev::core::I2CDevice;
use i2csensors::{Barometer, Thermometer};

use configurations::{Calibrations, Config};

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct BarometerThermometer {
    barometer: Rc<RefCell<Barometer<Error = LinuxI2CError>>>,
    thermometer: Rc<RefCell<Thermometer<Error = LinuxI2CError>>>,
}

impl BarometerThermometer {
    pub fn new() -> BarometerThermometer {
        let mut config = Config::new();

        let mut barometer: Option<Rc<RefCell<Barometer<Error = LinuxI2CError>>>> = None;
        let mut thermometer: Option<Rc<RefCell<Thermometer<Error = LinuxI2CError>>>> = None;




        Barometer {}
    }
}

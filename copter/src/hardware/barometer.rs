use i2cdev_bmp180::*;
use i2cdev_bmp280::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use i2csensors::Barometer;

use configurations::Config;
use logger::ModuleLogger;

use std::rc::Rc;
use std::cell::RefCell;
use std::thread::sleep;
use std::time::Duration;


pub struct BarometerThermometer {
    barometer: Rc<RefCell<Barometer<Error = LinuxI2CError>>>,
    logger: ModuleLogger,
}

impl BarometerThermometer {
    pub fn new() -> Result<BarometerThermometer, ()> {
        let config = Config::new().unwrap();
        let logger = ModuleLogger::new("Barometer", None);

        let mut barometer: Option<Rc<RefCell<Barometer<Error = LinuxI2CError>>>> = None;

        match config.hardware.barometer.name.as_ref() {
            "BMP180" => {
                logger.error("BMP180 not implemented yet.");
                return Err(());
            }
            "BMP280" => {
                logger.log("Initializing BMP280 barometer.");
                match get_bmp280() {
                    Ok(bmp280) => {
                        barometer = Some(Rc::new(RefCell::new(bmp280)));
                    }
                    Err(_) => {
                        logger.error("Couldn't initialize BMP280. Check your hardware connection and you configuration file.");
                        return Err(());
                    }
                }
            }
            _ => {
                logger.error("Unknown barometer model. Check your configuration file.");
                return Err(());
            }
        };

        let manager = BarometerThermometer {
            barometer: barometer.unwrap(),
            logger: logger,
        };

        sleep(Duration::from_millis(50));
        match manager.barometer.borrow_mut().pressure_kpa() {
            Ok(_) => {
                &manager.logger.log("Barometer check.");
            }
            Err(_) => {
                &manager.logger.error("Barometer failed to read.");
            }
        }

        Ok(manager)
    }
}

fn get_bmp180() -> Option<BMP180BarometerThermometer<LinuxI2CDevice>> {
    // Left for someone who owns a bmp180
    None
}

fn get_bmp280() -> Result<BMP280<LinuxI2CDevice>, ()> {
    let settings = BMP280Settings {
        compensation: BMP280CompensationAlgorithm::B64,
        t_sb: BMP280Timing::ms0_5,
        iir_filter_coeff: BMP280FilterCoefficient::Off,
        osrs_t: BMP280TemperatureOversampling::x1,
        osrs_p: BMP280PressureOversampling::StandardResolution,
        power_mode: BMP280PowerMode::NormalMode,
    };

    let baro = get_linux_bmp280_i2c_device().unwrap();
    match BMP280::new(baro, settings) {
        Ok(bmp280) => Ok(bmp280),
        Err(_) => Err(()),
    }
}
